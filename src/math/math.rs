/// Checks if a number is even using a bitwise AND operation.
/// Generic over types `T` supporting `BitAnd`, `From<u8>`, `PartialEq`.
pub fn is_even<T>(num: T) -> bool
where
    T: core::ops::BitAnd<Output = T> + From<u8> + PartialEq,
{
    (num & T::from(1u8)) == T::from(0u8)
}

/// Checks if a number is odd using a bitwise AND operation.
/// Generic over types `T` supporting `BitAnd`, `From<u8>`, `PartialEq`.
pub fn is_odd<T>(num: T) -> bool
where
    T: core::ops::BitAnd<Output = T> + From<u8> + PartialEq,
{
    (num & T::from(1u8)) != T::from(0u8)
}

/// Checks if a number is even or odd efficiently, returning both results.
/// Uses a bitwise AND and derives odd status from even status.
/// Requires type `T` to also implement `Copy`.
pub fn is_even_or_odd<T>(num: T) -> (bool, bool)
where
    T: core::ops::BitAnd<Output = T> + From<u8> + PartialEq + Copy,
{
    let is_even = (num & T::from(1u8)) == T::from(0u8);
    (is_even, !is_even)
}
