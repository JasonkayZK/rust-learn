use std::ptr;
use std::ptr::NonNull;

/// A Unsafe Double LinkedList
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
    len: i32,
}

impl<T> DoubleLinkedList<T> {
    pub fn new() -> Self {
        DoubleLinkedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn len(&self) -> i32 {
        self.len
    }

    pub fn push_front(&mut self, val: T) {
        self.len += 1;

        let new_node = Node::new(val);
        if self.head.is_null() {

        }


    }

    pub fn push_back(&mut self, val: T) {

    }

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
        unsafe {
            let mut cur_node = self.head;
            while !cur_node.is_null() {
                let next = (*cur_node).next;
                (*cur_node).prev = ptr::null_mut(); // clear prev ptr
                (*cur_node).next = ptr::null_mut(); // clear next ptr
                cur_node = next;
            }
        }
        println!("list has been dropped!");
    }
}

pub struct IntoIter<T> {
    list: DoubleLinkedList<T>,
}


pub struct Iter<'a, T: 'a> {
    head: Link<T>,
    tail: Link<T>,
    len: i32,
}


pub struct IterMut<'a, T: 'a> {
    head: Link<T>,
    tail: Link<T>,
    len: i32,
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
