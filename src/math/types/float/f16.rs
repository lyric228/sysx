use core::f16;

/// Расширение функциональности для f16
pub trait F16Ext {
    /// Вычисляет комплексный тангенс числа.
    fn ctan(&self) -> (f16, f16);
    
    /// Вычисляет комплексный гиперболический тангенс числа.
    fn ctanh(&self) -> (f16, f16);
}

impl F16Ext for f16 {
    fn ctan(&self) -> (f16, f16) {
        let x = *self;
        let sin_2x = (2.0 * x).sin();
        let cos_2x = (2.0 * x).cos();
        let denominator = cos_2x;
        
        if denominator.abs() < f16::EPSILON {
            return (f16::INFINITY * sin_2x.signum(), f16::from_f32(0.0));
        }
        
        (sin_2x / denominator, f16::from_f32(0.0))
    }
    
    fn ctanh(&self) -> (f16, f16) {
        let x = *self;
        let sinh_2x = (2.0 * x).sinh();
        let cosh_2x = (2.0 * x).cosh();
        let denominator = cosh_2x;
        
        if denominator.abs() < f16::EPSILON {
            return (f16::INFINITY * sinh_2x.signum(), f16::from_f32(0.0));
        }
        
        (sinh_2x / denominator, f16::from_f32(0.0))
    }
}
