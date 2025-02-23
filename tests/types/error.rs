use std::num::{
    ParseIntError,
    ParseFloatError,
};
use std::path::StripPrefixError;
use std::string::FromUtf8Error;
use std::io;

use anyhow::Error as AnyhowError;
use regex::Error as RegexError;
use sysx::types::error::*;


// SysxError conversion tests

#[test]
// Тест конвертации RegexError в SysxError.
// Проверяет автоматическое преобразование ошибок regex.
fn test_from_regex_error() {
    let regex_err = regex::Regex::new("[").unwrap_err();
    let sysx_err: SysxError = regex_err.into();
    assert!(matches!(sysx_err, SysxError::RegexFailure(_)));
}

#[test]
// Тест конвертации IO ошибок в SysxError.
// Проверяет автоматическое преобразование io::Error.
fn test_from_io_error() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let sysx_err: SysxError = io_err.into();
    assert!(matches!(sysx_err, SysxError::IoError(_)));
}

#[test]
// Тест конвертации ParseIntError в SysxError.
// Проверяет автоматическое преобразование ошибок парсинга целых чисел.
fn test_from_parse_int_error() {
    let parse_err: ParseIntError = "abc".parse::<i32>().unwrap_err();
    let sysx_err: SysxError = parse_err.into();
    assert!(matches!(sysx_err, SysxError::ParseIntError(_)));
}

#[test]
// Тест конвертации ParseFloatError в SysxError.
// Проверяет автоматическое преобразование ошибок парсинга чисел с плавающей точкой.
fn test_from_parse_float_error() {
    let parse_err: ParseFloatError = "abc".parse::<f64>().unwrap_err();
    let sysx_err: SysxError = parse_err.into();
    assert!(matches!(sysx_err, SysxError::ParseFloatError(_)));
}

#[test]
// Тест конвертации FromUtf8Error в SysxError.
// Проверяет автоматическое преобразование ошибок UTF-8.
fn test_from_utf8_error() {
    let invalid_utf8 = vec![0xFF, 0xFF];
    let utf8_err = String::from_utf8(invalid_utf8).unwrap_err();
    let sysx_err: SysxError = utf8_err.into();
    assert!(matches!(sysx_err, SysxError::FromUtf8Error(_)));
}

#[test]
// Тест конвертации StripPrefixError в SysxError.
// Проверяет автоматическое преобразование ошибок работы с путями.
fn test_from_strip_prefix_error() {
    use std::path::Path;
    let prefix = Path::new("/a");
    let path = Path::new("/b");
    let prefix_err = path.strip_prefix(prefix).unwrap_err();
    let sysx_err: SysxError = prefix_err.into();
    assert!(matches!(sysx_err, SysxError::StripPrefixError(_)));
}


// SysxError creation tests

#[test]
// Тест создания ошибки InvalidSyntax.
// Проверяет создание и сообщение ошибки синтаксиса.
fn test_invalid_syntax_error() {
    let err = SysxError::InvalidSyntax("invalid token".to_string());
    assert!(err.to_string().contains("Invalid type syntax"));
}

#[test]
// Тест создания ошибки ValidationError.
// Проверяет создание и сообщение ошибки валидации.
fn test_validation_error() {
    let err = SysxError::ValidationError {
        expected: "string",
        actual: "integer".to_string(),
        context: Some("in parameter".to_string()),
    };
    let err_str = err.to_string();
    assert!(err_str.contains("expected string"));
    assert!(err_str.contains("found integer"));
}


// TimeError tests

#[test]
// Тест создания и форматирования TimeError.
// Проверяет различные варианты ошибок времени.
fn test_time_errors() {
    let format_err = TimeError::InvalidFormat("invalid format".to_string());
    assert!(format_err.to_string().contains("Invalid time format"));

    let range_err = TimeError::OutOfRange;
    assert!(range_err.to_string().contains("out of range"));

    let negative_err = TimeError::NegativeDuration;
    assert!(negative_err.to_string().contains("Negative time duration"));
}

#[test]
// Тест конвертации TimeError в SysxError.
// Проверяет преобразование ошибок времени в основной тип ошибок.
fn test_time_error_conversion() {
    let time_err = TimeError::OutOfRange;
    let sysx_err = SysxError::TimeError(time_err);
    assert!(sysx_err.to_string().contains("out of range"));
}


// Result type tests

#[test]
// Тест использования типа Result.
// Проверяет работу с псевдонимом Result.
fn test_result_type() {
    let success: Result<i32> = Ok(42);
    assert_eq!(success.unwrap(), 42);

    let failure: Result<()> = Err(SysxError::InvalidSyntax("test".to_string()));
    assert!(failure.is_err());
}
