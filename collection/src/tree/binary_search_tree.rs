use std::cmp::max;
use std::marker::PhantomData;
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

    fn into_val(self: Box<Self>) -> T {
        self.val
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
    /// ```
    /// use collection::tree::binary_search_tree::BinarySearchTree;
    /// let mut tree = BinarySearchTree::new();
    /// ```
    /// # Panics
    /// This function will panic if the type `T` is not `PartialOrd`.
    /// ```should_panic
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

        self.size -= 1;
        self._min().map(|mut node| {
            unsafe {
                let node = Box::from_raw(node.as_ptr());
                self.remove(&node.val);
                node.into_val()
            }
        })
    }

    pub fn insert(&mut self, elem: T) {}

    pub fn remove(&mut self, elem: &T) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        None
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

    fn _find_node(&self, elem: &T) -> Option<NonNull<TreeNode<T>>> {
        if self.is_empty() {
            return None;
        }

        while let Some(node) = self._find_node_recursive(elem, self.root) {
            if node.val == *elem {
                return Some(node);
            }
        }

        None
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
        max(BinarySearchTree::_height(left), BinarySearchTree::_height(right)) + 1
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
        while unsafe { (*node.as_ptr()).left.is_some() } {
            node = unsafe { (*node.as_ptr()).left.unwrap() };
        }
        Some(node)
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
        IntoIter {
            tree: self,
        }
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

impl<T> Drop for IntoIter<T> where T: PartialOrd {
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
    fn test_binary_search_tree() {
        let mut tree: BinarySearchTree<i32> = BinarySearchTree::new();
        assert_eq!(tree.size(), 0);
        assert!(tree.is_empty());
        assert_eq!(tree.height(), 0);
        assert_eq!(tree.min(), None);
        assert_eq!(tree.max(), None);
        assert_eq!(tree.size(), 1);
        assert!(!tree.is_empty());
    }
}
