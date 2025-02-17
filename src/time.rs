use std::time::Duration;
use std::thread;
use std::str::FromStr;
use thiserror::Error;


/// Sleep with support for multiple input types and units
/// 
/// # Arguments
/// * `time` - Can be:
///   - `u64` (milliseconds)
///   - `Duration` directly
///   - `&str` with unit suffix (e.g. "5s", "100ms", "1.5h")
///   - `f64` (seconds)
/// 
/// # Examples
/// ```
/// sleep(500); // 500 ms
/// sleep("2s"); // 2 seconds
/// sleep("150ms"); // 150 milliseconds
/// sleep(1.8); // 1.8 seconds
/// sleep(Duration::from_secs(3));
/// ```
pub fn sleep<T: Into<SleepTime>>(time: T) {
    let duration = time.into().to_duration();
    thread::sleep(duration);
}

#[derive(Debug, Error)]
pub enum SleepError {
    #[error("Invalid time format: {0}")]
    InvalidFormat(String),
    #[error("Time value out of range")]
    OutOfRange,
    #[error("Negative time duration")]
    NegativeTime,
}

#[derive(Debug, Clone, Copy)]
pub struct SleepTime {
    seconds: f64,
}

impl SleepTime {
    /// Convert to Duration with overflow protection
    pub fn to_duration(self) -> Duration {
        assert!(self.seconds >= 0.0, "Time cannot be negative");
        
        // Безопасное преобразование с защитой от переполнения
        let secs = self.seconds.trunc() as u64;
        let nanos = (self.seconds.fract() * 1_000_000_000.0).round() as u32;
        
        // Корректировка при переполнении наносекунд
        let (secs, nanos) = if nanos >= 1_000_000_000 {
            (secs.saturating_add(1), 0)
        } else {
            (secs, nanos)
        };

        Duration::new(secs, nanos)
    }
}

impl From<u64> for SleepTime {
    /// Interpret u64 as milliseconds
    fn from(ms: u64) -> Self {
        SleepTime { seconds: ms as f64 / 1000.0 }
    }
}

impl From<Duration> for SleepTime {
    fn from(d: Duration) -> Self {
        SleepTime { seconds: d.as_secs_f64() }
    }
}

impl From<f64> for SleepTime {
    fn from(secs: f64) -> Self {
        SleepTime { seconds: secs }
    }
}

impl FromStr for SleepTime {
    type Err = SleepError;

    /// Parse string with time units:
    /// - ns: nanoseconds
    /// - ms: milliseconds
    /// - s: seconds (default)
    /// - m: minutes
    /// - h: hours
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_lowercase();
        let (num_part, unit) = s.split_at(
            s.find(|c: char| !c.is_numeric() && c != '.')
             .unwrap_or_else(|| s.len())
        );

        let num: f64 = num_part.parse()
            .map_err(|_| SleepError::InvalidFormat(s.clone()))?;

        let multiplier = match unit {
            "ns" => 1e-9,
            "ms" => 1e-3,
            "m" => 60.0,
            "h" => 3600.0,
            "s" | "" => 1.0,
            _ => return Err(SleepError::InvalidFormat(s.clone())),
        };

        if num < 0.0 {
            return Err(SleepError::NegativeTime);
        }

        Ok(SleepTime {
            seconds: num * multiplier
        })
    }
}

pub fn safe_sleep<T: Into<SleepTime>>(time: T) -> Result<(), SleepError> {
    let sleep_time = match time.into() {
        t if t.seconds < 0.0 => return Err(SleepError::NegativeTime),
        t => t,
    };
    
    thread::sleep(sleep_time.to_duration());
    Ok(())
}
