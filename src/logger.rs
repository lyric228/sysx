pub use colored::{Colorize, ColoredString, Color};
pub use chrono::Local;

/// Logging level with associated styles
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
    pub fn style(&self) -> (char, Color) {
        match self {
            LogLevel::Info => ('ℹ', Color::Blue),
            LogLevel::Success => ('✅', Color::Green),
            LogLevel::Warning => ('⚠', Color::Yellow),
            LogLevel::Error => ('❌', Color::Red),
            LogLevel::Bug => ('🐛', Color::BrightRed),
            LogLevel::Fatal => ('💀', Color::BrightRed),
            LogLevel::Debug => ('🐞', Color::Magenta),
            LogLevel::Trace => ('🔍', Color::Cyan),
        }
    }
}

#[macro_export]
macro_rules! log_level {
    ($level:ident) => {{
        #[allow(unused_imports)]
        use $crate::logger::LogLevel::*;
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

/// Main logging macro with simplified syntax
/// 
/// # Examples
/// ```
/// log!(INFO, "System initialized");
/// log!(ERROR, "File not found", "Path: /etc/config.yaml");
/// ```
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

/// Internal logging implementation
#[macro_export]
macro_rules! log_internal {
    ($level:expr, $msg:expr, $ctx:expr) => {{
        use $crate::logger::LogLevel;
        
        let (icon, color) = $level.style();
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let level_name = format!("{:?}", $level).to_uppercase();
        
        let styled_msg = $crate::style!(
            format!("[{}] {}", level_name, $msg), 
            color, 
            bold
        );
        
        let ctx_str = $ctx.map(|c: String| format!("\n  ↳ {}", c.dimmed()));
        
        println!(
            "{}   {} {}",
            timestamp.to_string().dimmed(),
            // icon.to_string().color(color),
            styled_msg,
            ctx_str.unwrap_or_default(),
        );
    }};
}

/// Style text with chained methods
/// 
/// # Examples
/// ```
/// let text = style!("Warning", LogLevel::Warning);
/// let text = style!("Error", Color::Red, bold italic);
/// ```
#[macro_export]
macro_rules! style {
    ($text:expr, $level:expr) => {{
        let (_, color) = $level.style();
        $text.color(color).bold()
    }};
    ($text:expr, $color:expr) => {
        $text.color($color)
    };
    ($text:expr, $color:expr, $($style:ident)+) => {
        $text.color($color)$(.$style())+
    };
}

/// Format timestamp with current time
pub fn format_timestamp() -> ColoredString {
    Local::now()
        .format("%Y-%m-%d %H:%M:%S%.3f")
        .to_string()
        .dimmed()
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::Color;

    #[test]
    fn test_log_level_styles() {
        assert_eq!(LogLevel::Success.style(), ('✅', Color::Green));
        assert_eq!(LogLevel::Fatal.style(), ('💀', Color::BrightRed));
    }

    #[test]
    fn test_log_macro() {
        log!(INFO, "Test message");
        log!(WARNING, "Deprecation notice"; "Will be removed in v2.0");
    }

    #[test]
    fn test_style_macro() {
        let styled = style!("Important", LogLevel::Error);
        assert!(styled.to_string().contains("31"));
        
        let styled = style!("Note", Color::Blue, italic);
        assert!(styled.to_string().contains("3"));
    }
}
