use std::cmp::max;
use std::fmt::Display;
use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

struct TreeNode<T> {
    pub val: T,
    pub left: Option<NonNull<TreeNode<T>>>,
    pub right: Option<NonNull<TreeNode<T>>>,
    pub parent: Option<NonNull<TreeNode<T>>>,
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
}

pub struct BinarySearchTree<T>
where
    T: PartialOrd,
{
    size: usize,
    root: Option<NonNull<TreeNode<T>>>,
    _marker: PhantomData<Box<TreeNode<T>>>,
}

impl<T: std::cmp::PartialOrd> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: std::cmp::PartialOrd> BinarySearchTree<T> {
    /// Creates a new, empty binary search tree.
    /// # Examples
    ///
    /// ```
    /// use collection::tree::binary_search_tree::BinarySearchTree;
    /// let mut tree: BinarySearchTree<i32> = BinarySearchTree::new();
    /// ```
    pub fn new() -> Self {
        Self {
            size: 0,
            root: None,
            _marker: PhantomData,
        }
    }

    /// Returns the number of elements in the binary search tree.
    /// # Examples
    /// ```
    /// use collection::tree::binary_search_tree::BinarySearchTree;
    /// let mut tree = BinarySearchTree::new();
    /// assert_eq!(tree.size(), 0);
    /// tree.insert(1);
    /// assert_eq!(tree.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns `true` if the binary search tree is empty.
    /// # Examples
    /// ```
    /// use collection::tree::binary_search_tree::BinarySearchTree;
    /// let mut tree = BinarySearchTree::new();
    /// assert!(tree.is_empty());
    /// tree.insert(1);
    /// assert!(!tree.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Pop the minimum value from the binary search tree.
    /// # Examples
    /// ```
    /// use collection::tree::binary_search_tree::BinarySearchTree;
    /// let mut tree = BinarySearchTree::new();
    /// tree.insert(1);
    /// tree.insert(2);
    /// tree.insert(3);
    /// assert_eq!(tree.pop_min(), Some(1));
    /// assert_eq!(tree.pop_min(), Some(2));
    /// assert_eq!(tree.pop_min(), Some(3));
    /// assert_eq!(tree.pop_min(), None);
    /// ```
    pub fn pop_min(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self._min()
            .map(|node| unsafe { self.remove(&(*node.as_ptr()).val).unwrap() })
    }

    /// Insert a value into the binary search tree.
    ///
    /// # Examples
    /// ```
    /// use collection::tree::binary_search_tree::BinarySearchTree;
    /// let mut tree = BinarySearchTree::new();
    /// tree.insert(1);
    /// tree.insert(2);
    /// tree.insert(3);
    /// ```
    pub fn insert(&mut self, elem: T) {
        let mut node = Box::new(TreeNode::new(elem));
        node.left = None;
        node.right = None;
        node.parent = None;
        let node = NonNull::new(Box::into_raw(node));

        // tree is empty
        if self.is_empty() {
            self.root = node;
            self.size += 1;
            return;
        }

        // tree is not empty

        // find the parent node
        let mut curr = self.root.unwrap();
        loop {
            unsafe {
                if (*node.unwrap().as_ptr()).val < (*curr.as_ptr()).val {
                    if (*curr.as_ptr()).left.is_none() {
                        (*curr.as_ptr()).left = node;
                        break;
                    } else {
                        curr = (*curr.as_ptr()).left.unwrap();
                    }
                } else {
                    if (*curr.as_ptr()).right.is_none() {
                        (*curr.as_ptr()).right = node;
                        break;
                    } else {
                        curr = (*curr.as_ptr()).right.unwrap();
                    }
                }
            }
        }

        // set the parent node
        unsafe {
            (*node.unwrap().as_ptr()).parent = Some(curr);
        }

        // update the size
        self.size += 1;
    }

    /// Remove a value from the binary search tree.
    ///
    /// # Examples
    /// ```
    /// use collection::tree::binary_search_tree::BinarySearchTree;
    /// let mut tree = BinarySearchTree::new();
    /// tree.insert(1);
    /// tree.insert(2);
    /// tree.insert(3);
    /// tree.remove(&2);
    /// assert!(!tree.contains(&2));
    /// assert!(tree.contains(&1));
    /// ```
    pub fn remove(&mut self, elem: &T) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let removed_node = self._find_node(elem);
        if removed_node.is_none() {
            return None;
        }

        let removed_node = removed_node.unwrap();
        Some(self._remove_node(removed_node))
    }

    /// Remove a node from the binary search tree.
    fn _remove_node(&mut self, mut node: NonNull<TreeNode<T>>) -> T {
        let mut removed_node = unsafe { Box::from_raw(node.as_mut()) };

        // node has no children
        if removed_node.left.is_none() && removed_node.right.is_none() {
            // node is root
            if removed_node.parent.is_none() {
                self.root = None;
            } else {
                let mut parent = unsafe { &mut *removed_node.parent.unwrap().as_mut() };
                if parent.left == Some(node) {
                    parent.left = None;
                } else {
                    parent.right = None;
                }
            }
        } else if removed_node.left.is_none() || removed_node.right.is_none() {
            // node has one child, find the child
            let mut child = if removed_node.left.is_some() {
                removed_node.left.unwrap()
            } else {
                removed_node.right.unwrap()
            };

            // node is root
            if removed_node.parent.is_none() {
                self.root = Some(child);
                unsafe {
                    (*child.as_ptr()).parent = None;
                }
            } else {
                // node is not root
                unsafe {
                    let mut parent = &mut *removed_node.parent.unwrap().as_mut();
                    if parent.left == Some(node) {
                        parent.left = Some(child);
                    } else {
                        parent.right = Some(child);
                    }

                    (*child.as_mut()).parent = removed_node.parent;
                }
            }
        } else {
            // node has two children
            let successor = self._find_successor(node);
            let successor_node = successor.unwrap();
            let successor_parent =
                unsafe { &mut (*successor_node.as_ptr()).parent.unwrap().as_mut() };

            // swap the values
            mem::swap(&mut removed_node.val, &mut successor_parent.val);
        }

        removed_node.left = None;
        removed_node.right = None;
        removed_node.parent = None;

        self.size -= 1;

        removed_node.val
    }

    /// Find the successor of a node.
    fn _find_successor(&self, node: NonNull<TreeNode<T>>) -> Option<NonNull<TreeNode<T>>> {
        let curr = unsafe { &*node.as_ptr() };
        if curr.right.is_some() {
            let mut curr = curr.right.unwrap();
            loop {
                unsafe {
                    if (*curr.as_ptr()).left.is_none() {
                        return Some(curr);
                    } else {
                        curr = (*curr.as_ptr()).left.unwrap();
                    }
                }
            }
        } else {
            let mut curr = node;
            loop {
                unsafe {
                    if (*curr.as_ptr()).parent.is_none() {
                        return None;
                    } else {
                        let parent = (*curr.as_ptr()).parent.unwrap();
                        if (*parent.as_ptr()).right == Some(curr) {
                            return Some(parent);
                        } else {
                            curr = parent;
                        }
                    }
                }
            }
        }
    }

    /// Returns if the binary search tree contains the given element.
    /// # Examples
    /// ```
    /// use collection::tree::binary_search_tree::BinarySearchTree;
    /// let mut tree = BinarySearchTree::new();
    /// assert!(!tree.contains(&1));
    /// tree.insert(1);
    /// assert!(tree.contains(&1));
    /// ```
    pub fn contains(&self, elem: &T) -> bool {
        self._find_node(elem).is_some()
    }

    /// Find the internal node with the given value.
    ///
    /// # Examples
    /// ```
    /// use collection::tree::binary_search_tree::BinarySearchTree;
    /// let mut tree = BinarySearchTree::new();
    /// tree.insert(1);
    /// //assert!(tree._find_node(&1).is_some());
    /// //assert!(tree._find_node(&2).is_none());
    /// ```
    fn _find_node(&self, elem: &T) -> Option<NonNull<TreeNode<T>>> {
        if self.is_empty() {
            return None;
        }

        let mut node = self.root.unwrap();
        loop {
            unsafe {
                if elem < &(*node.as_ptr()).val {
                    if (*node.as_ptr()).left.is_none() {
                        return None;
                    }
                    node = (*node.as_ptr()).left.unwrap();
                } else if elem > &(*node.as_ptr()).val {
                    if (*node.as_ptr()).right.is_none() {
                        return None;
                    }
                    node = (*node.as_ptr()).right.unwrap();
                } else {
                    return Some(node);
                }
            }
        }
    }

    /// Returns the height of the binary search tree.
    /// # Examples
    /// ```
    /// use collection::tree::binary_search_tree::BinarySearchTree;
    /// let mut tree = BinarySearchTree::new();
    /// assert_eq!(tree.height(), 0);
    /// tree.insert(1);
    /// assert_eq!(tree.height(), 1);
    /// tree.insert(2);
    /// assert_eq!(tree.height(), 2);
    /// tree.insert(3);
    /// assert_eq!(tree.height(), 3);
    /// ```
    pub fn height(&self) -> usize {
        BinarySearchTree::_height(self.root)
    }

    fn _height(root: Option<NonNull<TreeNode<T>>>) -> usize {
        if root.is_none() {
            return 0;
        }
        let root = root.unwrap();
        let left = unsafe { (*root.as_ptr()).left };
        let right = unsafe { (*root.as_ptr()).right };
        max(
            BinarySearchTree::_height(left),
            BinarySearchTree::_height(right),
        ) + 1
    }

    /// Returns the minimum element of the binary search tree.
    /// # Examples
    /// ```
    /// use collection::tree::binary_search_tree::BinarySearchTree;
    /// let mut tree = BinarySearchTree::new();
    /// assert_eq!(tree.min(), None);
    /// tree.insert(1);
    /// assert_eq!(tree.min(), Some(&1));
    /// tree.insert(2);
    /// assert_eq!(tree.min(), Some(&1));
    /// tree.insert(3);
    /// assert_eq!(tree.min(), Some(&1));
    /// ```
    pub fn min(&self) -> Option<&T> {
        self._min().map(|x| unsafe { &(*x.as_ptr()).val })
    }

    fn _min(&self) -> Option<NonNull<TreeNode<T>>> {
        if self.root.is_none() {
            return None;
        }

        let mut node = self.root.unwrap();
        loop {
            unsafe {
                if (*node.as_ptr()).left.is_none() {
                    return Some(node);
                }
                node = (*node.as_ptr()).left.unwrap();
            }
        }
    }

    /// Returns the maximum element of the binary search tree.
    /// # Examples
    /// ```
    /// use collection::tree::binary_search_tree::BinarySearchTree;
    /// let mut tree = BinarySearchTree::new();
    /// assert_eq!(tree.max(), None);
    /// tree.insert(1);
    /// assert_eq!(tree.max(), Some(&1));
    /// tree.insert(2);
    /// assert_eq!(tree.max(), Some(&2));
    /// tree.insert(3);
    /// assert_eq!(tree.max(), Some(&3));
    /// ```
    pub fn max(&self) -> Option<&T> {
        self._max().map(|x| unsafe { &(*x.as_ptr()).val })
    }

    fn _max(&self) -> Option<NonNull<TreeNode<T>>> {
        if self.root.is_none() {
            return None;
        }

        let mut node = self.root.unwrap();
        while unsafe { (*node.as_ptr()).right.is_some() } {
            node = unsafe { (*node.as_ptr()).right.unwrap() };
        }
        Some(node)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter { tree: self }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            root: self.root,
            _marker: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            root: self.root,
            _marker: PhantomData,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: PartialOrd + Display,
{
    pub fn traverse(&self) {
        if self.root.is_none() {
            println!("Empty tree");
            return;
        }

        let mut stack = Vec::new();
        let mut node = self.root.unwrap();
        print!("{{");
        loop {
            if unsafe { (*node.as_ptr()).left.is_some() } {
                stack.push(node);
                node = unsafe { (*node.as_ptr()).left.unwrap() };
            } else {
                unsafe {
                    let raw_node = &(*node.as_ptr());
                    print!("[");

                    // print node
                    print!("val: {}, ", raw_node.val);

                    // print left child
                    print!("left: ");
                    if raw_node.left.is_some() {
                        print!("{}, ", &(*raw_node.left.unwrap().as_ptr()).val);
                    } else {
                        print!("None, ");
                    }

                    // print right child
                    print!("right: ");
                    if raw_node.right.is_some() {
                        print!("{}, ", &(*raw_node.right.unwrap().as_ptr()).val);
                    } else {
                        print!("None, ");
                    }

                    // print parent
                    print!("parent: ");
                    if raw_node.parent.is_some() {
                        print!("{}", &(*raw_node.parent.unwrap().as_ptr()).val);
                    } else {
                        print!("None");
                    }

                    print!("] ");
                }
                if unsafe { (*node.as_ptr()).right.is_some() } {
                    node = unsafe { (*node.as_ptr()).right.unwrap() };
                } else if stack.len() > 0 {
                    node = stack.pop().unwrap();
                } else {
                    break;
                }
            }
        }
        println!("}}");
    }
}

impl<T: PartialOrd, const N: usize> From<[T; N]> for BinarySearchTree<T> {
    fn from(s: [T; N]) -> Self {
        let mut tree = Self::new();
        for elem in s.into_iter() {
            tree.insert(elem);
        }
        tree
    }
}

pub struct IntoIter<T: PartialOrd> {
    tree: BinarySearchTree<T>,
}

impl<T> Drop for IntoIter<T>
where
    T: PartialOrd,
{
    fn drop(&mut self) {
        // only need to ensure all our elements are read;
        // buffer will clean itself up afterwards.
        for _ in &mut *self {}

        println!("IntoIter has been dropped!")
    }
}

impl<T: PartialOrd> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.tree.pop_min()
    }
}

pub struct Iter<'a, T: 'a> {
    root: Option<NonNull<TreeNode<T>>>,
    _marker: PhantomData<&'a TreeNode<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.root.is_none() {
            return None;
        }
        let root = self.root.unwrap();
        let left = unsafe { (*root.as_ptr()).left };
        let right = unsafe { (*root.as_ptr()).right };
        if left.is_none() {
            self.root = right;
            return Some(unsafe { &(*root.as_ptr()).val });
        }
        self.root = left;
        self.next()
    }
}

pub struct IterMut<'a, T: 'a> {
    root: Option<NonNull<TreeNode<T>>>,
    _marker: PhantomData<&'a mut TreeNode<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.root.is_none() {
            return None;
        }
        let root = self.root.unwrap();
        let left = unsafe { (*root.as_ptr()).left };
        let right = unsafe { (*root.as_ptr()).right };
        if left.is_none() {
            self.root = right;
            return Some(unsafe { &mut (*root.as_ptr()).val });
        }
        self.root = left;
        self.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiling() {
        let _ = BinarySearchTree::<i32>::new();
    }

    #[test]
    fn test_insert() {
        let mut tree = BinarySearchTree::new();
        tree.insert(1);
        tree.insert(2);
        tree.insert(3);
    }

    #[test]
    fn test_remove() {
        let mut tree = BinarySearchTree::new();
        tree.insert(1);
        tree.insert(2);
        tree.insert(3);
        assert_eq!(tree.size, 3);

        tree.remove(&1);
        assert_eq!(tree.size, 2);
        assert!(!tree.contains(&1));
        assert!(tree.contains(&2));
        assert!(tree.contains(&3));

        tree.remove(&2);
        assert_eq!(tree.size, 1);
        assert!(!tree.contains(&2));
        assert!(tree.contains(&3));
    }

    #[test]
    fn test_pop_min() {
        let mut tree = BinarySearchTree::new();
        tree.insert(1);
        tree.insert(2);
        tree.insert(3);
        assert_eq!(tree.size(), 3);

        assert_eq!(tree.pop_min(), Some(1));
        tree.traverse();
        assert_eq!(tree.size(), 2);
        assert!(!tree.contains(&1));

        assert_eq!(tree.pop_min(), Some(2));
        tree.traverse();
        assert_eq!(tree.size(), 1);
        assert!(!tree.contains(&2));

        assert_eq!(tree.pop_min(), Some(3));
        assert_eq!(tree.size(), 0);
        assert!(!tree.contains(&3));

        assert_eq!(tree.pop_min(), None);
    }

    #[test]
    fn test_pop_min2() {
        let mut tree = BinarySearchTree::new();
        tree.insert(2);
        tree.insert(1);
        tree.insert(3);
        assert_eq!(tree.size(), 3);

        assert_eq!(tree.pop_min(), Some(1));
        tree.traverse();
        assert_eq!(tree.size(), 2);
        assert!(!tree.contains(&1));

        assert_eq!(tree.pop_min(), Some(2));
        tree.traverse();
        assert_eq!(tree.size(), 1);
        assert!(!tree.contains(&2));

        assert_eq!(tree.pop_min(), Some(3));
        assert_eq!(tree.size(), 0);
        assert!(!tree.contains(&3));

        assert_eq!(tree.pop_min(), None);
    }

}
