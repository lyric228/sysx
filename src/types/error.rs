use anyhow::Error as AnyhowError;
use rand::distr::uniform::Error as RandUniformError;
use regex::Error as RegexError;
use thiserror::Error;

/// Main error type for the sysx library.
#[derive(Debug, Error)]
pub enum SysxError {
    /// Invalid type syntax encountered during parsing.
    #[error("Invalid type syntax: {0}")]
    InvalidSyntax(String),

    /// Nested generics are not supported in type definitions.
    #[error("Nested generics not supported in type: {0}")]
    NestedGenerics(String),

    /// Environment variable not found error.
    #[error("Environment variable not found: {0}")]
    EnvVarNotFound(String),

    /// Regular expression compilation failure.
    #[error("Regex compilation failed: {0}")]
    RegexFailure(#[from] RegexError),

    /// Type validation mismatch error.
    #[error(
        "Type validation error: expected {expected}, found {actual}{:?}",
        context
    )]
    ValidationError {
        /// Expected type description.
        expected: &'static str,
        /// Detected type description.
        actual: String,
        /// Additional error context.
        context: Option<String>,
    },

    /// Unsupported type construct detected.
    #[error("Unsupported type construct: {0}")]
    UnsupportedConstruct(String),

    /// Error related to random generation operations.
    #[error("Random generation error: {0}")]
    RandomError(#[from] RandUniformError),

    /// Error related to time-based operations.
    #[error("Time error: {0}")]
    TimeError(TimeError),

    /// Formatting error.
    #[error(transparent)]
    FmtError(#[from] std::fmt::Error),

    /// I/O error.
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    /// Anyhow error.
    #[error(transparent)]
    AnyhowError(#[from] AnyhowError),

    /// ParseInt error.
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    /// ParseFloat error.
    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),

    /// FromUtf8 error.
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    /// Path strip prefix error.
    #[error("Path strip prefix error: {0}")]
    StripPrefixError(#[from] std::path::StripPrefixError),

    /// Mutex poison error.
    #[error("Mutex poisoned: {0}")]
    MutexPoison(String),
}

/// Errors related to time-based operations (e.g., sleep).
#[derive(Debug, Error)]
pub enum TimeError {
    /// Invalid time format string provided.
    #[error("Invalid time format: {0}")]
    InvalidFormat(String),

    /// Time value exceeds the supported range.
    #[error("Time value out of range")]
    OutOfRange,

    /// Negative time duration specified.
    #[error("Negative time duration specified")]
    NegativeDuration,
}

impl From<std::sync::PoisonError<std::sync::MutexGuard<'_, std::collections::HashMap<String, String>>>> 
    for SysxError 
{
    fn from(err: std::sync::PoisonError<std::sync::MutexGuard<'_, std::collections::HashMap<String, String>>>) -> Self {
        SysxError::MutexPoison(format!("Mutex consistency error: {}", err))
    }
}

/// Alias for the result type returned by sysx library functions.
pub type Result<T> = std::result::Result<T, SysxError>;

/// Alias for the main error type of the sysx library.
pub type Error = SysxError;
