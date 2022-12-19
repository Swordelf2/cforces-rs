pub fn log2(value: i32) -> i32 {
    assert!(value > 0);
    (32 - (value).leading_zeros() - 1) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log2() {
        assert_eq!(log2(1), 0);
        assert_eq!(log2(2), 1);
        assert_eq!(log2(3), 1);
        assert_eq!(log2(4), 2);
        assert_eq!(log2(5), 2);
        assert_eq!(log2(6), 2);
        assert_eq!(log2(7), 2);
        assert_eq!(log2(8), 3);
        assert_eq!(log2(9), 3);
        assert_eq!(log2(15), 3);
        assert_eq!(log2(16), 4);
        assert_eq!(log2(17), 4);
        assert_eq!(log2(1023), 9);
        assert_eq!(log2(1024), 10);
        assert_eq!(log2(4000), 11);
        assert_eq!(log2(4099), 12);
    }
}