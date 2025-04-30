pub use chrono::Local;
pub use colored::{Color, ColoredString, Colorize};

/// Logging levels with associated styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Info,
    Success,
    Warning,
    Error,
    Bug,
    Fatal,
    Debug,
    Trace,
}

impl LogLevel {
    /// Returns the color associated with the log level.
    pub fn style(&self) -> Color {
        match self {
            LogLevel::Info => Color::Blue,
            LogLevel::Success => Color::Green,
            LogLevel::Warning => Color::Yellow,
            LogLevel::Error => Color::Red,
            LogLevel::Bug => Color::BrightRed,
            LogLevel::Fatal => Color::BrightRed,
            LogLevel::Debug => Color::Magenta,
            LogLevel::Trace => Color::Cyan,
        }
    }
}

/// Macro to convert a log level identifier (e.g., INFO) to a LogLevel enum value.
#[macro_export]
macro_rules! log_level {
    ($level:ident) => {{
        use $crate::io::log::LogLevel::*;
        match stringify!($level).to_uppercase().as_str() {
            "INFO" => Info,
            "SUCCESS" => Success,
            "WARNING" => Warning,
            "ERROR" => Error,
            "BUG" => Bug,
            "FATAL" => Fatal,
            "DEBUG" => Debug,
            "TRACE" => Trace,
            _ => panic!("Unknown log level: {}", stringify!($level)),
        }
    }};
}
pub use log_level;

/// Primary logging macro with simplified syntax.
/// Formats a log message with a specified level and text, and optional context.
#[macro_export]
macro_rules! log {
    ($level:ident, $($msg:tt)*) => {
        $crate::log_internal!(
            $crate::log_level!($level),
            format!($($msg)*),
            None
        )
    };

    ($level:ident, $($msg:tt)*; $ctx:expr) => {
        $crate::log_internal!(
            $crate::log_level!($level),
            format!($($msg)*),
            Some($ctx.to_string())
        )
    };
}
pub use log;

/// Internal logging macro that handles the actual message output.
/// Takes log level, formatted message, and optional context.
#[macro_export]
macro_rules! log_internal {
    ($level:expr, $msg:expr, $ctx:expr) => {{
        let color = $level.style();
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let level_name = format!("{:?}", $level).to_uppercase();
        let styled_msg = $crate::style!(format!("[{}] {}", level_name, $msg), color, bold);
        let ctx_str = $ctx.map(|c: String| format!("\n  ↳ {}", c.dimmed()));

        println!(
            "{} {} {}",
            timestamp.to_string().dimmed(),
            styled_msg,
            ctx_str.unwrap_or_default(),
        );
    }};
}
pub use log_internal;

/// Macro for styling text using method chaining.
/// Takes text, a color or log level, and optional styles (e.g., bold, italic).
#[macro_export]
macro_rules! style {
    ($text:expr, $level:expr) => {{
        let color = $level.style();
        $text.color(color).bold()
    }};

    ($text:expr, $color:expr) => {
        $text.color($color)
    };

    ($text:expr, $color:expr, $($style:ident)+) => {
        $text.color($color)$(.$style())+
    };
}
pub use style;

/// Formats the current time as a dimmed timestamp string.
pub fn format_timestamp() -> ColoredString {
    Local::now()
        .format("%Y-%m-%d %H:%M:%S%.3f")
        .to_string()
        .dimmed()
}
