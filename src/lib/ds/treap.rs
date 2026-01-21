use std::cmp::Ordering;

/// Cartesian tree.
/// BST on keys K, min-heap on weights W.
#[derive(Debug)]
pub struct Treap<K, V, W> {
    root: NodePtr<K, V, W>,
}

type NodePtr<K, V, W> = Option<Box<Node<K, V, W>>>;

impl<K: Ord, V, W: Ord> Treap<K, V, W> {
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Returns whether a value was inserted
    pub fn insert(&mut self, key: K, value: V, weight: W) -> Option<V> {
        let (left_node, key_node, right_node) = Node::split_by(self.root.take(), &key);
        let new_node = Box::new(Node::new(key, value, weight));
        self.root = Node::merge(Node::merge(left_node, Some(new_node)), right_node);
        key_node.map(|n| n.value)
    }

    pub fn remove(&mut self, key: &K) -> Option<(K, V)> {
        let (left_node, key_node, right_node) = Node::split_by(self.root.take(), key);
        self.root = Node::merge(left_node, right_node);
        key_node.map(|n| (n.key, n.value))
    }

    #[must_use]
    pub fn find_greater_or_equal(&self, search_key: &K) -> Option<(&K, &V)> {
        Node::find_greater_or_equal(self.root.as_deref(), search_key)
            .map(|n| (&n.key, &n.value))
    }

    #[must_use]
    /// Returns how many items are in the treap with keys less than `search_key`
    pub fn count_less(&self, search_key: &K) -> usize {
        Node::count_less(self.root.as_deref(), search_key)
    }
}

#[derive(Debug)]
struct Node<K, V, W> {
    key: K,
    value: V,
    weight: W,
    left: NodePtr<K, V, W>,
    right: NodePtr<K, V, W>,
    size: usize,
}

impl<K: Ord, V, W: Ord> Node<K, V, W> {
    pub fn new(key: K, value: V, weight: W) -> Self {
        Self {
            key,
            value,
            weight,
            left: None,
            right: None,
            size: 1,
        }
    }
    /// Splits this treap into 3 treaps: `("< k", "= k", "> k")`.
    #[allow(clippy::type_complexity)]
    fn split_by(
        node: Option<Box<Node<K, V, W>>>,
        split_key: &K,
    ) -> (NodePtr<K, V, W>, NodePtr<K, V, W>, NodePtr<K, V, W>) {
        let Some(mut node) = node else {
            return (None, None, None);
        };
        match split_key.cmp(&node.key) {
            Ordering::Equal => {
                let (left_tree, right_tree) = (node.left.take(), node.right.take());
                node.update_size();
                (left_tree, Some(node), right_tree)
            }
            Ordering::Less => {
                let (left_tree, split_node, right_tree) =
                    Node::split_by(node.left.take(), split_key);
                node.left = right_tree;
                node.update_size();
                (left_tree, split_node, Some(node))
            }
            Ordering::Greater => {
                let (left_tree, split_node, right_tree) =
                    Node::split_by(node.right.take(), split_key);
                node.right = left_tree;
                node.update_size();
                (Some(node), split_node, right_tree)
            }
        }
    }

    fn merge(
        left_node: NodePtr<K, V, W>,
        right_node: NodePtr<K, V, W>,
    ) -> NodePtr<K, V, W> {
        let Some(mut left_node) = left_node else {
            return right_node;
        };
        let Some(mut right_node) = right_node else {
            return Some(left_node);
        };
        match left_node.weight.cmp(&right_node.weight) {
            Ordering::Less => {
                left_node.right = Node::merge(left_node.right.take(), Some(right_node));
                left_node.update_size();
                Some(left_node)
            }
            Ordering::Equal | Ordering::Greater => {
                right_node.left = Node::merge(Some(left_node), right_node.left.take());
                right_node.update_size();
                Some(right_node)
            }
        }
    }

    fn find_greater_or_equal<'n>(
        node: Option<&'n Node<K, V, W>>,
        search_key: &K,
    ) -> Option<&'n Node<K, V, W>> {
        let Some(node) = node else {
            return None;
        };
        match search_key.cmp(&node.key) {
            Ordering::Equal => Some(node),
            Ordering::Less => {
                Node::find_greater_or_equal(node.left.as_deref(), search_key)
                    .or(Some(node))
            }
            Ordering::Greater => {
                Node::find_greater_or_equal(node.right.as_deref(), search_key)
            }
        }
    }

    fn size(node: Option<&Node<K, V, W>>) -> usize {
        node.map_or(0, |n| n.size)
    }

    fn update_size(&mut self) {
        self.size =
            1 + Node::size(self.left.as_deref()) + Node::size(self.right.as_deref());
    }

    fn count_less(node: Option<&Node<K, V, W>>, search_key: &K) -> usize {
        let Some(node) = node else { return 0 };
        match search_key.cmp(&node.key) {
            Ordering::Equal => Node::size(node.left.as_deref()),
            Ordering::Less => Node::count_less(node.left.as_deref(), search_key),
            Ordering::Greater => {
                Node::size(Some(node)) - Node::size(node.right.as_deref())
                    + Node::count_less(node.right.as_deref(), search_key)
            }
        }
    }
}

const RNG_SEED: u32 = 1_415_811;

const RNG_MULTIPLIER: u32 = 22_695_477;
const RNG_INCREMENT: u32 = 1;
/// Treap with random weights
#[derive(Debug)]
pub struct RandomTreap<K, V> {
    treap: Treap<K, V, u32>,
    rng_state: u32,
}

impl<K: Ord, V> RandomTreap<K, V> {
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            treap: Treap::new(),
            rng_state: RNG_SEED,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let weight = self.next_rand();
        self.treap.insert(key, value, weight)
    }

    fn next_rand(&mut self) -> u32 {
        self.rng_state = self
            .rng_state
            .wrapping_mul(RNG_MULTIPLIER)
            .wrapping_add(RNG_INCREMENT);
        self.rng_state
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut treap = Treap::<i64, u32, usize>::new();
        treap.insert(2, 1, 1);
        treap.insert(3, 11, 100);
        treap.insert(14, 9, 150);
        treap.insert(77777, 2, 7812);
        treap.insert(7, 8, 124);
        treap.insert(9, 4, 1111);
        treap.insert(833, 2, 12);
        treap.insert(-15, 2, 123);

        assert_eq!(treap.find_greater_or_equal(&2), (Some((&2, &1))));
        assert_eq!(treap.find_greater_or_equal(&3), (Some((&3, &11))));
        assert_eq!(treap.find_greater_or_equal(&14), (Some((&14, &9))));
        assert_eq!(treap.find_greater_or_equal(&77777), (Some((&77777, &2))));
        assert_eq!(treap.find_greater_or_equal(&7), (Some((&7, &8))));
        assert_eq!(treap.find_greater_or_equal(&9), (Some((&9, &4))));
        assert_eq!(treap.find_greater_or_equal(&833), (Some((&833, &2))));
        assert_eq!(treap.find_greater_or_equal(&-15), (Some((&-15, &2))));

        assert_eq!(treap.find_greater_or_equal(&-16), (Some((&-15, &2))));
        assert_eq!(treap.find_greater_or_equal(&-117), (Some((&-15, &2))));
        assert_eq!(treap.find_greater_or_equal(&-18), (Some((&-15, &2))));

        assert_eq!(treap.find_greater_or_equal(&0), (Some((&2, &1))));
        assert_eq!(treap.find_greater_or_equal(&1), (Some((&2, &1))));

        assert_eq!(treap.find_greater_or_equal(&600), (Some((&833, &2))));

        assert_eq!(treap.find_greater_or_equal(&8), (Some((&9, &4))));

        assert_eq!(treap.find_greater_or_equal(&10), (Some((&14, &9))));
        assert_eq!(treap.find_greater_or_equal(&11), (Some((&14, &9))));
        assert_eq!(treap.find_greater_or_equal(&77778), None);
        assert_eq!(treap.find_greater_or_equal(&77779), None);
        assert_eq!(treap.find_greater_or_equal(&999_999_999), None);

        assert!(treap.remove(&-15).is_some());
        assert_eq!(treap.remove(&-14), None);
        assert!(treap.remove(&3).is_some());
        // Set is [2, 7, 9, 14, 833, 77777]
        assert_eq!(treap.count_less(&4), 1);
        assert_eq!(treap.count_less(&9), 2);
        assert_eq!(treap.count_less(&833), 4);
        assert_eq!(treap.count_less(&834), 5);
        assert_eq!(treap.count_less(&1_000_000), 6);
        assert_eq!(treap.count_less(&0), 0);
        assert_eq!(treap.count_less(&2), 0);
    }
}
