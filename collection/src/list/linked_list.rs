use crate::list::error::IndexOutOfRangeError;
use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ptr::NonNull;

struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Node<T> {
        Node {
            val,
            prev: None,
            next: None,
        }
    }

    fn into_val(self: Box<Self>) -> T {
        self.val
    }
}

pub struct LinkedList<T> {
    length: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    _marker: PhantomData<Box<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
            _marker: PhantomData,
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }

    /// Adds the given node to the front of the list.
    pub fn push_front(&mut self, val: T) {
        // Use box to help generate raw ptr
        let mut node = Box::new(Node::new(val));
        node.next = self.head;
        node.prev = None;
        let node = NonNull::new(Box::into_raw(node));

        match self.head {
            None => self.tail = node,
            Some(head) => unsafe { (*head.as_ptr()).prev = node },
        }

        self.head = node;
        self.length += 1;
    }

    /// Adds the given node to the back of the list.
    pub fn push_back(&mut self, val: T) {
        // Use box to help generate raw ptr
        let mut node = Box::new(Node::new(val));
        node.next = None;
        node.prev = self.tail;
        let node = NonNull::new(Box::into_raw(node));

        match self.tail {
            None => self.head = node,
            // Not creating new mutable (unique!) references overlapping `element`.
            Some(tail) => unsafe { (*tail.as_ptr()).next = node },
        }

        self.tail = node;
        self.length += 1;
    }

    /// Removes the first element and returns it, or `None` if the list is
    /// empty.
    ///
    /// This operation should compute in *O*(1) time.
    pub fn pop_front(&mut self) -> Option<T> {
        self.head.map(|node| {
            self.length -= 1;

            unsafe {
                let node = Box::from_raw(node.as_ptr());

                self.head = node.next;

                match self.head {
                    None => self.tail = None,
                    Some(head) => (*head.as_ptr()).prev = None,
                }
                node.into_val()
            }
        })
    }

    /// Removes the last element from a list and returns it, or `None` if
    /// it is empty.
    ///
    /// This operation should compute in *O*(1) time.
    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.map(|node| {
            self.length -= 1;

            unsafe {
                let node = Box::from_raw(node.as_ptr());

                self.tail = node.prev;

                match self.tail {
                    None => self.head = None,
                    Some(tail) => (*tail.as_ptr()).next = None,
                }
                node.into_val()
            }
        })
    }

    /// Provides a reference to the front element, or `None` if the list is
    /// empty.
    ///
    /// This operation should compute in *O*(1) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::list::linked_list::LinkedList;
    ///
    /// let mut dl = LinkedList::new();
    /// assert_eq!(dl.front(), None);
    ///
    /// dl.push_front(1);
    /// assert_eq!(dl.front(), Some(&1));
    /// ```
    pub fn peek_front(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.as_ref().val) }
    }

    /// Provides a reference to the back element, or `None` if the list is
    /// empty.
    ///
    /// This operation should compute in *O*(1) time.
    pub fn peek_back(&self) -> Option<&T> {
        unsafe { self.tail.as_ref().map(|node| &node.as_ref().val) }
    }

    /// Provides a mutable reference to the front element, or `None` if the list
    /// is empty.
    ///
    /// This operation should compute in *O*(1) time.
    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.as_mut().val) }
    }

    /// Provides a mutable reference to the back element, or `None` if the list
    /// is empty.
    ///
    /// This operation should compute in *O*(1) time.
    pub fn peek_back_mut(&mut self) -> Option<&mut T> {
        unsafe { self.tail.as_mut().map(|node| &mut node.as_mut().val) }
    }

    pub fn get_by_idx(&self, idx: usize) -> Result<Option<&T>, Box<dyn Error>> {
        let len = self.length;

        if idx >= len {
            return Err(Box::new(IndexOutOfRangeError {}));
        }

        // Iterate towards the node at the given index, either from the start or the end,
        // depending on which would be faster.
        let offset_from_end = len - idx - 1;
        let mut cur;
        if idx <= offset_from_end {
            // Head to Tail
            cur = self.head;
            for _ in 0..idx {
                match cur.take() {
                    None => {
                        cur = self.head;
                    }
                    Some(current) => unsafe {
                        cur = current.as_ref().prev;
                    },
                }
            }
        } else {
            // Tail to Head
            cur = self.tail;
            for _ in 0..offset_from_end {
                match cur.take() {
                    None => {
                        cur = self.tail;
                    }
                    Some(current) => unsafe {
                        cur = current.as_ref().prev;
                    },
                }
            }
        }

        unsafe { Ok(cur.as_ref().map(|node| &node.as_ref().val)) }
    }

    pub fn get_by_idx_mut(&self, idx: usize) -> Result<Option<&mut T>, Box<dyn Error>> {
        let len = self.length;

        if idx >= len {
            return Err(Box::new(IndexOutOfRangeError {}));
        }

        // Iterate towards the node at the given index, either from the start or the end,
        // depending on which would be faster.
        let offset_from_end = len - idx - 1;
        let mut cur;
        if idx <= offset_from_end {
            // Head to Tail
            cur = self.head;
            for _ in 0..idx {
                match cur.take() {
                    None => {
                        cur = self.head;
                    }
                    Some(current) => unsafe {
                        cur = current.as_ref().prev;
                    },
                }
            }
        } else {
            // Tail to Head
            cur = self.tail;
            for _ in 0..offset_from_end {
                match cur.take() {
                    None => {
                        cur = self.tail;
                    }
                    Some(current) => unsafe {
                        cur = current.as_ref().prev;
                    },
                }
            }
        }

        unsafe { Ok(cur.as_mut().map(|node| &mut node.as_mut().val)) }
    }

    pub fn insert_by_idx(&mut self, idx: usize, data: T) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    /// Removes the element at the given index and returns it.
    ///
    /// This operation should compute in *O*(*n*) time.
    pub fn remove_by_idx(&mut self, idx: usize) -> Result<T, Box<dyn Error>> {
        let len = self.length;

        if idx >= len {
            return Err(Box::new(IndexOutOfRangeError {}));
        }

        // Below, we iterate towards the node at the given index, either from
        // the start or the end, depending on which would be faster.
        let offset_from_end = len - at - 1;
        if at <= offset_from_end {
            let mut cursor = self.cursor_front_mut();
            for _ in 0..at {
                cursor.move_next();
            }
            cursor.remove_current().unwrap()
        } else {
            let mut cursor = self.cursor_back_mut();
            for _ in 0..offset_from_end {
                cursor.move_prev();
            }
            cursor.remove_current().unwrap()
        }
    }

    /// Returns `true` if the `LinkedList` contains an element equal to the given value.
    ///
    /// This operation should compute in *O*(*n*) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::list::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    ///
    /// list.push_back(0);
    /// list.push_back(1);
    /// list.push_back(2);
    ///
    /// assert_eq!(list.contains(&0), true);
    /// assert_eq!(list.contains(&10), false);
    /// ```
    pub fn contains(&self, elem: &T) -> bool
    where
        T: PartialEq<T>,
    {
        self.iter().any(|x| x == elem)
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            tail: self.tail,
            len: self.length,
            _marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            head: self.head,
            tail: self.tail,
            len: self.length,
            _marker: PhantomData,
        }
    }
}

impl<T: Debug> LinkedList<T> {
    fn traverse(&self) {
        print!("{{ ");
        for (idx, x) in self.iter().enumerate() {
            print!(" [{}: {:?}] ", idx, *x)
        }
        println!(" }}");
    }
}

pub struct IntoIter<T> {
    list: LinkedList<T>,
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        // only need to ensure all our elements are read;
        // buffer will clean itself up afterwards.
        for _ in &mut *self {}

        println!("IntoIter has been dropped!")
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.length, Some(self.list.length))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

pub struct Iter<'a, T: 'a> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    _marker: PhantomData<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| {
                self.len -= 1;

                unsafe {
                    let node = &*node.as_ptr();
                    self.head = node.next;
                    &node.val
                }
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    #[inline]
    fn last(mut self) -> Option<&'a T> {
        self.next_back()
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.tail.map(|node| {
                self.len -= 1;

                unsafe {
                    // Need an unbound lifetime to get 'a
                    let node = &*node.as_ptr();
                    self.tail = node.prev;
                    &node.val
                }
            })
        }
    }
}

pub struct IterMut<'a, T: 'a> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    _marker: PhantomData<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.head.map(|node| {
                self.len -= 1;

                unsafe {
                    let node = &mut *node.as_ptr();
                    self.head = node.next;
                    &mut node.val
                }
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }

    #[inline]
    fn last(mut self) -> Option<&'a mut T> {
        self.next_back()
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else {
            self.tail.map(|node| {
                self.len -= 1;

                unsafe {
                    // Need an unbound lifetime to get 'a
                    let node = &mut *node.as_ptr();
                    self.tail = node.prev;
                    &mut node.val
                }
            })
        }
    }
}

#[cfg(test)]
mod test {
    use crate::list::linked_list::LinkedList;

    #[test]
    fn test_compiling() {}

    #[test]
    fn test_push() {
        let mut list = _new_list_i32();

        assert_eq!(list.length, 5);

        list.traverse();
    }

    struct ZeroSizeType {}

    fn _new_list_i32() -> LinkedList<i32> {
        let mut list = LinkedList::new();

        list.push_front(456);
        list.push_front(123);
        list.push_back(789);

        list
    }

    fn _new_list_string() -> LinkedList<String> {
        let mut list = LinkedList::new();

        list.push_front(String::from("def"));
        list.push_front(String::from("abc"));
        list.push_back(String::from("hij"));

        list
    }

    fn _new_list_zst() -> LinkedList<ZeroSizeType> {
        let mut list = LinkedList::new();

        list.push_front(ZeroSizeType {});
        list.push_front(ZeroSizeType {});
        list.push_back(ZeroSizeType {});

        list
    }
}
