use std::cell::RefCell;
use std::rc::Rc;

// The core of Data: Node
type CoreNode<T> = Rc<RefCell<Node<T>>>;

// The Link of CoreNode
pub type LinkNode<T> = Option<CoreNode<T>>;

#[derive(Debug)]
pub struct Node<T> {
    val: T,
    next: LinkNode<T>,
    prev: LinkNode<T>,
}

impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        Node {
            val,
            prev: None,
            next: None,
        }
    }

    pub fn val(&self) -> &T {
        &self.val
    }

    pub fn set_next_by_raw_val(&mut self, val: T) {
        self.next = Some(Rc::new(RefCell::new(Self::new(val))))
    }

    pub fn set_prev_by_raw_val(&mut self, val: T) {
        self.prev = Some(Rc::new(RefCell::new(Self::new(val))))
    }

    pub fn get_next(&self) -> LinkNode<T> {
        match self.next.as_ref() {
            None => None,
            Some(core_node) => Some(core_node.clone()),
        }
    }

    pub fn get_prev(&self) -> LinkNode<T> {
        match self.prev.as_ref() {
            None => None,
            Some(core_node) => Some(core_node.clone()),
        }
    }
}

mod test {
    use crate::list::node::Node;

    #[test]
    fn test_new() {
        let node = Node::new(5);
        println!("{:?}", node)
    }

    #[test]
    fn test_set() {
        let mut node = Node::new(String::from("abc"));
        node.set_next_by_raw_val(String::from("next"));
        node.set_prev_by_raw_val(String::from("prev"));

        println!("{:?}", node)
    }

    #[test]
    fn test_get() {
        let mut node = Node::new(String::from("abc"));
        node.set_next_by_raw_val(String::from("next"));
        node.set_prev_by_raw_val(String::from("prev"));

        println!("{:?}", node.get_next().unwrap());
        println!("{:?}", node.get_prev().unwrap());

        println!("{:?}", node.get_next().unwrap()); // Still can take value
    }

}
