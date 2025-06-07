use std::{cmp::Ordering, sync::Arc};

use rand::{
    Rng,
    distr::{Alphanumeric, Uniform, uniform::SampleUniform},
    rng,
};

use crate::{Error, Result};

/// Generates a random value within the inclusive range [min, max].
/// Swaps min/max if min > max. Returns Error if values cannot be compared.
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
            ));
        }
    };

    let mut rng = rng();
    let distr = Uniform::new_inclusive(effective_min, effective_max)?;
    Ok(rng.sample(distr))
}

/// Generates a random boolean (50% chance).
pub fn random_bool() -> Result<bool> {
    let mut rng = rng();
    Ok(rng.random_bool(0.5))
}

/// Generates a random string of `length`. Uses `charset` if provided, otherwise alphanumeric.
/// Returns Error if `charset` is empty or distribution fails.
pub fn random_string(length: usize, charset: Option<&str>) -> Result<String> {
    let mut rng = rng();

    if let Some(chars) = charset {
        if chars.is_empty() {
            return Err(Error::InvalidSyntax("Provided charset is empty".into()));
        }
        let char_vec: Vec<char> = chars.chars().collect();
        let distr = Uniform::new(0, char_vec.len());
        let distr = distr.map_err(Error::RandomError)?;
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

/// Generates a vector of `length` random bytes.
pub fn random_bytes(length: usize) -> Result<Vec<u8>> {
    let mut rng = rng();
    let bytes: Vec<u8> = (0..length).map(|_| rng.random()).collect();
    Ok(bytes)
}

/// Returns an infinite iterator of random values within the inclusive range [min, max].
/// Swaps min/max if min > max. Returns Error if values cannot be compared.
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
            ));
        }
    };

    let distr = Arc::new(Uniform::new_inclusive(effective_min, effective_max)?);
    let mut rng = rng();
    Ok(std::iter::repeat_with(move || rng.sample(&*distr)))
}

/// Generates a random value from an inclusive `range`.
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

/// Returns a random boolean based on `numerator`/`denominator` probability.
/// Returns Error if denominator is zero.
pub fn random_ratio(numerator: u32, denominator: u32) -> Result<bool> {
    if denominator == 0 {
        return Err(Error::InvalidSyntax("Denominator cannot be zero".into()));
    }
    let mut rng = rng();
    Ok(rng.random_ratio(numerator, denominator))
}
