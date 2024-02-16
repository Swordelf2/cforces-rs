type Uint = u64;
type Int = i64;

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

/// Computes `val^(-1)` modulo `m`, which must be coprime with `val`.
///
/// # Panics
/// Panics if `val` is not coprime with `m`.
#[must_use]
#[allow(clippy::cast_sign_loss)]
pub fn inverse(val: Uint, m: Uint) -> Uint {
    let (gcd, k1, _) = gcd_ext(val, m);
    assert_eq!(gcd, 1, "given val = {val} is not coprime with m = {m}");
    if k1 >= 0 {
        (k1 as Uint) % m
    } else {
        let res = (-k1) as Uint % m;
        if res == 0 {
            0
        } else {
            m - res
        }
    }
}

#[must_use]
/// Computes gcd and `k1, k2`, such that `k1 * a + k2 * b = gcd(a, b)`
/// Returns `(gcd, k1, k2)`.
///
/// # Panics
/// Panics only in very weird cases.
pub fn gcd_ext(a: Uint, b: Uint) -> (Uint, Int, Int) {
    let mut r = (a, b);
    let mut k1: (Int, Int) = (1, 0);
    let mut k2: (Int, Int) = (0, 1);
    while r.1 != 0 {
        use std::convert::TryInto;
        let quot = r.0 / r.1;
        r = (r.1, r.0 - quot * r.1);
        let quot: Int = quot.try_into().unwrap();
        k1 = (k1.1, k1.0 - quot * k1.1);
        k2 = (k2.1, k2.0 - quot * k2.1);
    }
    (r.0, k1.0, k2.0)
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

    #[test]
    fn test_inverse_simple() {
        const P: u64 = 1_000_000_000 + 7;
        assert_eq!(inverse(3, P) * 3 % P, 1);
        assert_eq!(inverse(18, P) * 18 % P, 1);
        assert_eq!(inverse(19, P) * 19 % P, 1);
        assert_eq!(inverse(20, P) * 20 % P, 1);

        assert_eq!(inverse(3, P) * 6 % P, 2);
        assert_eq!(inverse(2, P) * 6 % P, 3);
        assert_eq!(inverse(4, P) * 20 % P, 5);
        assert_eq!(inverse(74, P) * 148 % P, 2);
        assert_eq!(inverse(8, P) * 280 % P, 35);
    }

    #[test]
    fn test_inverse_extensive() {
        for num in [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 133, 150_982, 123, 14214, 1522, 231_245, 1333,
            43231, 523_412, 999, 19239, 987,
        ] {
            for m in [5, 7, 11, 13, 19, 23, 998_244_353, 1_000_000_000 + 7] {
                if num >= m {
                    continue;
                }
                assert_eq!(inverse(num, m) * num % m, 1);
            }
        }
    }
}
