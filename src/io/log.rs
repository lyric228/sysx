pub use colored::{
    ColoredString,
    Colorize,
    Color,
};
pub use chrono::Local;


/// Logging level с привязанными стилями.
/// 
/// Перечисление описывает уровни логирования с их визуальным представлением.
/// 
/// # Пример
/// ```
/// let level = LogLevel::Info;
/// assert_eq!(format!("{:?}", level), "Info");
/// ```
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
    /// Возвращает цвет, соответствующий лог уровню.
    ///
    /// # Пример
    /// ```
    /// let color = LogLevel::Warning.style();
    /// // Вернёт Color::Yellow
    /// ```
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

/// Макрос для преобразования идентификатора лог-уровня в значение LogLevel.
/// 
/// Принимает уровень логирования (например, INFO) и возвращает соответствующий элемент
/// перечисления LogLevel.
/// 
/// # Пример
/// ```
/// // Передаём идентификатор уровня
/// let lvl = log_level!(INFO);
/// // lvl будет равен LogLevel::Info
/// ```
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

/// Основной макрос логирования с упрощённым синтаксисом.
/// 
/// Формирует лог-сообщение с указанным уровнем и текстом, а при наличии опционального
/// контекста - также выводит его.
/// 
/// # Пример
/// ```
/// // Пример вызова без контекста:
/// log!(INFO, "System initialized");
/// 
/// // Пример вызова с контекстом:
/// log!(ERROR, "File not found"; "Path: /etc/config.yaml");
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
pub use log;

/// Внутренний макрос логирования, который осуществляет фактический вывод сообщения.
/// 
/// Принимает уровень логирования ($level:expr), сформированное сообщение ($msg:expr),
/// а также опциональный контекст ($ctx:expr). Выводит сообщение с временной меткой и стилизацией.
/// 
/// # Пример
/// ```
/// log_internal!(LogLevel::Debug, format!("Debug info: {}", 42), None);
/// ```
#[macro_export]
macro_rules! log_internal {
    ($level:expr, $msg:expr, $ctx:expr) => {{
        let color = $level.style();
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let level_name = format!("{:?}", $level).to_uppercase();
        
        let styled_msg = $crate::style!(
            format!("[{}] {}", level_name, $msg), 
            color, 
            bold
        );
        
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

/// Макрос для стилизации текста с помощью цепочки методов.
/// 
/// Первым параметром принимает текст (или строку), вторым цвет или лог уровень, а третьим
/// опционально набор стилей: например, bold, italic.
/// 
/// # Примеры
/// ```
/// // Пример с лог уровнем:
/// let text = style!("Warning", LogLevel::Warning);
/// // Вернёт строку, стилизованную в цвет, соответствующий LogLevel::Warning, с жирным шрифтом.
///
/// // Пример с указанием цвета и стилей:
/// let text = style!("Error", Color::Red, bold italic);
/// // Вернёт красный текст с жирным и наклонным стилем.
/// ```
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

/// Форматирует временную метку с использованием текущего времени.
/// 
/// Возвращает строку с временной меткой, стилизованную в приглушённом цвете.
/// 
/// # Пример
/// ```
/// let timestamp = format_timestamp();
/// // Вернёт, например, "2023-10-05 14:23:45.678" в приглушённом цвете.
/// ```
pub fn format_timestamp() -> ColoredString {
    Local::now()
        .format("%Y-%m-%d %H:%M:%S%.3f")
        .to_string()
        .dimmed()
}
