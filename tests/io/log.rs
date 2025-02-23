use colored::Color;
use sysx::log::*;


// LogLevel tests

#[test]
// Тест соответствия стилей для каждого уровня логирования.
// Проверяет корректность назначения цветов для всех уровней лога.
fn test_log_level_styles() {
    assert_eq!(LogLevel::Info.style(), Color::Blue);
    assert_eq!(LogLevel::Success.style(), Color::Green);
    assert_eq!(LogLevel::Warning.style(), Color::Yellow);
    assert_eq!(LogLevel::Error.style(), Color::Red);
    assert_eq!(LogLevel::Bug.style(), Color::BrightRed);
    assert_eq!(LogLevel::Fatal.style(), Color::BrightRed);
    assert_eq!(LogLevel::Debug.style(), Color::Magenta);
    assert_eq!(LogLevel::Trace.style(), Color::Cyan);
}

#[test]
// Тест корректности Debug реализации для LogLevel.
// Проверяет строковое представление уровней лога.
fn test_log_level_debug() {
    assert_eq!(format!("{:?}", LogLevel::Info), "Info");
    assert_eq!(format!("{:?}", LogLevel::Warning), "Warning");
    assert_eq!(format!("{:?}", LogLevel::Error), "Error");
}


// log_level! macro tests

#[test]
// Тест преобразования строковых идентификаторов в LogLevel.
// Проверяет работу макроса log_level для всех уровней.
fn test_log_level_macro() {
    assert_eq!(log_level!(INFO), LogLevel::Info);
    assert_eq!(log_level!(SUCCESS), LogLevel::Success);
    assert_eq!(log_level!(WARNING), LogLevel::Warning);
    assert_eq!(log_level!(ERROR), LogLevel::Error);
    assert_eq!(log_level!(BUG), LogLevel::Bug);
    assert_eq!(log_level!(FATAL), LogLevel::Fatal);
    assert_eq!(log_level!(DEBUG), LogLevel::Debug);
    assert_eq!(log_level!(TRACE), LogLevel::Trace);
}

#[test]
#[should_panic(expected = "Unknown log level")]
// Тест обработки некорректного уровня лога.
// Проверяет, что макрос паникует при неизвестном уровне.
fn test_log_level_macro_invalid() {
    let _level = log_level!(INVALID);
}


// style! macro tests

#[test]
// Тест стилизации текста с уровнем лога.
// Проверяет применение цвета и жирного стиля.
fn test_style_with_log_level() {
    let styled = style!("test", LogLevel::Warning);
    assert_eq!(styled.fgcolor(), Some(Color::Yellow));
}

#[test]
// Тест стилизации текста с явным указанием цвета.
// Проверяет применение только цвета без дополнительных стилей.
fn test_style_with_color() {
    let styled = style!("test", Color::Red);
    assert_eq!(styled.fgcolor(), Some(Color::Red));
}

#[test]
// Тест стилизации текста с цветом и дополнительными стилями.
// Проверяет применение цвета и множественных стилей.
fn test_style_with_color_and_styles() {
    let styled = style!("test", Color::Blue, bold italic);
    assert_eq!(styled.fgcolor(), Some(Color::Blue));
}


// format_timestamp tests

#[test]
// Тест форматирования временной метки.
// Проверяет формат и стиль временной метки.
fn test_format_timestamp() {
    let timestamp = format_timestamp();
    assert!(timestamp.contains(":"));  // Проверяем наличие разделителя времени
    assert!(timestamp.is_dimmed());    // Проверяем применение стиля dimmed
}


// log! macro tests

#[test]
// Тест макроса логирования без контекста.
// Проверяет базовый вывод лога.
fn test_log_macro_basic() {
    log!(INFO, "Test message");  // Визуальная проверка вывода
}

#[test]
// Тест макроса логирования с контекстом.
// Проверяет вывод лога с дополнительным контекстом.
fn test_log_macro_with_context() {
    log!(ERROR, "Test error"; "Additional context");  // Визуальная проверка вывода
}
