use std::alloc::{self, Layout};
use std::ptr::NonNull;
use std::marker::PhantomData;
use std::{mem, ptr};
use std::ops::{Deref, DerefMut};

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
        // currently not handle ZSTs
        assert_ne!(mem::size_of::<T>(), 0, "We're not ready to handle ZSTs");

        RawVector {
            // No memory allocate here, and avoid not defining behavior
            ptr: NonNull::dangling(),
            cap: 0,
            _marker: PhantomData,
        }
    }

    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 { // no consideration for ZST
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
        assert!(new_layout.size() <= isize::MAX as usize, "Allocation too large");

        let new_ptr = if self.cap == 0 {
            unsafe {
                alloc::alloc(new_layout)
            }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe {
                alloc::realloc(old_ptr, old_layout, new_layout.size())
            }
        };

        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            // Fail to allocate memory will cause new_ptr to None!
            // handle with OOM in multiple platform[kill program](panic will simply make OOM worse!)
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
}

impl<T> Drop for RawVector<T> {
    fn drop(&mut self) {
        // We can't call "alloc::dealloc" when self.cap() == 0
        // Since we are not allocate any memory!
        if self.cap != 0 {
            // If T: !Drop, the line below is unnecessary!
            // We could check T is "need_drop" to avoid call pop
            // But this will be optimized by LLVM automatically!
            // while let Some(_) = self.pop() {}

            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
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
        if self.cap() == self.cap() {
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
            unsafe {
                Some(ptr::read(self.ptr().add(self.len)))
            }
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
            ptr::copy(self.ptr().add(idx),
                      self.ptr().add(idx + 1),
                      self.len - idx);
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
            ptr::copy(self.ptr().add(idx + 1),
                      self.ptr().add(idx),
                      self.len - idx);
            res
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        // Move pointer here
        let buf = self.ptr();
        // Copy cap & len here
        let cap = self.cap();
        let len = self.len;

        // We can't Drop Vector here!
        // Since which will release the memory that has been allocated!
        // [Then the IntoIter below will read a released memory!]
        mem::forget(self);

        unsafe {
            IntoIter {
                buf,
                cap,
                // The IntoIter will end when start == end
                start: buf.as_ptr(),
                end: if cap == 0 {
                    buf.as_ptr()
                } else {
                    buf.as_ptr().add(len)
                },
                _marker: PhantomData,
            }
        }
    }

}

/// <Deref> Implement directly from "std::slice::from_raw_parts"
impl<T> Deref for Vector<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe {
            std::slice::from_raw_parts(self.ptr(), self.len)
        }
    }
}

/// <DerefMut> Implement directly from "std::slice::from_raw_parts_mut"
impl<T> DerefMut for Vector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr(), self.len)
        }
    }
}

pub struct IntoIter<T> {
    buf: NonNull<T>,
    cap: usize,
    // Two pointer for optimizing DoubleEndedIterator
    // The IntoIter will end when start == end
    start: *const T,
    end: *const T,
    _marker: PhantomData<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // IntoIter elems are all consumed when start == end
        if self.start == self.end {
            None
        } else { // Read start for forward iteration
            unsafe {
                let res = ptr::read(self.start);
                self.start = self.start.offset(1);
                Some(res)
            }
        }
    }

    /// Returns (low_bound, Some(high_bound) or None)
    fn size_hint(&self) -> (usize, Option<usize>) {
        // ZST cause problem here!
        let len = (self.end as usize - self.start as usize) / mem::size_of::<T>();
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            unsafe { // Backward iteration
                self.end = self.end.offset(-1);
                Some(ptr::read(self.end))
            }
        }
    }
}

/// Drop the unused elem and memory in IntoIter
impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        self.grow();
        if self.cap() != 0 {
            // Drop each elem
            for _ in &mut *self {}

            // Deallocate the memory
            let layout = Layout::array::<T>(self.cap()).unwrap();
            unsafe {
                alloc::dealloc(self.buf.as_ptr() as *mut u8, layout);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::list::vector::Vector;

    #[test]
    fn compiling_test() {}

    #[test]
    fn new_test() {
        Vector::<isize>::new();
    }
}
