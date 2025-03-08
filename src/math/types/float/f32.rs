use core::f32;

/// Расширение функциональности для f32
pub trait F32Ext {
    /// Вычисляет комплексный тангенс числа.
    fn ctan(&self) -> (f32, f32);

    /// Вычисляет комплексный гиперболический тангенс числа.
    fn ctanh(&self) -> (f32, f32);
}

impl F32Ext for f32 {
    fn ctan(&self) -> (f32, f32) {
        let x = *self;

        if x == 0.0 {
            return (0.0, 0.0);
        }

        let x2 = 2.0 * x;
        let sin_2x = x2.sin();
        let cos_2x = x2.cos();
        let denominator = cos_2x + 1.0;

        if denominator.abs() < f32::EPSILON {
            return (f32::INFINITY * sin_2x.signum(), 0.0);
        }

        (sin_2x / denominator, 0.0)
    }

    fn ctanh(&self) -> (f32, f32) {
        let x = *self;

        if x == 0.0 {
            return (0.0, 0.0);
        }

        let x2 = 2.0 * x;
        let sinh_2x = x2.sinh();
        let cosh_2x = x2.cosh();

        let denominator = cosh_2x + 1.0;

        if denominator.abs() < f32::EPSILON {
            return (f32::INFINITY * sinh_2x.signum(), 0.0);
        }

        (sinh_2x / denominator, 0.0)
    }
}
