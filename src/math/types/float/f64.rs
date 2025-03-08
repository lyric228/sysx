use core::f64;

/// Расширение функциональности для f64
pub trait F64Ext {
    /// Вычисляет комплексный тангенс числа.
    fn ctan(&self) -> (f64, f64);

    /// Вычисляет комплексный гиперболический тангенс числа.
    fn ctanh(&self) -> (f64, f64);
}

impl F64Ext for f64 {
    fn ctan(&self) -> (f64, f64) {
        let x = *self;
        let sin_2x = (2.0 * x).sin();
        let cos_2x = (2.0 * x).cos();
        let denominator = cos_2x;

        if denominator.abs() < f64::EPSILON {
            return (f64::INFINITY * sin_2x.signum(), 0.0);
        }

        (sin_2x / denominator, 0.0)
    }

    fn ctanh(&self) -> (f64, f64) {
        let x = *self;
        let sinh_2x = (2.0 * x).sinh();
        let cosh_2x = (2.0 * x).cosh();
        let denominator = cosh_2x;

        if denominator.abs() < f64::EPSILON {
            return (f64::INFINITY * sinh_2x.signum(), 0.0);
        }

        (sinh_2x / denominator, 0.0)
    }
}
