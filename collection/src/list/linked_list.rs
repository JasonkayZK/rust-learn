use crate::list::error::IndexOutOfRangeError;
use std::error::Error;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::mem;
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
                        cur = current.as_ref().next;
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
        let mut cur = self._get_by_idx_mut(idx)?;
        unsafe { Ok(cur.as_mut().map(|node| &mut node.as_mut().val)) }
    }

    pub fn insert_by_idx(&mut self, idx: usize, data: T) -> Result<(), Box<dyn Error>> {
        let len = self.length;

        if idx > len {
            return Err(Box::new(IndexOutOfRangeError {}));
        }

        if idx == 0 {
            return Ok(self.push_front(data));
        } else if idx == len {
            return Ok(self.push_back(data));
        }

        unsafe {
            // Create Node
            let mut spliced_node = Box::new(Node::new(data));
            let before_node = self._get_by_idx_mut(idx - 1)?;
            let after_node = before_node.unwrap().as_mut().next;
            spliced_node.prev = before_node;
            spliced_node.next = after_node;
            let spliced_node = NonNull::new(Box::into_raw(spliced_node));

            // Insert Node
            before_node.unwrap().as_mut().next = spliced_node;
            after_node.unwrap().as_mut().prev = spliced_node;
        }

        self.length += 1;

        Ok(())
    }

    /// Removes the element at the given index and returns it.
    ///
    /// This operation should compute in *O*(*n*) time.
    pub fn remove_by_idx(&mut self, idx: usize) -> Result<T, Box<dyn Error>> {
        let len = self.length;

        if idx >= len {
            return Err(Box::new(IndexOutOfRangeError {}));
        }

        if idx == 0 {
            return Ok(self.pop_front().unwrap());
        } else if idx == len - 1 {
            return Ok(self.pop_back().unwrap());
        };

        let cur = self._get_by_idx_mut(idx)?.unwrap();

        self.unlink_node(cur);

        unsafe {
            let unlinked_node = Box::from_raw(cur.as_ptr());
            Ok(unlinked_node.val)
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

    fn _get_by_idx_mut(&self, idx: usize) -> Result<Option<NonNull<Node<T>>>, Box<dyn Error>> {
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
                        cur = current.as_ref().next;
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

        Ok(cur)
    }

    /// Unlinks the specified node from the current list.
    ///
    /// Warning: this will not check that the provided node belongs to the current list.
    ///
    /// This method takes care not to create mutable references to `element`,
    /// to maintain validity of aliasing pointers.
    #[inline]
    fn unlink_node(&mut self, mut node: NonNull<Node<T>>) {
        let node = unsafe { node.as_mut() }; // this one is ours now, we can create an &mut.

        // Not creating new mutable (unique!) references overlapping `element`.
        match node.prev {
            Some(prev) => unsafe { (*prev.as_ptr()).next = node.next },
            // this node is the head node
            None => self.head = node.next,
        };

        match node.next {
            Some(next) => unsafe { (*next.as_ptr()).prev = node.prev },
            // this node is the tail node
            None => self.tail = node.prev,
        };

        self.length -= 1;
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

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        struct DropGuard<'a, T>(&'a mut LinkedList<T>);

        impl<'a, T> Drop for DropGuard<'a, T> {
            fn drop(&mut self) {
                // Continue the same loop we do below. This only runs when a destructor has
                // panicked. If another one panics this will abort.
                while self.0.pop_front().is_some() {}
            }
        }

        while let Some(node) = self.pop_front() {
            let guard = DropGuard(self);
            drop(node);
            mem::forget(guard);
        }

        println!("LinkedList dropped!")
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
    fn test_push_and_pop() {
        let mut list = _new_list_i32();

        assert_eq!(list.length, 5);
        list.traverse();

        assert_eq!(list.pop_front(), Some(-1));
        assert_eq!(list.pop_back(), Some(i32::MAX));

        assert_eq!(list.length, 3);
        list.traverse();
    }

    #[test]
    fn test_peak() {
        let mut list = _new_list_string();

        assert_eq!(list.peek_front(), Some(&String::from("abc")));
        assert_eq!(list.peek_back(), Some(&String::from("hij")));

        let cur = list.peek_front_mut();
        assert_eq!(cur, Some(&mut String::from("abc")));
        cur.map(|x| x.push(' '));

        let cur = list.peek_back_mut();
        assert_eq!(cur, Some(&mut String::from("hij")));
        cur.map(|x| x.push(' '));

        assert_eq!(list.peek_front(), Some(&String::from("abc ")));
        assert_eq!(list.peek_back(), Some(&String::from("hij ")));
        assert_eq!(list.length, 3);

        list.traverse();
    }

    #[test]
    fn test_get_idx() {
        let list = _new_list_i32();

        assert_eq!(list.get_by_idx(2).unwrap(), Some(&456));
        assert_eq!(list.get_by_idx(3).unwrap(), Some(&789));

        print!("before change: ");
        list.traverse();
        let cur = list.get_by_idx_mut(2).unwrap().unwrap();
        assert_eq!(cur, &mut 456);

        *cur <<= 1;
        print!("after change: ");
        list.traverse();

        assert_eq!(list.get_by_idx(2).unwrap(), Some(&(456 << 1)));
    }

    #[test]
    fn test_get_idx_err() {
        let list = _new_list_i32();

        assert!(list.get_by_idx(99).is_err());
        assert!(list.get_by_idx_mut(99).is_err());
    }

    #[test]
    fn test_insert_idx() {
        let mut list = LinkedList::new();

        list.push_back(String::from("1"));
        list.push_back(String::from("2"));
        list.push_back(String::from("3"));

        list.insert_by_idx(1, String::from("99")).unwrap();
        list.traverse();

        assert_eq!(list.get_by_idx(0).unwrap(), Some(&String::from("1")));
        assert_eq!(list.get_by_idx(1).unwrap(), Some(&String::from("99")));
    }

    #[test]
    fn test_insert_idx_err() {
        let mut list = LinkedList::new();

        assert!(list.insert_by_idx(99, String::from("99")).is_err());
    }

    #[test]
    fn test_remove_idx() {
        let mut list = LinkedList::new();

        list.push_back(String::from("1"));
        list.push_back(String::from("2"));
        list.push_back(String::from("3"));

        let removed = list.remove_by_idx(1).unwrap();
        list.traverse();

        assert_eq!(removed, String::from("2"));

        assert_eq!(list.get_by_idx(0).unwrap(), Some(&String::from("1")));
        assert_eq!(list.get_by_idx(1).unwrap(), Some(&String::from("3")));
    }

    #[test]
    fn test_remove_idx_err() {
        let mut list: LinkedList<i32> = LinkedList::new();

        assert!(list.remove_by_idx(99).is_err());
    }

    #[test]
    fn test_contains() {
        let list = _new_list_i32();

        assert!(list.contains(&-1));
        assert!(!list.contains(&-2));
    }

    #[test]
    fn test_clear() {
        let mut list = _new_list_zst();

        assert_eq!(list.length(), 3);

        list.clear();

        assert_eq!(list.length(), 0);
    }

    #[test]
    fn test_iterator() {
        let mut list1 = _new_list_i32();

        print!("before change: ");
        list1.traverse();
        list1.iter_mut().for_each(|x| *x = *x - 1);
        print!("after change: ");
        list1.traverse();

        let list2 = _new_list_string();
        let list2_to_len = list2.into_iter().map(|x| x.len()).collect::<Vec<usize>>();
        println!(
            "transform list2 into len vec, list2_to_len: {:?}",
            list2_to_len
        );

        // Compiling err:
        // list2.traverse()
    }

    struct ZeroSizeType {}

    fn _new_list_i32() -> LinkedList<i32> {
        let mut list = LinkedList::new();

        list.push_front(456);
        list.push_front(123);
        list.push_back(789);
        list.push_front(-1);
        list.push_back(i32::MAX);

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
