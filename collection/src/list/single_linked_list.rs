use crate::list::node::LinkNode;

#[derive(Debug)]
pub struct SingleLinkedList<T> {
    head: LinkNode<T>,
    tail: LinkNode<T>,
    length: isize,
}
