/// Проверяет, является ли число чётным.
///
/// Использует побитовую операцию AND для эффективной проверки, равен ли младший бит 0.
pub fn is_even<T>(num: T) -> bool
where
    T: core::ops::BitAnd<Output = T> + From<u8> + PartialEq,
{
    (num & T::from(1u8)) == T::from(0u8)
}

/// Проверяет, является ли число нечётным.
///
/// Использует побитовую операцию AND для эффективной проверки, равен ли младший бит 1.
pub fn is_odd<T>(num: T) -> bool
where
    T: core::ops::BitAnd<Output = T> + From<u8> + PartialEq,
{
    (num & T::from(1u8)) != T::from(0u8)
}

/// Альтернативная реализация, которая избегает лишних вычислений.
/// Используйте эту функцию, когда нужно проверить оба условия.
pub fn is_even_or_odd<T>(num: T) -> (bool, bool)
where
    T: core::ops::BitAnd<Output = T> + From<u8> + PartialEq + Copy,
{
    let is_even = (num & T::from(1u8)) == T::from(0u8);
    (is_even, !is_even)
}
