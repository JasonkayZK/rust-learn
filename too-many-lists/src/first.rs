use std::mem;

#[derive(Debug)]
enum EnumList {
    Empty,
    ElemThenEmpty(i32),
    ElemThenNotEmpty(i32, Box<EnumList>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    Elem(Box<Node>),
}

#[derive(Debug)]
pub struct List {
    head: Link,
}

impl List {
    fn new() -> Self {
        List { head: Link::Empty }
    }

    fn push(&mut self, elem: i32) {
        let node = Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        };
        self.head = Link::Elem(Box::new(node));
    }

    fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::Elem(node) => {
                let node = *node;
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::Elem(mut node) = cur_link {
            cur_link = mem::replace(&mut node.next, Link::Empty);
        }

        println!("list has been cleared to empty");
    }
}

#[cfg(test)]
mod test {
    use crate::first::{EnumList, Link, List, Node};

    #[test]
    fn test_1() {
        let a = EnumList::ElemThenEmpty(100);
        println!("{:?}", a);

        let b = EnumList::ElemThenNotEmpty(10, Box::new(a));
        println!("{:?}", b);

        let c = EnumList::Empty;
        println!("{:?}", c);
    }

    #[test]
    fn test_2() {
        let a = Link::Elem(Box::new(Node {
            elem: 100,
            next: Link::Empty,
        }));
        println!("{:?}", a);
        match a {
            Link::Empty => {
                println!("a.elem: Empty")
            }
            Link::Elem(ref node) => {
                println!("a.elem: {:?}", node.elem);
                println!("a.next: {:?}", node.next);
            }
        }

        let b = Link::Elem(Box::new(Node { elem: 10, next: a }));
        println!("b: {:?}", b);
        match b {
            Link::Elem(ref node) => {
                println!("b.elem: {:?}", node.elem);
                println!("b.next: {:?}", node.next);
            }
            Link::Empty => println!("b.elem: Empty"),
        }
    }

    #[test]
    fn test_3() {
        let a = Link::Elem(Box::new(Node {
            elem: 100,
            next: Link::Empty,
        }));
        println!("a: {:?}", a);
        match a {
            Link::Elem(ref node) => {
                println!("a.elem: {:?}", node.elem);
                println!("a.next: {:?}", node.next);
            }
            Link::Empty => println!("a.elem: Empty"),
        }

        let b = Link::Elem(Box::new(Node { elem: 10, next: a }));
        println!("b: {:?}", b);
        match b {
            Link::Elem(ref node) => {
                println!("b.elem: {:?}", node.elem);
                println!("b.next: {:?}", node.next);
            }
            Link::Empty => println!("b.elem: Empty"),
        }
        let list = List { head: b };
        println!("list: {:?}", list);
        println!("list.head: {:?}", list.head);
    }

    #[test]
    fn test_all() {
        let mut list = List::new();
        println!("list: {:?}", list);
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);
        println!("list: {:?}", list);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        println!("list: {:?}", list);
    }
}
