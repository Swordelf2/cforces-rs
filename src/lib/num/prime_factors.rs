#![allow(clippy::module_name_repetitions)]

type Uint = u64;

// Struct, that takes some time (`O(nlogn)` where `n == max_val`) to init,
// and then returns biggest prime divisors for any number in
// `[0..max_value]`.
pub struct PrimeFactors {
    /// Mapping from a value to its largest prime factor.
    factor: Vec<Uint>,
}

#[allow(clippy::cast_possible_truncation)]
impl PrimeFactors {
    /// # Panics
    /// Panics if `max_val` is too large.
    #[must_use]
    pub fn new(max_val: Uint) -> Self {
        assert!(max_val <= 1_000_000);
        let mut prime_factors: Vec<Uint> = vec![0; (max_val + 1) as usize];
        prime_factors[0] = 0;
        prime_factors[1] = 1;
        for start in std::iter::once(2).chain((3..=max_val).step_by(2)) {
            if prime_factors[start as usize] != 0 {
                continue;
            }
            prime_factors[start as usize] = start;
            let mut val = start * 2;
            while val <= max_val {
                prime_factors[val as usize] = start;
                val += start;
            }
        }
        Self {
            factor: prime_factors,
        }
    }

    /// Returns the greatest prime factor of `val`.
    #[must_use]
    pub fn prime_factor(&self, val: Uint) -> Uint {
        self.factor[val as usize]
    }

    /// Returns an iterator over pairs of `(cnt, prime_div)`:
    /// prime factors of `val` in descending order and their count in the factorization of `val`.
    ///
    /// # Panics
    /// Panics if `val` == 0.
    #[must_use]
    pub fn prime_factors(&self, val: Uint) -> PrimeFactorsIter {
        assert!(val > 0);
        PrimeFactorsIter {
            prime_factors: self,
            cur_val: val,
        }
    }

    /// Returns whether the given `val` is a prime number.
    #[must_use]
    pub fn is_prime(&self, val: Uint) -> bool {
        val >= 2 && self.factor[val as usize] == val
    }
}

pub struct PrimeFactorsIter<'a> {
    prime_factors: &'a PrimeFactors,
    cur_val: Uint,
}

#[allow(clippy::cast_possible_truncation)]
impl<'a> Iterator for PrimeFactorsIter<'a> {
    type Item = (usize, Uint);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_val <= 1 {
            return None;
        }

        let mut prime = self.prime_factors.factor[self.cur_val as usize];
        let mut cnt = 0;
        loop {
            cnt += 1;
            self.cur_val /= prime;
            let new_prime = self.prime_factors.factor[self.cur_val as usize];
            if new_prime != prime {
                break;
            }
            prime = new_prime;
        }

        Some((cnt, prime))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::bool_assert_comparison)]
    fn test_prime_factors() {
        let pf = PrimeFactors::new(1_000_000);

        assert_eq!(pf.prime_factor(0), 0);
        assert_eq!(pf.prime_factor(1), 1);
        assert_eq!(pf.prime_factor(2), 2);
        assert_eq!(pf.prime_factor(3), 3);
        assert_eq!(pf.prime_factor(4), 2);
        assert_eq!(pf.prime_factor(5), 5);
        assert_eq!(pf.prime_factor(6), 3);
        assert_eq!(pf.prime_factor(18), 3);
        assert_eq!(pf.prime_factor(121), 11);

        assert_eq!(pf.is_prime(2), true);
        assert_eq!(pf.is_prime(3), true);
        assert_eq!(pf.is_prime(5), true);
        assert_eq!(pf.is_prime(7), true);
        assert_eq!(pf.is_prime(11), true);
        assert_eq!(pf.is_prime(1741), true);
        assert_eq!(pf.is_prime(6301), true);
        assert_eq!(pf.is_prime(7919), true);
        assert_eq!(pf.is_prime(7800), false);
        assert_eq!(pf.is_prime(1_000_000), false);
        assert_eq!(pf.is_prime(999_999), false);

        assert_eq!(
            pf.prime_factors(2 * 2 * 2 * 2 * 5 * 5 * 7 * 11 * 19)
                .collect::<Vec<(usize, Uint)>>(),
            &[(1, 19), (1, 11), (1, 7), (2, 5), (4, 2)]
        );
        assert_eq!(
            pf.prime_factors(2).collect::<Vec<(usize, Uint)>>(),
            &[(1, 2)]
        );
        assert_eq!(
            pf.prime_factors(5).collect::<Vec<(usize, Uint)>>(),
            &[(1, 5)]
        );
        assert_eq!(
            pf.prime_factors(5 * 7).collect::<Vec<(usize, Uint)>>(),
            &[(1, 7), (1, 5)]
        );
    }
}
