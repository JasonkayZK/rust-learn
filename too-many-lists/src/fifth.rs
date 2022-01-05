use std::{ptr};

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: ptr::null_mut() }
    }

    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            elem,
            next: None,
        });
        let raw_tail: *mut Node<T> = &mut *new_tail; // Change to raw pointer

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }
        self.tail = raw_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }
            head.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(| node| {
            &mut node.elem
        })
    }

    pub fn peek_back(&self) -> Option<&T> {
        if self.tail.is_null() {
            return None;
        }

        let ret_elem;
        unsafe {
            ret_elem = &(*self.tail).elem;
        }
        Some(&ret_elem)
    }

    pub fn peek_back_mut(&mut self) -> Option<&mut T> {
        if self.tail.is_null() {
            return None;
        }

        let ret_elem;
        unsafe {
            ret_elem = &mut (*self.tail).elem;
        }
        Some(ret_elem)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            list: self,
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref()
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut{
            next: self.head.as_deref_mut()
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_node = self.head.take();
        while let Some(mut box_node) = cur_node {
            cur_node = box_node.next.take();
        };

        println!("list has been dropped!");
    }
}

pub struct IntoIter<T> {
    list: List<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use crate::fifth::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.peek_back(), Some(&3));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.peek_back(), Some(&3));

        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_back(), Some(&3));

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), None);
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_back(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }
}
