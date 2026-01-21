/// Reduces a vector of distinct ints to a permutation.
#[must_use]
pub fn reduced_perm(v: &[usize]) -> Vec<usize> {
    use std::collections::HashMap;
    let mut v_sorted = v.to_vec();
    v_sorted.sort_unstable();
    let mut inv_map = HashMap::<usize, usize>::with_capacity(v.len());
    for (i, &elem) in v_sorted.iter().enumerate() {
        inv_map.insert(elem, i);
    }
    v.iter().map(|elem| inv_map[elem]).collect()
}

#[must_use]
pub fn inv_perm(v: &[usize]) -> Vec<usize> {
    let mut inv = vec![0; v.len()];
    for i in 0..v.len() {
        inv[v[i]] = i;
    }
    inv
}

/// NOT TESTED
pub fn inversions_odd(v: &mut [usize]) -> bool {
    let mut inv = inv_perm(v);
    let mut ans = 0;
    for i in 0..v.len() {
        let pos = inv[i];
        if pos != i {
            let vi = v[i];
            let vpos = v[pos];
            v.swap(i, pos);
            inv[vi] = pos;
            inv[vpos] = i;
            ans += 1;
        }
    }

    ans % 2 == 1
}

#[cfg(test)]
mod tests {

    #[test]
    fn reduced_perm() {
        use super::reduced_perm;
        assert_eq!(reduced_perm(&[1, 2, 4]), [0, 1, 2]);
        assert_eq!(reduced_perm(&[1, 2, 3]), [0, 1, 2]);
        assert_eq!(reduced_perm(&[5, 10000, 1]), [1, 2, 0]);
        assert_eq!(reduced_perm(&[5, 6, 7, 8, 9, 1]), [1, 2, 3, 4, 5, 0]);
        assert_eq!(reduced_perm(&[100]), [0]);
        assert_eq!(reduced_perm(&[19, 18]), [1, 0]);
        assert_eq!(reduced_perm(&[18, 3, 14, 17, 0]), [4, 1, 2, 3, 0]);
    }

    #[test]
    fn inv_perm() {
        use super::inv_perm;
        assert_eq!(inv_perm(&[0, 1, 2]), [0, 1, 2]);
        assert_eq!(inv_perm(&[1, 0]), [1, 0]);
        assert_eq!(inv_perm(&[2, 1, 0]), [2, 1, 0]);
        assert_eq!(inv_perm(&[2, 0, 1]), [1, 2, 0]);
    }
}
