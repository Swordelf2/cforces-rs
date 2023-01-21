type Uint = u64;

/// Stores all factors of all numbers up to `max_val`.
pub struct Factors {
    num_to_factors: Vec<Vec<Uint>>,
}

#[allow(clippy::cast_possible_truncation)]
impl Factors {
    /// # Panics
    /// Panics if `max_val` is too large.
    #[must_use]
    pub fn new(max_val: Uint) -> Self {
        assert!(max_val <= 1_000_000);
        let mut num_to_factors = vec![Vec::new(); (max_val + 1) as usize];
        for factor in 1..=max_val {
            for val in (factor..=max_val).step_by(factor as usize) {
                num_to_factors[val as usize].push(factor);
            }
        }
        Factors { num_to_factors }
    }

    /// Returns all factors of `val` in ascending order.
    #[must_use]
    pub fn factors(&self, val: Uint) -> &[Uint] {
        &self.num_to_factors[val as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factors() {
        let f = Factors::new(500_000);
        assert_eq!(f.factors(0), &[]);
        assert_eq!(f.factors(1), &[1]);
        assert_eq!(f.factors(2), &[1, 2]);
        assert_eq!(f.factors(3), &[1, 3]);
        assert_eq!(f.factors(4), &[1, 2, 4]);
        assert_eq!(f.factors(5), &[1, 5]);
        assert_eq!(f.factors(6), &[1, 2, 3, 6]);
        assert_eq!(f.factors(7), &[1, 7]);
        assert_eq!(f.factors(8), &[1, 2, 4, 8]);
        assert_eq!(f.factors(9), &[1, 3, 9]);
        assert_eq!(f.factors(10), &[1, 2, 5, 10]);
        assert_eq!(f.factors(12), &[1, 2, 3, 4, 6, 12]);
        assert_eq!(
            f.factors(988),
            &[1, 2, 4, 13, 19, 26, 38, 52, 76, 247, 494, 988]
        );
    }
}
