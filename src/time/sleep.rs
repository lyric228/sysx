pub use std::time::Duration;
use std::{convert::TryFrom, num::TryFromIntError, str::FromStr, thread};

use thiserror::Error;

/// Suspends the current thread for the specified duration.
///
/// Accepts `u64` (ms), `f64` (s), `&str` ("100ms", "2s"), or `Duration`.
/// Returns `Err(SleepError)` for invalid inputs instead of panicking.
pub fn sleep<T: TryInto<SleepTime>>(time: T) -> Result<(), SleepError>
where
    T::Error: Into<SleepError>,
{
    let sleep_time = time.try_into().map_err(Into::into)?;
    thread::sleep(sleep_time.to_duration());
    Ok(())
}

/// Possible errors when parsing sleep time.
#[derive(Debug, Error, PartialEq)]
pub enum SleepError {
    /// Invalid time string format
    #[error("Invalid time format: '{0}'")]
    InvalidFormat(String),
    /// Time value out of range
    #[error("Time value out of range: {0}")]
    OutOfRange(String),
    /// Negative time value
    #[error("Negative sleep time: {0}")]
    NegativeTime(String),
    /// Integer conversion error
    #[error("Integer conversion error: {0}")]
    IntConversion(#[from] TryFromIntError),
}

/// Represents sleep time internally with nanosecond precision
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SleepTime {
    nanoseconds: u128,
}

impl SleepTime {
    /// Creates new SleepTime from nanoseconds
    pub fn new(nanoseconds: u128) -> Self {
        Self { nanoseconds }
    }

    /// Converts to Duration
    pub fn to_duration(self) -> Duration {
        Duration::from_nanos(self.nanoseconds as u64)
    }

    /// Total seconds as f64 (lossy conversion)
    pub fn as_secs_f64(&self) -> f64 {
        self.nanoseconds as f64 / 1_000_000_000.0
    }
}

// Safe conversions (infallible)
impl From<u64> for SleepTime {
    /// Converts milliseconds to SleepTime
    fn from(ms: u64) -> Self {
        Self {
            nanoseconds: ms as u128 * 1_000_000,
        }
    }
}

impl From<Duration> for SleepTime {
    /// Converts Duration to SleepTime
    fn from(d: Duration) -> Self {
        Self {
            nanoseconds: d.as_nanos(),
        }
    }
}

// Fallible conversions
impl TryFrom<f64> for SleepTime {
    type Error = SleepError;

    /// Converts seconds (f64) to SleepTime with error handling
    fn try_from(secs: f64) -> Result<Self, Self::Error> {
        if secs < 0.0 {
            return Err(SleepError::NegativeTime(secs.to_string()));
        }

        let nanoseconds = secs * 1_000_000_000.0;

        // Handle overflow and NaN
        if nanoseconds.is_nan() || nanoseconds.is_infinite() {
            return Err(SleepError::OutOfRange(secs.to_string()));
        }

        if nanoseconds > u128::MAX as f64 {
            return Err(SleepError::OutOfRange(secs.to_string()));
        }

        Ok(Self {
            nanoseconds: nanoseconds as u128,
        })
    }
}

impl TryFrom<&str> for SleepTime {
    type Error = SleepError;

    /// Parses time strings with units
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl FromStr for SleepTime {
    type Err = SleepError;

    /// Parses time strings with flexible format
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err(SleepError::InvalidFormat("empty string".into()));
        }

        // Split into numeric and alphabetic parts
        let num_end = s
            .find(|c: char| !c.is_ascii_digit() && c != '.' && c != '-' && c != '+')
            .unwrap_or(s.len());

        let (num_part, unit_part) = s.split_at(num_end);
        let unit_part = unit_part.trim().to_lowercase();

        // Parse number
        let num: f64 = num_part
            .parse()
            .map_err(|_| SleepError::InvalidFormat(format!("invalid number: '{num_part}'")))?;

        // Handle negative values
        if num < 0.0 {
            return Err(SleepError::NegativeTime(s.to_string()));
        }

        // Calculate multiplier
        let multiplier = match unit_part.as_str() {
            "" | "s" | "sec" | "secs" => 1_000_000_000.0,
            "ms" | "msec" => 1_000_000.0,
            "us" | "usec" => 1_000.0,
            "ns" | "nsec" => 1.0,
            "m" | "min" | "mins" => 60.0 * 1_000_000_000.0,
            "h" | "hour" | "hours" => 3600.0 * 1_000_000_000.0,
            _ => {
                return Err(SleepError::InvalidFormat(format!(
                    "unknown unit: '{unit_part}'"
                )));
            }
        };

        let nanoseconds = num * multiplier;

        // Check for overflow and invalid values
        if nanoseconds.is_nan() || nanoseconds.is_infinite() {
            return Err(SleepError::OutOfRange(s.to_string()));
        }

        if nanoseconds > u128::MAX as f64 {
            return Err(SleepError::OutOfRange(s.to_string()));
        }

        Ok(Self {
            nanoseconds: nanoseconds as u128,
        })
    }
}

// Additional utilities
impl SleepTime {
    /// Create from seconds (f64)
    pub fn from_secs_f64(secs: f64) -> Result<Self, SleepError> {
        secs.try_into()
    }

    /// Create from milliseconds (u64)
    pub fn from_millis(ms: u64) -> Self {
        ms.into()
    }
}
