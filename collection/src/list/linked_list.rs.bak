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
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            self.length -= 1;
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().val
        })
    }

    fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            self.length -= 1;
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().val
        })
    }

    fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.val))
    }

    fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.val))
    }

    fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.val))
    }

    fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.val))
    }

    fn get_by_idx(&self, idx: isize) -> Option<Ref<T>> {
        match idx {
            _ if idx < 0 || idx > self.length => None,
            _ if idx == 0 => self.peek_front(),
            _ => {
                if let Some(ref first_node_ref) = self.head.clone() {
                    let mut cur_idx = 0;
                    let mut cur_node = first_node_ref.clone();

                    while let Some(ref mut  next_node_ref) = cur_node.clone().borrow().next {
                        cur_node = next_node_ref.clone();
                        cur_idx += 1;
                        if cur_idx == idx {
                            break;
                        }
                    }
                    cur_node
                        .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.val))
                } else {
                    None
                }
            }
        }
    }

    fn get_by_idx_mut(&self, idx: isize) -> Option<RefMut<T>> {
        todo!()
    }

    fn insert_by_idx(&mut self, _idx: isize, _data: T) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn remove_by_idx(&mut self, _idx: isize) -> Result<T, Box<dyn Error>> {
        todo!()
    }

    fn traverse(&self) {
        if let Some(ref first_node_ref) = self.head {
            let mut cur_node = first_node_ref.clone();
            print!(
                "[val: {}, strong_ref: {}] => ",
                cur_node.borrow().val,
                Rc::strong_count(&cur_node)
            );

            while let Some(ref next_node_ref) = cur_node.clone().borrow().next {
                cur_node = next_node_ref.clone();
                print!(
                    "[{}:{}] => ",
                    cur_node.borrow().val,
                    Rc::strong_count(&cur_node)
                );
            }
        }
        println!();
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
    fn test_pop() {
        let mut list: LinkedList<String> = LinkedList::new();

        list.push_front(String::from("abc"));
        list.push_back(String::from("def"));
        list.push_back(String::from("ghi"));
        list.traverse();
        assert_eq!(list.length, 3);

        let front = list.pop_front();
        println!("pop front: {:?}", front.as_ref().unwrap());
        list.traverse();
        assert_eq!("abc", front.unwrap().as_str());
        assert_eq!(list.length, 2);

        let back = list.pop_back();
        println!("pop back: {:?}", back.as_ref().unwrap());
        list.traverse();
        assert_eq!("ghi", back.unwrap().as_str());
        assert_eq!(list.length, 1);

        let back = list.pop_back();
        println!("pop back: {:?}", back.as_ref().unwrap());
        list.traverse();
        assert_eq!("def", back.unwrap().as_str());
        assert_eq!(list.length, 0);

        println!("list len: {:?}", list.length);
        list.traverse();
    }

    #[test]
    fn test_peek() {}

    #[test]
    fn test_peek_mut() {}
}
