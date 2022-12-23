/// Represents a union of disjoint sets. Each set's elements are arranged in a
/// tree, whose root is the set's representative.
pub struct DisjointSets {
    parent: Vec<usize>,
}

impl DisjointSets {
    /// Initializes disjoint sets containing one element each.
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
        }
    }

    /// Finds the set's representative. Do path compression along the way to make
    /// future queries faster.
    pub fn leader(&mut self, mut u: usize) -> usize {
        loop {
            let v = self.parent[u];
            if u == v {
                break u;
            }
            // Path compression
            self.parent[u] = self.parent[v];
            u = v;
        }
    }

    /// Merges the sets containing u and v into a single set containing their
    /// union. Returns true if u and v were previously in different sets.
    pub fn merge(&mut self, u: usize, v: usize) -> bool {
        let (pu, pv) = (self.leader(u), self.leader(v));
        self.parent[pu] = pv;
        pu != pv
    }

    pub fn same(&mut self, u: usize, v: usize) -> bool {
        self.leader(u) == self.leader(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test_disjoint_sets() {
        {
            let mut dsets = DisjointSets::new(1);
            dsets.merge(0, 0);
            dsets.merge(0, 0);
            dsets.merge(0, 0);
            assert_eq!(dsets.same(0, 0), true);
        }
        {
            const SIZE: usize = 10;
            let mut dsets = DisjointSets::new(SIZE);
            // Make fully connected
            dsets.merge(0, 1);
            dsets.merge(2, 3);
            dsets.merge(8, 9);
            dsets.merge(5, 6);
            dsets.merge(3, 4);
            dsets.merge(7, 8);
            dsets.merge(4, 5);
            dsets.merge(1, 2);
            dsets.merge(6, 7);
            for u in 0..SIZE {
                for v in 0..SIZE {
                    assert_eq!(dsets.same(u, v), true);
                }
            }
        }
        {
            const SIZE: usize = 10;
            let mut dsets = DisjointSets::new(SIZE);
            dsets.merge(0, 1);
            dsets.merge(2, 3);
            dsets.merge(4, 9);
            dsets.merge(2, 9);
            dsets.merge(5, 1);
            assert_eq!(dsets.same(0, 1), true);
            assert_eq!(dsets.same(1, 5), true);
            assert_eq!(dsets.same(0, 5), true);
            assert_eq!(dsets.same(2, 3), true);
            assert_eq!(dsets.same(2, 9), true);
            assert_eq!(dsets.same(3, 9), true);
            assert_eq!(dsets.same(3, 9), true);
            assert_eq!(dsets.same(4, 9), true);
            assert_eq!(dsets.same(4, 9), true);
            assert_eq!(dsets.same(4, 2), true);
            assert_eq!(dsets.same(4, 3), true);
            assert_eq!(dsets.same(0, 4), false);
            assert_eq!(dsets.same(0, 2), false);
            assert_eq!(dsets.same(0, 9), false);
            assert_eq!(dsets.same(1, 4), false);
            for u in 6..=8 {
                for v in (0..SIZE).filter(|&v| u != v) {
                    assert_eq!(dsets.same(u, v), false);
                }
            }
        }
    }
}
