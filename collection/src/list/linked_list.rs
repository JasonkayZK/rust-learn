use crate::list::list::List;
use crate::list::node::{LinkNode, Node};
use std::cell::{Ref, RefMut};
use std::error::Error;
use std::fmt::Display;
use std::rc::Rc;

pub struct LinkedList<T> {
    head: LinkNode<T>,
    tail: LinkNode<T>,
    length: isize,
}

impl<T: Display> List<T> for LinkedList<T> {
    fn new() -> Self {
        LinkedList {
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
                old_head.borrow_mut().prev = Some(Rc::clone(&new_head));
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(Rc::clone(&new_head));
                self.head = Some(new_head);
            }
        }
        self.length += 1;
    }

    fn push_back(&mut self, data: T) {
        let new_tail = Node::new_core_node(data);

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(Rc::clone(&new_tail));
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(Rc::clone(&new_tail));
                self.tail = Some(new_tail);
            }
        }
        self.length += 1;
    }

    fn pop_front(&mut self) -> Option<T> {
        if self.length <= 0 {
            // Head is none
            None
        }

        let old_head = self.head.take();
        match old_head.borrow_mut().next.take() {
            None => {
                self.tail.take(); // If head is None, remove tail
            }
            Some(new_head) => {
                new_head.borrow_mut().prev.take();
                self.head = Some(new_head);
            }
        }
        self.length -= 1;
        Rc::try_unwrap(old_head).ok().unwrap().into_inner().val
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

    fn get_idx(&self) -> Option<Ref<T>> {
        todo!()
    }

    fn get_idx_mut(&self) -> Option<RefMut<T>> {
        todo!()
    }

    fn insert(&mut self, idx: isize, data: T) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn remove(&mut self, idx: isize) -> Result<T, Box<dyn Error>> {
        todo!()
    }

    fn traverse(&self) {
        print!("[");
        if let Some(ref first_node_ref) = self.head {
            let mut cur_node = first_node_ref.clone();
            print!("{}:{} ", cur_node.borrow().val, Rc::strong_count(&cur_node));

            while let Some(ref next_node_ref) = cur_node.clone().borrow().next {
                cur_node = next_node_ref.clone();
                print!("{}:{} ", cur_node.borrow().val, Rc::strong_count(&cur_node));
            }
        }
        println!("]");
    }
}

mod test {
    use crate::list::linked_list::LinkedList;
    use crate::list::list::List;

    #[test]
    fn test_new() {
        let list: LinkedList<String> = LinkedList::new();
        list.traverse();
    }

    #[test]
    fn test_get_len() {
        let list: LinkedList<String> = LinkedList::new();
        list.traverse();
        println!("new list len: {:?}", list.length);
    }

    #[test]
    fn test_push() {
        let mut list: LinkedList<String> = LinkedList::new();

        list.push_front(String::from("abc"));
        list.push_back(String::from("def"));
        list.traverse();

        list.push_back(String::from("ghi"));
        list.push_front(String::from("789"));
        list.push_front(String::from("456"));

        assert_eq!(list.length, 5);

        println!("list len: {:?}", list.length);
        list.traverse();
    }

    #[test]
    fn test_pop() {}

    #[test]
    fn test_peak() {}

    #[test]
    fn test_peak_mut() {}
}
