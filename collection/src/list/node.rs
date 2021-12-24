use std::cell::RefCell;
use std::rc::Rc;

// The core of Data: Node
type CoreNode<T> = Rc<RefCell<Node<T>>>;

// The Link of CoreNode
pub type LinkNode<T> = Option<CoreNode<T>>;

#[derive(Debug)]
pub struct Node<T> {
    pub val: T,
    pub next: LinkNode<T>,
    pub prev: LinkNode<T>,
}

impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        Node {
            val,
            prev: None,
            next: None,
        }
    }

    pub fn new_core_node(val: T) -> CoreNode<T> {
        Rc::new(RefCell::new(Self::new(val)))
    }

    pub fn val(&self) -> &T {
        &self.val
    }

    pub fn set_next_by_raw_val(&mut self, val: T) -> LinkNode<T> {
        let old_val = self.next.take();
        self.next = Some(Rc::new(RefCell::new(Self::new(val))));
        old_val
    }

    pub fn set_prev_by_raw_val(&mut self, val: T) -> LinkNode<T> {
        let old_val = self.prev.take();
        self.prev = Some(Rc::new(RefCell::new(Self::new(val))));
        old_val
    }

    pub fn get_next(&self) -> LinkNode<T> {
        match self.next.as_ref() {
            None => None,
            Some(core_node) => Some(Rc::clone(core_node)),
        }
    }

    pub fn get_prev(&self) -> LinkNode<T> {
        match self.prev.as_ref() {
            None => None,
            Some(core_node) => Some(Rc::clone(core_node)),
        }
    }
}

mod test {
    use crate::list::node::Node;
    use std::rc::Rc;

    #[test]
    fn test_new() {
        let node = Node::new(5);
        println!("{:?}", node)
    }

    #[test]
    fn test_get_val() {
        let node = Node::new(2.2);
        println!("{}", node.val())
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

        println!(
            "next: {:?}",
            node.get_next().unwrap().try_borrow().unwrap().val
        );

        let prev1 = node.get_prev().unwrap();
        println!("prev: {:?}", prev1.try_borrow().unwrap().val);

        // Still can take value, but different from above
        let prev2 = node.get_prev().unwrap();
        println!("prev: {:?}", prev2.try_borrow().unwrap().val);

        println!(
            "prev: {:?}",
            node.get_prev().unwrap().try_borrow().unwrap().val // Rc will be optimized!
        );

        let prev3 = node.get_prev().unwrap();
        println!("rc count: {}", Rc::strong_count(&prev3)); // 4
    }
}
