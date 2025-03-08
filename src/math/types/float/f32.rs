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
        let sin_2x = (2.0 * x).sin();
        let cos_2x = (2.0 * x).cos();
        let denominator = cos_2x;
        
        if denominator.abs() < f32::EPSILON {
            return (f32::INFINITY * sin_2x.signum(), 0.0);
        }
        
        (sin_2x / denominator, 0.0)
    }
    
    fn ctanh(&self) -> (f32, f32) {
        let x = *self;
        let sinh_2x = (2.0 * x).sinh();
        let cosh_2x = (2.0 * x).cosh();
        let denominator = cosh_2x;
        
        if denominator.abs() < f32::EPSILON {
            return (f32::INFINITY * sinh_2x.signum(), 0.0);
        }
        
        (sinh_2x / denominator, 0.0)
    }
}
