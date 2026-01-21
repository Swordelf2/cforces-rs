use std::cmp::Ordering;

type Int = i64;

/// Finds partition point based on predicate `p`,
/// assuming `p(rng)` is `[1, 1, ... 1, 0, 0, 0]`.
/// Partition point is the first `0` in the sequence.
/// See `std::slice::partition_point` for more info
#[allow(clippy::match_bool)]
pub fn partition_point(mut rng: std::ops::Range<Int>, p: impl Fn(Int) -> bool) -> Int {
    while rng.start < rng.end {
        let middle = rng.start + (rng.end - rng.start) / 2;
        match p(middle) {
            true => rng.start = middle + 1,
            false => rng.end = middle,
        }
    }
    rng.start
}

pub fn merge<T, I1, I2>(iter1: I1, iter2: I2) -> Vec<T>
where
    T: Ord,
    I1: IntoIterator<Item = T>,
    I2: IntoIterator<Item = T>,
{
    merge_by(iter1, iter2, T::cmp)
}

/// Merges two already-sorted iterators.
pub fn merge_by<T, I1, I2>(
    iter1: I1,
    iter2: I2,
    mut cmp: impl FnMut(&T, &T) -> Ordering,
) -> Vec<T>
where
    I1: IntoIterator<Item = T>,
    I2: IntoIterator<Item = T>,
{
    let mut iter1 = iter1.into_iter().peekable();
    let mut iter2 = iter2.into_iter().peekable();

    let mut ans = Vec::with_capacity(iter1.size_hint().0 + iter2.size_hint().0);

    loop {
        match (iter1.peek(), iter2.peek()) {
            (Some(a), Some(b)) => {
                if cmp(a, b) != Ordering::Greater {
                    ans.push(iter1.next().unwrap());
                } else {
                    ans.push(iter2.next().unwrap());
                }
            }
            (Some(_), None) => {
                ans.extend(iter1);
                break;
            }
            (None, Some(_)) => {
                ans.extend(iter2);
                break;
            }
            (None, None) => break,
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::unreadable_literal)]
    fn test_partition_point() {
        assert_eq!(partition_point(0..1, |x| x < 0), 0);
        assert_eq!(partition_point(0..1, |x| x < 1), 1);
        assert_eq!(partition_point(0..0, |x| x < 0), 0);
        assert_eq!(partition_point(0..0, |_x| true), 0);
        assert_eq!(partition_point(0..0, |_x| false), 0);
        assert_eq!(partition_point(1..1, |_x| true), 1);
        assert_eq!(partition_point(1..1, |_x| false), 1);
        assert_eq!(partition_point(-5..5, |x| x < 0), 0);
        assert_eq!(partition_point(-30..25, |x| x <= 0), 1);
        assert_eq!(partition_point(-10..166, |x| x <= 24), 25);
        assert_eq!(partition_point(-10..166, |x| x < -1000), -10);
        assert_eq!(partition_point(-10..166, |x| x < 1000), 166);
        assert_eq!(partition_point(-5..5, |x| x < 0), 0);
        assert_eq!(
            partition_point(-1_000_000_000_000_000..1_000_000_000_000_000, |x| x
                < 4234987234),
            4234987234
        );
        assert_eq!(
            partition_point(-1_000_000_000_000_000..1_000_000_000_000_000, |x| x
                < 234239487),
            234239487
        );
        assert_eq!(
            partition_point(-1_000_000_000_000_000..1_000_000_000_000_000, |x| x
                < -230498234293847),
            -230498234293847
        );
    }
}
