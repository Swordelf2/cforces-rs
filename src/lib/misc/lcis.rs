//! Longest common increasing subsequence

use std::{cmp::max, collections::HashMap};

type Int = i32;

fn lcis(nums1: &[Int], nums2: &[Int]) -> usize {
    let (nums1, nums2, value_limit): (Vec<usize>, Vec<usize>, usize) = {
        let mut nums_all: Vec<Int> =
            nums1.iter().copied().chain(nums2.iter().copied()).collect();
        nums_all.sort_unstable();
        nums_all.dedup();
        let num_2_idx: HashMap<Int, usize> = nums_all
            .iter()
            .enumerate()
            .map(|(idx, &num)| (num, idx))
            .collect();
        (
            nums1.iter().map(|num| num_2_idx[num]).collect(),
            nums2.iter().map(|num| num_2_idx[num]).collect(),
            nums_all.len(),
        )
    };

    let mut max_ans: usize = 0;

    let mut column_ans = vec![0; nums2.len()];
    for &row_val in &nums1 {
        let mut cur_ans = 0;

        for j in 0..nums2.len() {
            if nums2[j] < row_val {
                cur_ans = max(cur_ans, column_ans[j]);
            } else if nums2[j] == row_val {
                let ans = cur_ans + 1;
                column_ans[j] = max(column_ans[j], ans);
                max_ans = max(max_ans, ans);
            }
        }
    }

    max_ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_empty() {
        let a: [Int; 0] = [];
        let b: [Int; 0] = [];
        assert_eq!(lcis(&a, &b), 0);
    }

    #[test]
    fn one_empty() {
        let a = [1, 2, 3];
        let b: [Int; 0] = [];
        assert_eq!(lcis(&a, &b), 0);

        let a: [Int; 0] = [];
        let b = [1, 2, 3];
        assert_eq!(lcis(&a, &b), 0);
    }

    #[test]
    fn no_common_elements() {
        let a = [1, 2, 3];
        let b = [4, 5, 6];
        assert_eq!(lcis(&a, &b), 0);
    }

    #[test]
    fn single_common_element() {
        let a = [1, 2, 3];
        let b = [3, 4, 5];
        assert_eq!(lcis(&a, &b), 1);
    }

    #[test]
    fn identical_sequences_strictly_increasing() {
        let a = [1, 2, 3, 4, 5];
        let b = [1, 2, 3, 4, 5];
        assert_eq!(lcis(&a, &b), 5);
    }

    #[test]
    fn identical_sequences_not_strictly_increasing() {
        let a = [1, 2, 2, 3];
        let b = [1, 2, 2, 3];
        // LCIS must be strictly increasing
        assert_eq!(lcis(&a, &b), 3); // e.g., [1, 2, 3]
    }

    #[test]
    fn common_but_not_increasing() {
        let a = [3, 2, 1];
        let b = [3, 2, 1];
        assert_eq!(lcis(&a, &b), 1);
    }

    #[test]
    fn typical_case() {
        let a = [3, 4, 9, 1];
        let b = [5, 3, 8, 9, 10, 2, 1];
        assert_eq!(lcis(&a, &b), 2); // [3, 9]
    }

    #[test]
    fn duplicates_in_inputs() {
        let a = [1, 3, 3, 5];
        let b = [1, 3, 5, 5];
        assert_eq!(lcis(&a, &b), 3); // [1, 3, 5]
    }

    #[test]
    fn negative_numbers() {
        let a = [-3, -2, -1, 0];
        let b = [-4, -3, -2, 0];
        assert_eq!(lcis(&a, &b), 3); // [-3, -2, 0]
    }

    #[test]
    fn interleaved_choices() {
        let a = [1, 4, 2, 3];
        let b = [1, 2, 3, 4];
        assert_eq!(lcis(&a, &b), 3); // [1, 2, 3]
    }

    #[test]
    fn single_element_repeated() {
        let a = [2, 2, 2, 2];
        let b = [2, 2];
        assert_eq!(lcis(&a, &b), 1);
    }

    #[test]
    fn stress_ordering_constraints() {
        let a = [1, 3, 2, 4, 3, 5];
        let b = [1, 2, 3, 4, 5];
        assert_eq!(lcis(&a, &b), 4); // [1, 2, 3, 5] or similar
    }

    #[test]
    #[ignore = "this test is wrong for now"]
    fn large_mixed_case_40_elements() {
        let a: [Int; 40] = [
            -5, 1, 3, 7, 2, 4, 6, 8, 3, 5, 9, 10, 1, 2, 11, 3, 12, 4, 5, 13, 0, 6, 14, 7,
            15, 8, 9, 16, 10, 11, 17, 12, 13, 18, 14, 15, 19, 16, 17, 20,
        ];

        let b: [Int; 40] = [
            -10, -5, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
            19, 20, 21, 22, 3, 5, 7, 9, 11, 13, 15, 17, 19, 2, 4, 6, 8, 10, 12,
        ];

        // One valid LCIS is:
        // [-5, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
        //  11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
        //
        // Length = 21
        assert_eq!(lcis(&a, &b), 21);
    }

    use proptest::prelude::*;

    use super::Int;

    fn lcis_oracle_cubic(a: &[Int], b: &[Int]) -> usize {
        let n = a.len();
        let m = b.len();

        // dp[i][j] = LCIS length using a[..=i], ending exactly at b[j]
        let mut dp = vec![vec![0usize; m]; n];

        for i in 0..n {
            for j in 0..m {
                if a[i] == b[j] {
                    let mut best = 0;
                    for ii in 0..i {
                        for jj in 0..j {
                            if a[ii] < a[i] && b[jj] < b[j] {
                                best = best.max(dp[ii][jj]);
                            }
                        }
                    }
                    dp[i][j] = best + 1;
                }
            }
        }

        dp.into_iter()
            .flat_map(|row| row.into_iter())
            .max()
            .unwrap_or(0)
    }

    proptest! {
        #[test]
        fn lcis_matches_cubic_oracle(
            a in prop::collection::vec(-20..=100, 0..80),
            b in prop::collection::vec(-20..=100, 0..80),
        ) {
            let fast = lcis(&a, &b);
            let slow = lcis_oracle_cubic(&a, &b);
            prop_assert_eq!(fast, slow);
        }
    }
}
