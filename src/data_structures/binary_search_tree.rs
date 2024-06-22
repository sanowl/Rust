use std::cmp::Ordering;
use std::sync::Arc;
use std::fmt::Debug;

pub struct BinarySearchTree<T>
where
    T: Ord + Clone + Debug,
{
    root: Option<Arc<Node<T>>>,
}

#[derive(Clone, Debug)]
struct Node<T>
where
    T: Ord + Clone + Debug,
{
    value: T,
    left: Option<Arc<Node<T>>>,
    right: Option<Arc<Node<T>>>,
    height: usize,
}

impl<T> Default for BinarySearchTree<T>
where
    T: Ord + Clone + Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord + Clone + Debug,
{
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    pub fn search(&self, value: &T) -> bool {
        self.root.as_ref().map_or(false, |node| node.search(value))
    }

    pub fn insert(&mut self, value: T) {
        self.root = Some(match self.root.take() {
            Some(node) => Arc::new((*node).clone().insert(value)),
            None => Arc::new(Node::new(value)),
        });
    }

    pub fn minimum(&self) -> Option<&T> {
        self.root.as_ref().map(|node| node.minimum())
    }

    pub fn maximum(&self) -> Option<&T> {
        self.root.as_ref().map(|node| node.maximum())
    }

    pub fn floor(&self, value: &T) -> Option<&T> {
        self.root.as_ref().and_then(|node| node.floor(value))
    }

    pub fn ceil(&self, value: &T) -> Option<&T> {
        self.root.as_ref().and_then(|node| node.ceil(value))
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        BinarySearchTreeIter::new(self.root.as_ref())
    }

    pub fn height(&self) -> usize {
        self.root.as_ref().map_or(0, |node| node.height)
    }

    pub fn remove(&mut self, value: &T) {
        if let Some(root) = self.root.take() {
            self.root = Node::remove(root, value);
        }
    }
}

impl<T> Node<T>
where
    T: Ord + Clone + Debug,
{
    fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
            height: 1,
        }
    }

    fn search(&self, value: &T) -> bool {
        match self.value.cmp(value) {
            Ordering::Equal => true,
            Ordering::Greater => self.left.as_ref().map_or(false, |node| node.search(value)),
            Ordering::Less => self.right.as_ref().map_or(false, |node| node.search(value)),
        }
    }

    fn insert(mut self, value: T) -> Self {
        match self.value.cmp(&value) {
            Ordering::Less => {
                self.right = Some(match self.right.take() {
                    Some(node) => Arc::new((*node).clone().insert(value)),
                    None => Arc::new(Node::new(value)),
                });
            }
            Ordering::Greater => {
                self.left = Some(match self.left.take() {
                    Some(node) => Arc::new((*node).clone().insert(value)),
                    None => Arc::new(Node::new(value)),
                });
            }
            Ordering::Equal => return self,
        }
        self.update_height();
        self.balance()
    }

    fn minimum(&self) -> &T {
        self.left.as_ref().map_or(&self.value, |node| node.minimum())
    }

    fn maximum(&self) -> &T {
        self.right.as_ref().map_or(&self.value, |node| node.maximum())
    }

    fn floor(&self, value: &T) -> Option<&T> {
        match self.value.cmp(value) {
            Ordering::Equal => Some(&self.value),
            Ordering::Greater => self.left.as_ref().and_then(|node| node.floor(value)),
            Ordering::Less => self.right.as_ref().and_then(|node| node.floor(value)).or(Some(&self.value)),
        }
    }

    fn ceil(&self, value: &T) -> Option<&T> {
        match self.value.cmp(value) {
            Ordering::Equal => Some(&self.value),
            Ordering::Less => self.right.as_ref().and_then(|node| node.ceil(value)),
            Ordering::Greater => self.left.as_ref().and_then(|node| node.ceil(value)).or(Some(&self.value)),
        }
    }

    fn update_height(&mut self) {
        self.height = 1 + std::cmp::max(
            self.left.as_ref().map_or(0, |node| node.height),
            self.right.as_ref().map_or(0, |node| node.height),
        );
    }

    fn balance_factor(&self) -> i8 {
        let left_height = self.left.as_ref().map_or(0, |node| node.height);
        let right_height = self.right.as_ref().map_or(0, |node| node.height);
        (left_height as i8) - (right_height as i8)
    }

    fn balance(mut self) -> Self {
        let balance = self.balance_factor();
        if balance > 1 {
            if self.left.as_ref().unwrap().balance_factor() < 0 {
                self.left = Some(Arc::new((*self.left.unwrap()).clone().rotate_left()));
            }
            self.rotate_right()
        } else if balance < -1 {
            if self.right.as_ref().unwrap().balance_factor() > 0 {
                self.right = Some(Arc::new((*self.right.unwrap()).clone().rotate_right()));
            }
            self.rotate_left()
        } else {
            self
        }
    }

    fn rotate_right(mut self) -> Self {
        let mut new_root = match self.left.take() {
            Some(left) => (*left).clone(),
            None => return self,
        };
        self.left = new_root.right.take();
        self.update_height();
        new_root.right = Some(Arc::new(self));
        new_root.update_height();
        new_root
    }

    fn rotate_left(mut self) -> Self {
        let mut new_root = match self.right.take() {
            Some(right) => (*right).clone(),
            None => return self,
        };
        self.right = new_root.left.take();
        self.update_height();
        new_root.left = Some(Arc::new(self));
        new_root.update_height();
        new_root
    }

    fn remove(node: Arc<Node<T>>, value: &T) -> Option<Arc<Node<T>>> {
        let mut node = (*node).clone();
        match value.cmp(&node.value) {
            Ordering::Less => {
                if let Some(left) = node.left.take() {
                    node.left = Node::remove(left, value);
                    Some(Arc::new(node.balance()))
                } else {
                    Some(Arc::new(node))
                }
            }
            Ordering::Greater => {
                if let Some(right) = node.right.take() {
                    node.right = Node::remove(right, value);
                    Some(Arc::new(node.balance()))
                } else {
                    Some(Arc::new(node))
                }
            }
            Ordering::Equal => match (node.left.take(), node.right.take()) {
                (None, None) => None,
                (Some(left), None) => Some(left),
                (None, Some(right)) => Some(right),
                (Some(left), Some(right)) => {
                    let mut successor = (*right).clone();
                    let min_value = successor.minimum().clone();
                    node.value = min_value;
                    node.right = Node::remove(right, &node.value);
                    Some(Arc::new(node.balance()))
                }
            },
        }
    }
}

struct BinarySearchTreeIter<'a, T>
where
    T: Ord + Clone + Debug,
{
    stack: Vec<&'a Node<T>>,
}

impl<'a, T> BinarySearchTreeIter<'a, T>
where
    T: Ord + Clone + Debug,
{
    fn new(root: Option<&'a Arc<Node<T>>>) -> Self {
        let mut iter = BinarySearchTreeIter { stack: Vec::new() };
        if let Some(node) = root {
            iter.stack_push_left(node);
        }
        iter
    }

    fn stack_push_left(&mut self, mut node: &'a Node<T>) {
        while let Some(left) = node.left.as_ref() {
            self.stack.push(node);
            node = left;
        }
        self.stack.push(node);
    }
}

impl<'a, T> Iterator for BinarySearchTreeIter<'a, T>
where
    T: Ord + Clone + Debug,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            if let Some(right) = node.right.as_ref() {
                self.stack_push_left(right);
            }
            Some(&node.value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn prequel_memes_tree() -> BinarySearchTree<&'static str> {
        let mut tree = BinarySearchTree::new();
        tree.insert("hello there");
        tree.insert("general kenobi");
        tree.insert("you are a bold one");
        tree.insert("kill him");
        tree.insert("back away...I will deal with this jedi slime myself");
        tree.insert("your move");
        tree.insert("you fool");
        tree
    }

    #[test]
    fn test_search() {
        let tree = prequel_memes_tree();
        assert!(tree.search(&"hello there"));
        assert!(tree.search(&"you are a bold one"));
        assert!(tree.search(&"general kenobi"));
        assert!(tree.search(&"you fool"));
        assert!(tree.search(&"kill him"));
        assert!(!tree.search(&"but i was going to tosche station to pick up some power converters"));
        assert!(!tree.search(&"only a sith deals in absolutes"));
        assert!(!tree.search(&"you underestimate my power"));
    }

    #[test]
    fn test_maximum_and_minimum() {
        let tree = prequel_memes_tree();
        assert_eq!(*tree.maximum().unwrap(), "your move");
        assert_eq!(*tree.minimum().unwrap(), "back away...I will deal with this jedi slime myself");
        
        let mut tree2: BinarySearchTree<i32> = BinarySearchTree::new();
        assert!(tree2.maximum().is_none());
        assert!(tree2.minimum().is_none());
        tree2.insert(0);
        assert_eq!(*tree2.minimum().unwrap(), 0);
        assert_eq!(*tree2.maximum().unwrap(), 0);
        tree2.insert(-5);
        assert_eq!(*tree2.minimum().unwrap(), -5);
        assert_eq!(*tree2.maximum().unwrap(), 0);
        tree2.insert(5);
        assert_eq!(*tree2.minimum().unwrap(), -5);
        assert_eq!(*tree2.maximum().unwrap(), 5);
    }

    #[test]
    fn test_floor_and_ceil() {
        let tree = prequel_memes_tree();
        assert_eq!(*tree.floor(&"hello there").unwrap(), "hello there");
        assert_eq!(*tree.floor(&"these are not the droids you're looking for").unwrap(), "kill him");
        assert!(tree.floor(&"another death star").is_none());
        assert_eq!(*tree.floor(&"you fool").unwrap(), "you fool");
        assert_eq!(*tree.floor(&"but i was going to tasche station").unwrap(), "back away...I will deal with this jedi slime myself");
        assert_eq!(*tree.floor(&"you underestimate my power").unwrap(), "you fool");
        assert_eq!(*tree.floor(&"your new empire").unwrap(), "your move");
        assert_eq!(*tree.ceil(&"hello there").unwrap(), "hello there");
        assert_eq!(*tree.ceil(&"these are not the droids you're looking for").unwrap(), "you are a bold one");
        assert_eq!(*tree.ceil(&"another death star").unwrap(), "back away...I will deal with this jedi slime myself");
        assert_eq!(*tree.ceil(&"you fool").unwrap(), "you fool");
        assert_eq!(*tree.ceil(&"but i was going to tasche station").unwrap(), "general kenobi");
        assert_eq!(*tree.ceil(&"you underestimate my power").unwrap(), "your move");
        assert!(tree.ceil(&"your new empire").is_none());
    }

    #[test]
    fn test_iterator() {
        let tree = prequel_memes_tree();
        let mut iter = tree.iter();
        assert_eq!(iter.next().unwrap(), &"back away...I will deal with this jedi slime myself");
        assert_eq!(iter.next().unwrap(), &"general kenobi");
        assert_eq!(iter.next().unwrap(), &"hello there");
        assert_eq!(iter.next().unwrap(), &"kill him");
        assert_eq!(iter.next().unwrap(), &"you are a bold one");
        assert_eq!(iter.next().unwrap(), &"you fool");
        assert_eq!(iter.next().unwrap(), &"your move");
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_remove() {
        let mut tree = BinarySearchTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(2);
        tree.insert(4);
        tree.insert(6);
        tree.insert(8);

        assert!(tree.search(&5));
        tree.remove(&5);
        assert!(!tree.search(&5));
        assert!(tree.search(&3));
        assert!(tree.search(&7));
        
        tree.remove(&2);
        assert!(!tree.search(&2));
        assert!(tree.search(&4));
        
        tree.remove(&7);
        assert!(!tree.search(&7));
        assert!(tree.search(&6));
        assert!(tree.search(&8));
    }

    #[test]
    fn test_height() {
        let mut tree = BinarySearchTree::new();
        assert_eq!(tree.height(), 0);
        tree.insert(5);
        assert_eq!(tree.height(), 1);
        tree.insert(3);
        assert_eq!(tree.height(), 2);
        tree.insert(7);
        assert_eq!(tree.height(), 2);
        tree.insert(1);
        assert_eq!(tree.height(), 3);
        tree.insert(9);
        assert_eq!(tree.height(), 3);
    }

    #[test]
    fn test_balancing() {
        // Test left-left case
        let mut tree = BinarySearchTree::new();
        tree.insert(3);
        tree.insert(2);
        tree.insert(1);
        assert_eq!(tree.height(), 2);

        // Test right-right case
        let mut tree = BinarySearchTree::new();
        tree.insert(1);
        tree.insert(2);
        tree.insert(3);
        assert_eq!(tree.height(), 2);

        // Test left-right case
        let mut tree = BinarySearchTree::new();
        tree.insert(3);
        tree.insert(1);
        tree.insert(2);
        assert_eq!(tree.height(), 2);

        // Test right-left case
        let mut tree = BinarySearchTree::new();
        tree.insert(1);
        tree.insert(3);
        tree.insert(2);
        assert_eq!(tree.height(), 2);

        // Test more complex balancing
        let mut tree = BinarySearchTree::new();
        for i in 1..=10 {
            tree.insert(i);
        }
        assert!(tree.height() <= 4);
    }
}