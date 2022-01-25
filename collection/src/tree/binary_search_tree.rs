use std::marker::PhantomData;
use std::ptr::NonNull;

struct TreeNode<T> {
    val: T,
    left: Option<NonNull<TreeNode<T>>>,
    right: Option<NonNull<TreeNode<T>>>,
    parent: Option<NonNull<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    fn new(val: T) -> TreeNode<T> {
        TreeNode {
            val,
            left: None,
            right: None,
            parent: None,
        }
    }

    fn into_val(self: Box<Self>) -> T {
        self.val
    }
}

pub struct BinarySearchTree<'a, T: 'a>
where
    T: PartialOrd,
{
    size: usize,
    min: &'a TreeNode<T>,
    max: &'a TreeNode<T>,
    root: Option<NonNull<TreeNode<T>>>,
    _marker: PhantomData<Box<TreeNode<T>>>,
}

impl<T> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            min: &None,
            max: &None,
            root: None,
            _marker: PhantomData,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn height(&self) -> usize {
        todo!()
    }

    pub fn from(arr: &[T]) -> Self {
        todo!()
    }

    pub fn insert(&mut self, elem: T) {
        todo!()
    }

    pub fn remove(&mut self, elem: &T) {
        todo!()
    }

    pub fn contains(&self, elem: &T) -> bool {
        todo!()
    }

    pub fn min(&self) -> Option<&T> {
        todo!()
    }

    pub fn max(&self) -> Option<&T> {
        todo!()
    }

    pub fn pre_order_traversal(&self) {
        todo!()
    }

    pub fn into_iter(self) -> IntoIter<T> {
        todo!()
    }

    pub fn iter(&self) -> Iter<T> {
        todo!()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        todo!()
    }

    pub fn tree_node_cursor(&self) -> TreeNodeCursor<T> {
        todo!()
    }

    pub fn tree_node_cursor_mut(&self) -> TreeNodeCursorMut<T> {
        todo!()
    }
}

pub struct IntoIter<T> {}

pub struct Iter<T> {}

pub struct IterMut<T> {}

pub struct TreeNodeCursor<T> {}

pub struct TreeNodeCursorMut<T> {}
