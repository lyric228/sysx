use anyhow::Error as AnyhowError;
use rand::distr::uniform::Error as RandUniformError;
use regex::Error as RegexError;
use thiserror::Error;

/// Main error type for the sysx library.
#[derive(Debug, Error)]
pub enum SysxError {
    /// Invalid type syntax during parsing.
    #[error("Invalid type syntax: {0}")]
    InvalidSyntax(String),

    /// Nested generics are not supported.
    #[error("Nested generics not supported in type: {0}")]
    NestedGenerics(String),

    /// Environment variable not found.
    #[error("Environment variable not found: {0}")]
    EnvVarNotFound(String),

    /// Regex compilation failed.
    #[error("Regex compilation failed: {0}")]
    RegexFailure(#[from] RegexError),

    /// Type validation mismatch.
    #[error(
        "Type validation error: expected {:?}, found {:?}{:?}",
        expected,
        actual,
        context
    )]
    ValidationError {
        /// Expected type.
        expected: String,
        /// Actual type.
        actual:   String,
        /// Additional context.
        context:  Option<String>,
    },

    /// Unsupported type construct.
    #[error("Unsupported type construct: {0}")]
    UnsupportedConstruct(String),

    /// Error during random generation.
    #[error("Random generation error: {0}")]
    RandomError(#[from] RandUniformError),

    /// Time-based operation error.
    #[error("Time error: {0}")]
    TimeError(TimeError),

    /// Formatting error.
    #[error(transparent)]
    FmtError(#[from] std::fmt::Error),

    /// I/O error.
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    /// Anyhow error wrapper.
    #[error(transparent)]
    AnyhowError(#[from] AnyhowError),

    /// ParseInt error wrapper.
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    /// ParseFloat error wrapper.
    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),

    /// FromUtf8 error wrapper.
    #[error(transparent)]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    /// Path strip prefix error wrapper.
    #[error("Path strip prefix error: {0}")]
    StripPrefixError(#[from] std::path::StripPrefixError),

    /// Serialization error.
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

/// Errors for time-based operations.
#[derive(Debug, Error)]
pub enum TimeError {
    /// Invalid time format string.
    #[error("Invalid time format: {0}")]
    InvalidFormat(String),

    /// Time value out of supported range.
    #[error("Time value out of range")]
    OutOfRange,

    /// Negative time duration specified.
    #[error("Negative time duration specified")]
    NegativeDuration,
}

/// Result type for sysx library functions.
pub type Result<T> = std::result::Result<T, SysxError>;

/// Main error type alias.
pub type Error = SysxError;
