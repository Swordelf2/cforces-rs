use std::{
    cmp::{max, min},
    ops::Range,
};

type Int = i64;

/// Associative operation used in the [`SegmentTree`].
fn op(a: Int, b: Int) -> Int {
    a + b
}

/// Zero in respect to the associative operation `op`, i.e.:  
/// \forall x: op(x, ZERO) = op(ZERO, x) = x
const ZERO: Int = 0;

/// Segment Tree.
///
/// Note: Make sure `Int` is as big as you need - likely `i64`.
///
/// # Panics
///
/// May or may not panic if `n`, `range`, `idx` are out of bounds.
pub struct SegmentTree {
    tree: Vec<Int>,
    len: usize,
}

impl SegmentTree {
    /// Returns a segment tree for `n` array elements, assumed to be all zeroes.
    #[must_use]
    pub fn new(n: usize) -> SegmentTree {
        SegmentTree {
            tree: vec![ZERO; 4 * n],
            len: n,
        }
    }

    #[must_use]
    pub fn from_slice(arr: &[Int]) -> SegmentTree {
        let mut st = SegmentTree {
            tree: vec![ZERO; 4 * arr.len()],
            len: arr.len(),
        };
        let root = st.root();
        Self::build_recursive(&mut st.tree, arr, &root);
        st
    }

    #[must_use]
    pub fn compute(&self, range: Range<usize>) -> Int {
        let root = self.root();
        Self::compute_recursive(&self.tree, &root, range)
    }

    pub fn update(&mut self, idx: usize, value: Int) {
        let root = self.root();
        Self::update_recursive(&mut self.tree, &root, idx, value);
    }

    fn build_recursive(tree: &mut [Int], arr: &[Int], node: &Node) {
        match node.range.len() {
            0 => (),
            1 => {
                tree[node.id] = arr[node.range.start];
            }
            _ => {
                let (left_node, right_node) = node.children();
                Self::build_recursive(tree, arr, &left_node);
                Self::build_recursive(tree, arr, &right_node);
                tree[node.id] = op(tree[left_node.id], tree[right_node.id]);
            }
        }
    }

    fn compute_recursive(tree: &[Int], node: &Node, compute_range: Range<usize>) -> Int {
        if compute_range.is_empty() {
            return ZERO;
        }
        if node.range == compute_range {
            return tree[node.id];
        }
        let (left_node, right_node) = node.children();
        let left_compute_range =
            compute_range.start..min(compute_range.end, left_node.range.end);
        let right_compute_range =
            max(compute_range.start, right_node.range.start)..compute_range.end;
        let left_value = Self::compute_recursive(tree, &left_node, left_compute_range);
        let right_value = Self::compute_recursive(tree, &right_node, right_compute_range);
        op(left_value, right_value)
    }

    fn update_recursive(tree: &mut [Int], node: &Node, idx: usize, value: Int) {
        debug_assert!(!node.range.is_empty());
        if node.range.len() == 1 {
            tree[node.id] = value;
            return;
        }
        let (left_node, right_node) = node.children();
        if left_node.range.contains(&idx) {
            Self::update_recursive(tree, &left_node, idx, value);
        } else {
            Self::update_recursive(tree, &right_node, idx, value);
        }
        tree[node.id] = op(tree[left_node.id], tree[right_node.id]);
    }

    fn root(&self) -> Node {
        Node {
            id: 1,
            range: 0..self.len,
        }
    }
}

#[derive(Debug)]
struct Node {
    id: usize,
    range: Range<usize>,
}

impl Node {
    fn children(&self) -> (Node, Node) {
        let middle = self.range.start + self.range.len() / 2;
        (
            Node {
                id: self.id * 2,
                range: self.range.start..middle,
            },
            Node {
                id: self.id * 2 + 1,
                range: middle..self.range.end,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn update_and_check(arr: &mut [Int], st: &mut SegmentTree, idx: usize, value: Int) {
        arr[idx] = value;
        st.update(idx, value);
        check_st(arr, st);
    }

    #[test]
    fn test() {
        let mut arr = [0, 2, -5, 17, -100, 44];
        let mut st = SegmentTree::from_slice(&arr);
        check_st(&arr, &st);
        update_and_check(&mut arr, &mut st, 0, 15);
        update_and_check(&mut arr, &mut st, 2, -123);
        update_and_check(&mut arr, &mut st, 1, 38);
        update_and_check(&mut arr, &mut st, 5, -11);
        update_and_check(&mut arr, &mut st, 2, 11111);
        update_and_check(&mut arr, &mut st, 3, 8);
        update_and_check(&mut arr, &mut st, 4, -5);
    }

    #[test]
    fn test_trivial() {
        const ST_LEN: usize = 5;
        let arr = [0; ST_LEN];
        let st = SegmentTree::new(ST_LEN);
        check_st(&arr, &st);
        let st = SegmentTree::new(1);
        check_st(&arr[..1], &st);
    }

    #[allow(clippy::reversed_empty_ranges)]
    fn check_st(arr: &[Int], st: &SegmentTree) {
        for i in 0..arr.len() {
            for j in i..arr.len() {
                assert_eq!(
                    st.compute(i..j),
                    arr[i..j].iter().sum(),
                    "i = {}, j = {}",
                    i,
                    j
                );
            }
        }
        assert_eq!(st.compute(1..0), 0);
    }
}
