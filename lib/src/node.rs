use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::with::With;

#[derive(Debug)]
pub struct Node<T> {
    value: Option<T>,
    parent: Option<RefCell<Weak<Node<T>>>>,
    next: Option<RefCell<Rc<Node<T>>>>,
}

impl<T> Default for Node<T> {
    fn default() -> Self {
        Node { value: None, parent: None, next: None }
    }
}

impl<T> With<T> for Node<T> {
    fn with(x: T) -> Self {
        Node {
            value: Some(x),
            ..Default::default()
        }
    }
}

impl<T> Node<T> {
    fn new() -> Self {
        Node { value: None, parent: None, next: None }
    }

    pub fn value(&self) -> &Option<T> {
        &self.value
    }
    pub fn parent(&self) -> &Option<RefCell<Weak<Node<T>>>> {
        &self.parent
    }
    pub fn next(&self) -> &Option<RefCell<Rc<Node<T>>>> {
        &self.next
    }
    pub fn set_value(&mut self, value: Option<T>) {
        self.value = value;
    }
    pub fn set_parent(&mut self, parent: Option<RefCell<Weak<Node<T>>>>) {
        self.parent = parent;
    }
    pub fn set_next(&mut self, next: Option<RefCell<Rc<Node<T>>>>) {
        self.next = next;
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::node::Node;
    use crate::with::With;

    #[test]
    fn init_node_test() {
        let node1 = Node::with(String::from("hello"));
        println!("init node1: {:?}", node1);

        let mut node2 = Node::new();
        node2.value = Some(String::from("hello"));
        println!("init node2: {:?}", node2);
    }

    #[test]
    fn getter_test() {
        let node = Node::with(String::from("hello"));
        println!("init node: {:?}", node);
        println!("value: {:?}, parent: {:?}, next: {:?}", node.value(), node.parent(), node.next());
    }

    #[test]
    fn setter_test() {
        let mut node = Node::with(String::from("hello"));
        node.set_value(Option::from(String::from("world")));

        let parent = Node::with(String::from("parent"));
        let next = Node::with(String::from("next"));
        node.set_parent(Some(RefCell::new(Rc::downgrade(&Rc::new(parent))).clone()));
        node.set_next(Some(RefCell::new(Rc::new(next)).clone()));

        println!("init node: {:?}", node);
        println!("value: {:?}, parent: {:?}, next: {:?}", node.value(), node.parent(), node.next());
    }
}
