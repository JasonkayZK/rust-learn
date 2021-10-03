use std::borrow::Borrow;
use std::cell::RefCell;
use std::error::Error;
use std::rc::{Rc, Weak};
use crate::node::Node;
use crate::with::With;

#[derive(Debug)]
pub struct MyLinkedList<T> {
    head: Option<Node<T>>,
    tail: Option<Node<T>>,
    len: usize,
}

impl<T> Default for MyLinkedList<T> {
    fn default() -> Self {
        MyLinkedList { head: None, tail: None, len: 0 }
    }
}

impl<T> With<Node<T>> for MyLinkedList<T> {
    fn with(x: Node<T>) -> Self {
        MyLinkedList {
            head: Some(x),
            len: 1,
            ..Default::default()
        }
    }
}

impl<T> MyLinkedList<T> {
    pub fn new() -> Self {
        MyLinkedList {
            ..Default::default()
        }
    }

    pub fn insert(&mut self, x: T, index: usize) -> Result<bool, dyn Error> { Ok(true) }

    pub fn delete(&mut self, index: usize) -> Result<bool, dyn Error> {
        Ok(true)
    }

    pub fn replace(&mut self, index: usize) -> Result<bool, dyn Error> { Ok(true) }

    pub fn get_elem(&self, index: usize) -> Result<Option<Node<T>>, dyn Error> { Ok(None) }

    pub fn prepend(&mut self, data: T) -> Result<bool, dyn Error> { Ok(true) }

    pub fn clear(&mut self) -> Result<bool, dyn Error> { Ok(true) }

    pub fn traverse(&self) -> Result<bool, dyn Error> { Ok(true) }

    pub fn stringify(&self) -> Result<String, dyn Error> { Ok(String::from("")) }

    pub fn head(&self) -> &Option<Node<T>> {
        &self.head
    }
    pub fn tail(&self) -> &Option<Node<T>> {
        &self.tail
    }
    pub fn len(&self) -> &usize { &self.len }

    pub fn set_head(&mut self, head: Option<Node<T>>) {
        self.head = head;
    }
    pub fn set_tail(&mut self, tail: Option<Node<T>>) {
        self.tail = tail;
    }
}

#[cfg(test)]
mod tests {
    use crate::with::With;
    use crate::MyLinkedList;
    use crate::node::Node;

    #[test]
    fn init_list_test() {
        let node1 = Node::with(String::from("hello"));
        let list1 = MyLinkedList::with(node1);
        println!("init list1: {:?}", list1);
        assert_eq!(list1.len(), 1);

        let mut list2: MyLinkedList<String> = MyLinkedList::new();
        println!("init list2: {:?}", list2);
        assert_eq!(list2.len(), 0);
    }
}
