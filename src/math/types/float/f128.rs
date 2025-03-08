use core::f128;

/// Расширение функциональности для f128
pub trait F128Ext {
    /// Вычисляет комплексный тангенс числа.
    fn ctan(&self) -> (f128, f128);

    /// Вычисляет комплексный гиперболический тангенс числа.
    fn ctanh(&self) -> (f128, f128);
}

impl F128Ext for f128 {
    fn ctan(&self) -> (f128, f128) {
        let x = *self;
        let sin_2x = (2.0 * x).sin();
        let cos_2x = (2.0 * x).cos();
        let denominator = cos_2x;

        if denominator.abs() < f128::EPSILON {
            return (f128::INFINITY * sin_2x.signum(), f128::from_f64(0.0));
        }

        (sin_2x / denominator, f128::from_f64(0.0))
    }

    fn ctanh(&self) -> (f128, f128) {
        let x = *self;
        let sinh_2x = (2.0 * x).sinh();
        let cosh_2x = (2.0 * x).cosh();
        let denominator = cosh_2x;

        if denominator.abs() < f128::EPSILON {
            return (f128::INFINITY * sinh_2x.signum(), f128::from_f64(0.0));
        }

        (sinh_2x / denominator, f128::from_f64(0.0))
    }
}
