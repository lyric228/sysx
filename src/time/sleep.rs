use std::str::FromStr;
use std::thread;
pub use std::time::Duration;

use thiserror::Error;

/// Suspends the current thread for the specified duration.
///
/// Accepts `u64` (ms), `f64` (s), `&str` ("100ms", "2s"), or `Duration`.
pub fn sleep<T: Into<SleepTime>>(time: T) {
    let duration = time.into().to_duration();
    thread::sleep(duration);
}

/// Possible errors when parsing sleep time.
#[derive(Debug, Error)]
pub enum SleepError {
    /// Invalid time string format.
    #[error("Invalid time format: {0}")]
    InvalidFormat(String),
    /// Time value out of range.
    #[error("Time out of range")]
    OutOfRange,
    /// Negative time value.
    #[error("Negative sleep time")]
    NegativeTime,
}

/// Represents sleep time internally.
/// Stores time in seconds as a floating-point number.
#[derive(Debug, Clone, Copy)]
pub struct SleepTime {
    seconds: f64,
}

impl SleepTime {
    /// Converts `SleepTime` to `Duration`, handling potential overflow.
    pub fn to_duration(self) -> Duration {
        let seconds = self.seconds.abs();
        let secs = seconds.trunc() as u64;
        let nanos = (seconds.fract() * 1_000_000_000.0).round() as u32;

        // Adjust for potential floating-point inaccuracies near the second boundary
        let (secs, nanos) = if nanos >= 1_000_000_000 {
            (secs.saturating_add(1), 0)
        } else {
            (secs, nanos)
        };

        Duration::new(secs, nanos)
    }
}

impl From<u64> for SleepTime {
    /// Converts milliseconds (`u64`) to `SleepTime`.
    fn from(ms: u64) -> Self {
        SleepTime {
            seconds: ms as f64 / 1000.0,
        }
    }
}

impl From<Duration> for SleepTime {
    /// Converts `Duration` to `SleepTime`.
    fn from(d: Duration) -> Self {
        SleepTime {
            seconds: d.as_secs_f64(),
        }
    }
}

impl From<f64> for SleepTime {
    /// Converts seconds (`f64`) to `SleepTime`.
    fn from(secs: f64) -> Self {
        SleepTime { seconds: secs }
    }
}

impl From<&str> for SleepTime {
    /// Converts string slice to `SleepTime` using `FromStr`.
    /// Panics on parsing errors (use `safe_sleep` for fallible parsing).
    fn from(s: &str) -> Self {
        s.parse()
            .unwrap_or_else(|_| panic!("Failed to convert string '{s}' to SleepTime"))
    }
}

impl FromStr for SleepTime {
    type Err = SleepError;

    /// Parses a time string with optional units (ns, ms, s, m, h).
    /// Defaults to seconds if no unit is provided.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_lowercase();
        let (num_part, unit) = s.split_at(
            s.find(|c: char| !c.is_numeric() && c != '.')
                .unwrap_or(s.len()),
        );

        let num: f64 = num_part
            .parse()
            .map_err(|_| SleepError::InvalidFormat(s.to_string()))?;

        let multiplier = match unit {
            "ns" => 1e-9,
            "ms" => 1e-3,
            "m" => 60.0,
            "h" => 3600.0,
            "s" | "" => 1.0,
            _ => return Err(SleepError::InvalidFormat(s.to_string())),
        };

        if num < 0.0 {
            return Err(SleepError::NegativeTime);
        }

        Ok(SleepTime {
            seconds: num * multiplier,
        })
    }
}

/// Attempts to sleep for the given time, returning an error on failure.
///
/// Returns `Err(SleepError::NegativeTime)` if the time is negative.
pub fn safe_sleep<T: Into<SleepTime>>(time: T) -> Result<(), SleepError> {
    let sleep_time = time.into();

    if sleep_time.seconds < 0.0 {
        return Err(SleepError::NegativeTime);
    }

    thread::sleep(sleep_time.to_duration());
    Ok(())
}
