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

/// Converts `val` into vec of bits in the order of
/// ascending significance (i.e. least significant bit is the first)
#[must_use]
pub fn num_to_bits(mut val: Uint) -> Vec<bool> {
    let mut res: Vec<bool> = Vec::new();
    while val > 0 {
        res.push((val % 2) == 1);
        val /= 2;
    }
    res
}

/// Assuming `bits` is a slice of bits in the order of
/// ascending significance, returns the number it represents.
#[must_use]
#[allow(clippy::bool_comparison)]
pub fn bits_to_num(bits: &[bool]) -> Uint {
    let mut val: Uint = 0;
    let mut pow_2 = 1;
    for &bit in bits {
        if bit == true {
            val += pow_2;
        }
        pow_2 = pow_2.saturating_mul(2);
    }
    val
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

    #[test]
    fn test_num_to_bits() {
        assert_eq!(num_to_bits(1), &[true]);
        assert_eq!(num_to_bits(2), &[false, true]);
        assert_eq!(num_to_bits(3), &[true, true]);
        assert_eq!(num_to_bits(4), &[false, false, true]);
        assert_eq!(num_to_bits(5), &[true, false, true]);
        assert_eq!(num_to_bits(13), &[true, false, true, true]);
        assert_eq!(num_to_bits(14), &[false, true, true, true]);
    }

    #[test]
    fn test_bits_to_num() {
        for num in
            (0..=10).chain([16, 18, 34, 123_235_199, 2_251_123, !0u64].iter().copied())
        {
            assert_eq!(num, bits_to_num(&num_to_bits(num)));
        }
    }
}
