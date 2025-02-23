
use std::cmp::Ordering;
use std::sync::Arc;

use rand::distr::{
    uniform::SampleUniform,
    Alphanumeric,
    Uniform,
};
use rand::{
    Rng,
    rng,
};
// TODO: Check rand::prelude, maybe choto krutoe

use crate::{
    Error,
    Result,
};


/// Generates a random value within the inclusive range [min, max].
///
/// Returns an Error::InvalidSyntax if the values cannot be compared.
///
/// # Arguments
/// * `min` - Lower bound (inclusive)
/// * `max` - Upper bound (inclusive)
///
/// # Examples
/// ```
/// let int = random(1, 10).unwrap();
/// let float = random(5.0, 7.5).unwrap();
/// ```
pub fn random<T>(min: T, max: T) -> Result<T>
where
    T: PartialOrd + Copy + SampleUniform,
{
    let (effective_min, effective_max) = match min.partial_cmp(&max) {
        Some(Ordering::Greater) => (max, min),
        Some(_) => (min, max),
        None => {
            return Err(Error::InvalidSyntax(
                "Invalid range comparison: cannot compare given values".into(),
            ))
        }
    };

    let mut rng = rng();
    let distr = Uniform::new_inclusive(effective_min, effective_max)?;
    Ok(rng.sample(distr))
}

/// Generates a random boolean value.
///
/// # Examples
/// ```
/// let random_bool = random_bool().unwrap();
/// ```
pub fn random_bool() -> Result<bool> {
    let mut rng = rng();
    Ok(rng.random_bool(0.5))
}

/// Generates a random string of the given length.
/// A custom charset can be provided; if None, an alphanumeric set is used.
///
/// # Arguments
/// * `length` - Length of the random string.
/// * `charset` - Optional custom set of characters. If None or empty, Alphanumeric is used.
///
/// # Errors
/// Returns Error::InvalidSyntax if a provided charset is empty or there is an error creating a distribution.
///
/// # Examples
/// ```
/// let rand_str = random_string(10, None).unwrap();
/// let custom_charset = "abcdef012345";
/// let rand_str_custom = random_string(15, Some(custom_charset)).unwrap();
/// ```
pub fn random_string(length: usize, charset: Option<&str>) -> Result<String> {
    let mut rng = rng();

    if let Some(chars) = charset {
        if chars.is_empty() {
            return Err(Error::InvalidSyntax("Provided charset is empty".into()));
        }
        let char_vec: Vec<char> = chars.chars().collect();
        let distr = Uniform::new(0, char_vec.len());
        let distr = distr.map_err(|e| Error::RandomError(e))?;
        let s: String = (0..length)
            .map(|_| {
                let idx = rng.sample(distr);
                char_vec[idx]
            })
            .collect();
        Ok(s)
    } else {
        let s: String = (0..length)
            .map(|_| rng.sample(Alphanumeric) as char)
            .collect();
        Ok(s)
    }
}

/// Generates a random vector of bytes of the given length.
///
/// # Arguments
/// * `length` - Number of random bytes to generate.
///
/// # Examples
/// ```
/// let random_bytes = random_bytes(16).unwrap();
/// ```
pub fn random_bytes(length: usize) -> Result<Vec<u8>> {
    let mut rng = rng();
    let bytes: Vec<u8> = (0..length).map(|_| rng.random()).collect();
    Ok(bytes)
}

/// Returns an iterator that produces random values within the inclusive range [min, max].
///
/// The iterator is infinite.
///
/// # Arguments
/// * `min` - Lower bound (inclusive)
/// * `max` - Upper bound (inclusive)
///
/// # Examples
/// ```
/// let mut iter = random_iter(1, 100).unwrap();
/// let first_value = iter.next().unwrap();
/// ```
pub fn random_iter<T>(min: T, max: T) -> Result<impl Iterator<Item = T>>
where
    T: PartialOrd + Copy + SampleUniform + 'static,
{
    let (effective_min, effective_max) = match min.partial_cmp(&max) {
        Some(Ordering::Greater) => (max, min),
        Some(_) => (min, max),
        None => {
            return Err(Error::InvalidSyntax(
                "Invalid range comparison: cannot compare given values".into(),
            ))
        }
    };

    let distr = Arc::new(Uniform::new_inclusive(effective_min, effective_max)?);
    let mut rng = rng();
    Ok(std::iter::repeat_with(move || rng.sample(&*distr)))
}

/// Generates a random value from an inclusive range.
///
/// # Arguments
/// * `range` - An inclusive range of values
///
/// # Examples
/// ```
/// let value = random_range(1..=10).unwrap();
/// ```
/// Generates a random value from an inclusive range.
///
/// # Arguments
/// * `range` - An inclusive range of values
///
/// # Examples
/// ```
/// let value = random_range(1..=10).unwrap();
/// ```
pub fn random_range<T>(range: std::ops::RangeInclusive<T>) -> Result<T>
where
    T: Copy + SampleUniform,
{
    let start = *range.start();
    let end = *range.end();
    let distr = Uniform::new_inclusive(start, end)?;
    let mut rng = rng();
    Ok(rng.sample(distr))
}

/// Returns a random boolean value based on the provided ratio (numerator/denominator).
///
/// Returns true with probability numerator/denominator.
///
/// # Arguments
/// * `numerator` - The numerator of the ratio
/// * `denominator` - The denominator of the ratio
///
/// # Examples
/// ```
/// let flag = random_ratio(1, 3).unwrap(); // Approximately 33% chance true
/// ```
pub fn random_ratio(numerator: u32, denominator: u32) -> Result<bool> {
    if denominator == 0 {
        return Err(Error::InvalidSyntax("Denominator cannot be zero".into()));
    }
    let mut rng = rng();
    Ok(rng.random_ratio(numerator, denominator))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_int_range() {
        let num = random(5, 15).unwrap();
        assert!(num >= 5 && num <= 15);
    }

    #[test]
    fn test_random_float_range() {
        let num = random(1.0, 2.0).unwrap();
        assert!(num >= 1.0 && num <= 2.0);
    }

    #[test]
    fn test_random_iter() {
        let mut iter = random_iter(1, 10).unwrap();
        let first = iter.next().unwrap();
        assert!(first >= 1 && first <= 10);
    }

    #[test]
    fn test_random_range() {
        let value = random_range(1..=100).unwrap();
        assert!(value >= 1 && value <= 100);
    }

    #[test]
    fn test_random_ratio() {
        let flag = random_ratio(1, 2).unwrap();
        assert!(flag == true || flag == false);
    }
}
