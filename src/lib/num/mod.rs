type Uint = u64;

pub mod factors;
pub mod prime_factors;

/// Computes greatest common divisor.
///
/// # Panics
/// Panics if `a <= 0` or `b <= 0`.
#[must_use]
pub fn gcd(mut a: Uint, mut b: Uint) -> Uint {
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    assert!(a > 0);
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(0, 1), 1);
        assert_eq!(gcd(1, 0), 1);
        assert_eq!(gcd(2, 1), 1);
        assert_eq!(gcd(3, 1), 1);
        assert_eq!(gcd(1, 3), 1);
        assert_eq!(gcd(2, 3), 1);
        assert_eq!(gcd(3, 4), 1);
        assert_eq!(gcd(2, 4), 2);
        assert_eq!(gcd(4, 2), 2);
        assert_eq!(gcd(5, 3), 1);
        assert_eq!(gcd(11, 3), 1);
        assert_eq!(gcd(4, 18), 2);
        assert_eq!(gcd(100, 10), 10);
        assert_eq!(gcd(15, 35), 5);
        assert_eq!(gcd(64, 48), 16);
        assert_eq!(gcd(9, 6), 3);
        assert_eq!(gcd(1_000_000_000_000, 1_000_000), 1_000_000);
    }
}
