use sysx::math::{is_even, is_odd};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even_odd() {
        assert!(is_even(42));
        assert!(!is_even(13));
        assert!(is_even(-4));
        assert!(!is_even(-3));
        assert!(is_even(0));
    }

    #[test]
    fn test_is_odd() {
        assert!(is_odd(1));
        assert!(!is_odd(2));
        assert!(is_odd(3));
        assert!(!is_odd(4));
        assert!(is_odd(1_u32));
        assert!(!is_odd(2_u32));
    }

    #[test]
    fn test_all_types() {
        assert!(is_even(2_u8));
        assert!(is_even(4_i16));
        assert!(is_even(6_u32));
        assert!(is_even(-8_i64));
        assert!(is_even(10_usize));
    }
}
