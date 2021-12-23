use crate::list::list::List;
use crate::list::node::{LinkNode, Node};
use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use std::vec::IntoIter;

#[derive(Debug)]
pub struct SingleLinkedList<T> {
    head: LinkNode<T>,
    tail: LinkNode<T>,
    length: isize,
}

impl<T> List<T> for SingleLinkedList<T> {
    fn new() -> Self {
        SingleLinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    fn length(&self) -> isize {
        return self.length;
    }

    fn push_front(&mut self, data: T) {
        let new_head = Node::new_core_node(data);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
        self.length += 1;
    }

    fn push_back(&mut self, data: T) {

    }

    fn pop_front(&mut self) -> Option<T> {
        todo!()
    }

    fn pop_back(&mut self) -> Option<T> {
        todo!()
    }

    fn peek_front(&self) -> Option<Ref<T>> {
        todo!()
    }

    fn peek_back(&self) -> Option<Ref<T>> {
        todo!()
    }

    fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        todo!()
    }

    fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        todo!()
    }
}

mod test {
    use crate::list::list::List;
    use crate::list::single_linked_list::SingleLinkedList;

    #[test]
    fn test_new() {
        let list: SingleLinkedList<String> = SingleLinkedList::new();
        println!("new list: {:?}", list)
    }

    #[test]
    fn test_get_len() {
        let list: SingleLinkedList<String> = SingleLinkedList::new();
        println!("new list len: {:?}", list.length)
    }

    #[test]
    fn test_push() {
        let mut list: SingleLinkedList<String> = SingleLinkedList::new();

        list.push_front(String::from("abc"));
        list.push_front(String::from("def"));

        println!("list: {:#?}, len: {}", list, list.length);
    }

    #[test]
    fn test_pop() {

    }
}
