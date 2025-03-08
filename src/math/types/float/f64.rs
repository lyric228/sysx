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

        if x == 0.0 {
            return (0.0, 0.0);
        }

        let x2 = 2.0 * x;
        let sin_2x = x2.sin();
        let cos_2x = x2.cos();
        let denominator = cos_2x + 1.0;

        if denominator.abs() < f64::EPSILON {
            return (f64::INFINITY * sin_2x.signum(), 0.0);
        }

        (sin_2x / denominator, 0.0)
    }

    fn ctanh(&self) -> (f64, f64) {
        let x = *self;

        if x == 0.0 {
            return (0.0, 0.0);
        }

        let x2 = 2.0 * x;
        let sinh_2x = x2.sinh();
        let cosh_2x = x2.cosh();

        let denominator = cosh_2x + 1.0;

        if denominator.abs() < f64::EPSILON {
            return (f64::INFINITY * sinh_2x.signum(), 0.0);
        }

        (sinh_2x / denominator, 0.0)
    }
}
