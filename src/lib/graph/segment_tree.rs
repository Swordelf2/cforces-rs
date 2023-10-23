use std::{
    cmp::{max, min},
    ops::Range,
};

type Int = i32;

/// Associative operation used in the SegmentTree.
fn op(a: Int, b: Int) -> Int {
    a + b
}

/// Zero in respect to the associative operation `op`, i.e.:  
/// \forall x op(x, ZERO) = op(ZERO, x) = x
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
        let mut tree = vec![ZERO; 4 * arr.len()];
        Self::build_recursive(&mut tree, arr, 1, 0..arr.len());
        SegmentTree {
            tree,
            len: arr.len(),
        }
    }

    #[must_use]
    pub fn compute(&self, range: Range<usize>) -> Int {
        Self::compute_recursive(&self.tree, 1, 0..self.len, range)
    }

    pub fn update(&mut self, idx: usize, value: Int) {
        Self::update_recursive(&mut self.tree, 1, 0..self.len, idx, value);
    }

    fn build_recursive(
        tree: &mut [Int],
        arr: &[Int],
        node: usize,
        node_range: Range<usize>,
    ) {
        match node_range.len() {
            0 => (),
            1 => {
                tree[node] = arr[node_range.start];
            }
            _ => {
                let (left, right) = Self::children(node);
                let (left_range, right_range) = Self::split_range(node_range);
                Self::build_recursive(tree, arr, left, left_range);
                Self::build_recursive(tree, arr, left, right_range);
                tree[node] = op(tree[left], tree[right]);
            }
        }
    }

    fn compute_recursive(
        tree: &[Int],
        node: usize,
        node_range: Range<usize>,
        compute_range: Range<usize>,
    ) -> Int {
        if compute_range.is_empty() {
            return ZERO;
        }
        if node_range == compute_range {
            return tree[node];
        }
        let (left_node, right_node) = Self::children(node);
        let (left_node_range, right_node_range) = Self::split_range(node_range);
        let left_compute_range =
            compute_range.start..min(compute_range.end, left_node_range.end);
        let right_compute_range =
            max(compute_range.start, right_node_range.start)..compute_range.end;
        let left_value =
            Self::compute_recursive(tree, left_node, left_node_range, left_compute_range);
        let right_value = Self::compute_recursive(
            tree,
            right_node,
            right_node_range,
            right_compute_range,
        );
        op(left_value, right_value)
    }

    fn update_recursive(
        tree: &mut [Int],
        node: usize,
        node_range: Range<usize>,
        idx: usize,
        value: Int,
    ) {
        debug_assert!(!node_range.is_empty());
        if node_range.len() == 1 {
            tree[node_range.start] = value;
            return;
        }
        let (left_node, right_node) = Self::children(node);
        let (left_range, right_range) = Self::split_range(node_range);
        if left_range.contains(&idx) {
            Self::update_recursive(tree, left_node, left_range, idx, value);
        } else {
            Self::update_recursive(tree, right_node, right_range, idx, value);
        }
        tree[node] = op(tree[left_node], tree[right_node]);
    }

    fn children(node: usize) -> (usize, usize) {
        (node * 2, node * 2 + 1)
    }

    fn split_range(range: Range<usize>) -> (Range<usize>, Range<usize>) {
        let middle = range.start + range.len() / 2;
        (range.start..middle, middle..range.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
