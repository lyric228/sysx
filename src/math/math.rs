use core::ops::BitAnd;

/// Проверяет, является ли число чётным.
pub fn is_even<T>(num: T) -> bool
where
    T: BitAnd<Output = T> + From<u8> + PartialEq,
{
    let one = T::from(1u8);
    num & one == T::from(0u8)
}

/// Проверяет, является ли число нечётным.
pub fn is_odd<T>(n: T) -> bool
where
    T: BitAnd<Output = T> + PartialEq + From<u8>,
{
    n & T::from(1u8) != T::from(0u8)
}
