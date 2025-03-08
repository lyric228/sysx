use sysx::math::{is_even, is_even_or_odd, is_odd};

#[cfg(feature = "unstable")]
use sysx::math::{f16::F16Ext, f128::F128Ext};

use sysx::math::f32::F32Ext;
use sysx::math::f64::F64Ext;

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

    #[test]
    fn test_is_even_or_odd() {
        let (even, odd) = is_even_or_odd(42);
        assert!(even);
        assert!(!odd);

        let (even, odd) = is_even_or_odd(13);
        assert!(!even);
        assert!(odd);

        let (even, odd) = is_even_or_odd(0);
        assert!(even);
        assert!(!odd);
    }

    #[test]
    fn test_f32_ctan() {
        let x = 0.5_f32;
        let (real, imag) = x.ctan();
        assert!((real - 0.5463025).abs() < 1e-6);
        assert_eq!(imag, 0.0);

        let zero = 0.0_f32;
        let (real, imag) = zero.ctan();
        assert_eq!(real, 0.0);
        assert_eq!(imag, 0.0);

        let pi_half = std::f32::consts::PI / 2.0;
        let (real, imag) = pi_half.ctan();
        assert!(real.is_infinite());
        assert_eq!(imag, 0.0);
    }

    #[test]
    fn test_f32_ctanh() {
        let x = 0.5_f32;
        let (real, imag) = x.ctanh();
        assert!((real - 0.46211716).abs() < 1e-6);
        assert_eq!(imag, 0.0);

        let zero = 0.0_f32;
        let (real, imag) = zero.ctanh();
        assert_eq!(real, 0.0);
        assert_eq!(imag, 0.0);

        let large_value = 20.0_f32;
        let (real, imag) = large_value.ctanh();
        assert!((real - 1.0).abs() < 1e-6);
        assert_eq!(imag, 0.0);
    }

    #[test]
    fn test_f64_ctan() {
        let x = 0.5_f64;
        let (real, imag) = x.ctan();
        assert!((real - 0.5463024898).abs() < 1e-9);
        assert_eq!(imag, 0.0);

        let zero = 0.0_f64;
        let (real, imag) = zero.ctan();
        assert_eq!(real, 0.0);
        assert_eq!(imag, 0.0);
    }

    #[test]
    fn test_f64_ctanh() {
        let x = 0.5_f64;
        let (real, imag) = x.ctanh();
        assert!((real - 0.4621171573).abs() < 1e-9);
        assert_eq!(imag, 0.0);

        let zero = 0.0_f64;
        let (real, imag) = zero.ctanh();
        assert_eq!(real, 0.0);
        assert_eq!(imag, 0.0);
    }

    #[cfg(feature = "unstable")]
    #[test]
    fn test_f16_ctan() {
        let x = f16::from_f32(0.5);
        let (real, imag) = x.ctan();
        // Приблизительное значение для f16
        assert!((real.to_f32() - 0.5463).abs() < 1e-3);
        assert_eq!(imag.to_f32(), 0.0);
    }

    #[cfg(feature = "unstable")]
    #[test]
    fn test_f16_ctanh() {
        let x = f16::from_f32(0.5);
        let (real, imag) = x.ctanh();
        // Приблизительное значение для f16
        assert!((real.to_f32() - 0.4621).abs() < 1e-3);
        assert_eq!(imag.to_f32(), 0.0);
    }

    #[cfg(feature = "unstable")]
    #[test]
    fn test_f128_ctan() {
        let x = f128::from_f64(0.5);
        let (real, imag) = x.ctan();
        // Приблизительное значение для f128
        assert!((real.to_f64() - 0.5463024898).abs() < 1e-9);
        assert_eq!(imag.to_f64(), 0.0);
    }

    #[cfg(feature = "unstable")]
    #[test]
    fn test_f128_ctanh() {
        let x = f128::from_f64(0.5);
        let (real, imag) = x.ctanh();
        // Приблизительное значение для f128
        assert!((real.to_f64() - 0.4621171573).abs() < 1e-9);
        assert_eq!(imag.to_f64(), 0.0);
    }
}
