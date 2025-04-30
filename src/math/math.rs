/// Checks if a number is even.
///
/// This function uses a bitwise AND operation (`& 1`) to efficiently
/// determine if the least significant bit is zero.
/// It's generic over types `T` that support the necessary bitwise (`BitAnd`),
/// conversion (`From<u8>`), and comparison (`PartialEq`) operations.
///
/// # Arguments
///
/// * `num` - The number to check.
///
/// # Returns
///
/// `true` if the number is even, `false` otherwise.
pub fn is_even<T>(num: T) -> bool
where
    T: core::ops::BitAnd<Output = T> + From<u8> + PartialEq,
{
    (num & T::from(1u8)) == T::from(0u8)
}

/// Checks if a number is odd.
///
/// This function uses a bitwise AND operation (`& 1`) to efficiently
/// determine if the least significant bit is one.
/// It's generic over types `T` that support the necessary bitwise (`BitAnd`),
/// conversion (`From<u8>`), and comparison (`PartialEq`) operations.
///
/// # Arguments
///
/// * `num` - The number to check.
///
/// # Returns
///
/// `true` if the number is odd, `false` otherwise.
pub fn is_odd<T>(num: T) -> bool
where
    T: core::ops::BitAnd<Output = T> + From<u8> + PartialEq,
{
    (num & T::from(1u8)) != T::from(0u8)
}

/// Checks if a number is even or odd, returning both results efficiently.
///
/// This function calculates the even status once using a bitwise AND (`& 1`)
/// and derives the odd status directly from it (`!is_even`), avoiding redundant checks.
/// Use this function when you need both the even and odd status of a number.
///
/// Requires the type `T` to also implement the `Copy` trait, in addition to
/// `BitAnd`, `From<u8>`, and `PartialEq`.
///
/// # Arguments
///
/// * `num` - The number to check. Must be `Copy`.
///
/// # Returns
///
/// A tuple `(bool, bool)` where the first element is `true` if `num` is even,
/// and the second element is `true` if `num` is odd.
pub fn is_even_or_odd<T>(num: T) -> (bool, bool)
where
    T: core::ops::BitAnd<Output = T> + From<u8> + PartialEq + Copy,
{
    let is_even = (num & T::from(1u8)) == T::from(0u8);
    (is_even, !is_even)
}
