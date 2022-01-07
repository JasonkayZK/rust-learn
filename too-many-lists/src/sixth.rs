//! A Unsafe Double LinkedList
use std::marker::PhantomData;
use std::ptr;
use std::ptr::NonNull;

struct Node<T> {
    val: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        Node {
            val,
            next: None,
            prev: None,
        }
    }
}

type Link<T> = Option<NonNull<Node<T>>>;

pub struct DoubleLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    len: usize,
    marker: PhantomData<Box<Node<T>>>,
}

/// An iterator over the elements of a `DoubleLinkedList`.
pub struct Iter<'a, T: 'a> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

pub struct IterMut<'a, T: 'a> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a mut Node<T>>,
}

impl<T> DoubleLinkedList<T> {
    pub fn new() -> Self {
        DoubleLinkedList {
            head: None,
            tail: None,
            len: 0,
            marker: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_front(&mut self, val: T) {
        self.len += 1;

        let new_node = Node::new(val);
        if self.head.is_null() {}
    }

    pub fn push_back(&mut self, val: T) {}

    pub fn pop_front(&mut self) -> T {}

    pub fn pop_back(&mut self) -> T {}

    pub fn peek_front(&self) -> &T {}

    pub fn peek_front_mut(&mut self) -> &mut T {}

    pub fn peek_back(&self) -> &T {}

    pub fn peek_back_mut(&mut self) -> &mut T {}

    pub fn into_iter(self) -> IntoIter<T> {}

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: self.head,
            tail: self.tail,
            len: self.len,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            head: self.head,
            tail: self.tail,
            len: self.len,
        }
    }
}

impl<T> Drop for DoubleLinkedList<T> {
    fn drop(&mut self) {
        println!("list has been dropped!");
    }
}

#[cfg(test)]
mod test {
    use crate::sixth::DoubleLinkedList;

    #[test]
    fn test_all() {
        let list: DoubleLinkedList<i32> = DoubleLinkedList::new();
        println!("len of new list: {:?}", list.len);
    }
}
