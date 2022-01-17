use std::alloc::{self, Layout};
use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::{mem, ptr};

/// Internal struct of Vector for sharing elem between Vector & IntoIter
struct RawVector<T> {
    ptr: NonNull<T>,
    cap: usize,
    _marker: PhantomData<T>,
}

unsafe impl<T: Send> Send for RawVector<T> {}

unsafe impl<T: Sync> Sync for RawVector<T> {}

impl<T> RawVector<T> {
    pub fn new() -> Self {
        // !0 is usize::MAX. This branch should be stripped at compile time
        let cap = if mem::size_of::<T>() == 0 { !0 } else { 0 };

        RawVector {
            // `NonNull::dangling()` doubles as "unallocated" and "zero-sized allocation"
            // No memory allocate here, and avoid not defining behavior
            cap,
            ptr: NonNull::dangling(),
            _marker: PhantomData,
        }
    }

    fn grow(&mut self) {
        // Since we set the capacity to usize::MAX when T has size 0,
        // getting to here necessarily means the Vec is overfull.
        assert_ne!(mem::size_of::<T>(), 0, "capacity overflow");

        let (new_cap, new_layout) = if self.cap == 0 {
            // no consideration for ZST
            (1, Layout::array::<T>(1).unwrap())
        } else {
            // This can't overflow since self.cap <= isize::MAX.
            let new_cap = self.cap << 1;

            // `Layout::array` checks that the number of bytes is <= usize::MAX,
            // but this is redundant since old_layout.size() <= isize::MAX,
            // so the `unwrap` should never fail.
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        // Ensure that the new allocation doesn't exceed `isize::MAX` bytes.
        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            // Fail to allocate memory will cause new_ptr to None!
            // handle with OOM in multiple platform[kill program](panic will simply make OOM worse!)
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;

        println!("grow capacity success, current cap: {}", self.cap)
    }
}

impl<T> Drop for RawVector<T> {
    fn drop(&mut self) {
        // Handle ZST
        let elem_size = mem::size_of::<T>();

        // We can't call "alloc::dealloc" when self.cap() == 0
        // Since we are not allocate any memory!
        if self.cap != 0 && elem_size != 0 {
            unsafe {
                alloc::dealloc(
                    self.ptr.as_ptr() as *mut u8,
                    Layout::array::<T>(self.cap).unwrap(),
                );
            }
        }

        println!("RawVector has been dropped!")
    }
}

/// A Vector implementation
///  Refer to https://github.com/rust-lang/nomicon/tree/master/src/vec
pub struct Vector<T> {
    buf: RawVector<T>,
    len: usize,
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        Vector {
            buf: RawVector::new(),
            len: 0,
        }
    }

    fn ptr(&self) -> *mut T {
        self.buf.ptr.as_ptr()
    }

    fn cap(&self) -> usize {
        self.buf.cap
    }

    pub fn push(&mut self, elem: T) {
        // Cause capacity grow when no capacity here
        if self.len == self.cap() {
            self.buf.grow();
        }

        // No index here:
        // foo[idx] = elem will call drop on foo[idx] !
        // Use ptr::write to force write elem in foo[idx] without call "drop"!
        unsafe {
            ptr::write(self.ptr().add(self.len), elem);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            // Directly move foo[idx] out of Vector will deallocate this memory (in Vector) forever[which cause problem]!
            // So use prt::read to read raw bit in memory!
            unsafe { Some(ptr::read(self.ptr().add(self.len))) }
        }
    }

    pub fn insert(&mut self, idx: usize, elem: T) {
        assert!(idx <= self.len, "index out of bounds");

        if self.cap() == self.len {
            self.buf.grow();
        }

        unsafe {
            // copy [i..len] to [i+1..len+1]
            // ptr::copy is the same as "memmove" in C!
            ptr::copy(self.ptr().add(idx), self.ptr().add(idx + 1), self.len - idx);
            // write elem binary into arr[idx]
            ptr::write(self.ptr().add(idx), elem);
            self.len += 1;
        }
    }

    pub fn remove(&mut self, idx: usize) -> T {
        assert!(idx < self.len, "index out of bounds");

        unsafe {
            self.len -= 1;
            let res = ptr::read(self.ptr().add(idx));
            // copy [i+1..len+1] to [i..len] to override arr[idx]!ß
            ptr::copy(self.ptr().add(idx + 1), self.ptr().add(idx), self.len - idx);
            res
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        unsafe {
            let iter = RawValIter::new(&self);

            // buf not implement the "Copy" Trait, But Vector implement Drop!
            let buf = ptr::read(&self.buf);

            // We can't Drop Vector here!
            // Since which will release the memory that has been allocated!
            // [Then the IntoIter below will read a released memory!]
            mem::forget(self);

            IntoIter {
                // The IntoIter will end when start == end
                iter,
                _buf: buf,
            }
        }
    }

    pub fn drain(&mut self) -> Drain<T> {
        unsafe {
            let iter = RawValIter::new(&self);

            // this is a mem::forget safety thing. If Drain is forgotten, we just
            // leak the whole Vector's contents. Also we need to do this *eventually*
            // anyway, so why not do it now?
            self.len = 0;

            Drain {
                iter,
                vec: PhantomData,
            }
        }
    }
}

impl<T: Display> Vector<T> {
    pub fn traverse(&self) {
        print!("{{ ");
        for (idx, x) in self.iter().enumerate() {
            print!(" [{}: {}] ", idx, *x)
        }
        println!(" }}");
    }
}

impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        // If T: !Drop, the line below is unnecessary!
        // We could check T is "need_drop" to avoid call pop
        // But this will be optimized by LLVM automatically!
        while let Some(_) = self.pop() {}

        // Use RawVector to drop!
        println!("Vector has been dropped!")
    }
}

/// <Deref> Implement directly from "std::slice::from_raw_parts"
impl<T> Deref for Vector<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.ptr(), self.len) }
    }
}

/// <DerefMut> Implement directly from "std::slice::from_raw_parts_mut"
impl<T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.ptr(), self.len) }
    }
}

/// RawValIter for IntoIter & Drain
struct RawValIter<T> {
    // Two pointer for optimizing DoubleEndedIterator
    // The IntoIter will end when start == end
    start: *const T,
    end: *const T,
}

impl<T> RawValIter<T> {
    // unsafe to construct because it has no associated lifetimes.
    // This is necessary to store a RawValIter in the same struct as
    // its actual allocation. OK since it's a private implementation
    // detail.
    unsafe fn new(slice: &[T]) -> Self {
        RawValIter {
            start: slice.as_ptr(),
            end: if mem::size_of::<T>() == 0 {
                // Handle ZST
                ((slice.as_ptr() as usize) + slice.len()) as *const _
            } else if slice.len() == 0 {
                // if `len = 0`, then this is not actually allocated memory.
                // Need to avoid offsetting because that will give wrong
                // information to LLVM via GEP.
                slice.as_ptr()
            } else {
                slice.as_ptr().add(slice.len())
            },
        }
    }
}

impl<T> Iterator for RawValIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // IntoIter elems are all consumed when start == end
        if self.start == self.end {
            None
        } else {
            // Read start for forward iteration
            unsafe {
                let res = ptr::read(self.start);
                self.start = if mem::size_of::<T>() == 0 {
                    // Handle ZST
                    (self.start as usize + 1) as *const _
                } else {
                    self.start.offset(1)
                };
                Some(res)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let elem_size = mem::size_of::<T>();
        let len =
            (self.end as usize - self.start as usize) / if elem_size == 0 { 1 } else { elem_size };
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for RawValIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                // Backward iteration
                self.end = if mem::size_of::<T>() == 0 {
                    (self.end as usize - 1) as *const _
                } else {
                    self.end.offset(-1)
                };
                Some(ptr::read(self.end))
            }
        }
    }
}

pub struct IntoIter<T> {
    _buf: RawVector<T>, // we don't actually care about this. Just need it to live.
    iter: RawValIter<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    /// Returns (low_bound, Some(high_bound) or None)
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter.next_back()
    }
}

/// Drop the unused elem and memory in IntoIter
impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        // only need to ensure all our elements are read;
        // buffer will clean itself up afterwards.
        for _ in &mut *self {}

        println!("IntoIter has been dropped!")
    }
}

/// Drain Consumes the specific range of the Vector
pub struct Drain<'a, T: 'a> {
    vec: PhantomData<&'a mut Vector<T>>,
    iter: RawValIter<T>,
}

impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T> DoubleEndedIterator for Drain<'a, T> {
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

impl<'a, T> Drop for Drain<'a, T> {
    fn drop(&mut self) {
        for _ in &mut *self {}

        println!("Drain has been dropped!")
    }
}

#[cfg(test)]
mod test {
    use crate::list::vector::Vector;

    #[test]
    fn compiling_test() {}

    #[test]
    fn new_test() {
        let list = Vector::<isize>::new();
        println!("new list len: {:?}", list.len());
    }

    #[test]
    fn test_push() {
        let mut list: Vector<String> = Vector::new();

        list.push(String::from("abc"));
        list.push(String::from("def"));

        list.insert(2, String::from("ghi"));
        list.insert(3, String::from("789"));
        list.insert(4, String::from("456"));

        assert_eq!(list.len(), 5);

        println!("list len: {:?}", list.len());
        list.traverse();
    }

    #[test]
    fn test_pop() {
        let mut list: Vector<String> = Vector::new();

        list.push(String::from("abc"));
        list.push(String::from("def"));
        list.push(String::from("ghi"));
        list.traverse();
        assert_eq!(list.len(), 3);

        let front = list.pop();
        println!("pop front: {:?}", front.as_ref().unwrap());
        list.traverse();
        assert_eq!("ghi", front.as_ref().unwrap());
        assert_eq!(list.len(), 2);

        let back = list.remove(0);
        println!("pop idx: {:?}", back);
        list.traverse();
        assert_eq!("abc", back);
        assert_eq!(list.len(), 1);

        let back = list.pop();
        println!("pop idx: {:?}", back.as_ref().unwrap());
        list.traverse();
        assert_eq!("def", back.as_ref().unwrap());
        assert_eq!(list.len(), 0);

        println!("list len: {:?}", list.len());
        list.traverse();
    }

    #[test]
    fn test_iter() {
        let mut list: Vector<i32> = Vector::new();

        list.push(32);
        list.push(2);
        list.push(344);
        list.push(342);

        list.iter()
            .enumerate()
            .for_each(|x| print!(" {{ {:?} }} ", x));
        println!()
    }

    #[test]
    fn test_iter_mut() {
        let mut list: Vector<i32> = Vector::new();

        list.push(32);
        list.push(2);
        list.push(344);
        list.push(342);

        for x in list.iter_mut() {
            *x <<= 1
        }

        list.traverse()
    }

    #[test]
    fn test_into_iter() {
        let mut list: Vector<i32> = Vector::new();

        list.push(32);
        list.push(2);
        list.push(344);
        list.push(342);

        let new_list = list.into_iter().filter(|x| *x > 300).collect::<Vec<_>>();
        println!("new_list: {:?}", new_list)
    }

    #[test]
    fn test_drain() {
        let mut list: Vector<i32> = Vector::new();

        list.push(32);
        list.push(2);
        list.push(344);
        list.push(342);

        let new_list = list.drain().filter(|x| *x > 300).collect::<Vec<_>>();
        println!("new_list: {:?}", new_list)
    }

    #[test]
    fn test_zst() {
        #[derive(Debug)]
        struct X{}

        let mut list: Vector<X> = Vector::new();

        list.push(X{});
        list.push(X{});
        list.push(X{});
        list.push(X{});

        println!("pop: {:?}", list.pop());
        println!("remove: {:?}", list.remove(0));

        // no-op
        list.into_iter().for_each(|x| print!("{:?}", x));
        println!();
    }
}
