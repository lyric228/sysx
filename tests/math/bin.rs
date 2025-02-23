use sysx::math::bin::*;
use sysx::SysxError;


// bin_to_str tests

#[test]
// Тест конвертации простой бинарной строки в текст.
// Проверяет базовое преобразование ASCII символов.
fn test_bin_to_str_basic() {
    let bin = "01001000 01100101 01101100 01101100 01101111";
    assert_eq!(bin_to_str(bin).unwrap(), "Hello");
}

#[test]
// Тест обработки некорректной бинарной строки.
// Проверяет обработку ошибки при неверном формате.
fn test_bin_to_str_invalid_binary() {
    let bin = "01001000 0110210";
    assert!(matches!(bin_to_str(bin).unwrap_err(), SysxError::ParseIntError(_)));
}

#[test]
// Тест обработки некорректной UTF-8 последовательности.
// Проверяет обработку ошибки при неверной UTF-8 кодировке.
fn test_bin_to_str_invalid_utf8() {
    let bin = "11111111 11111111";
    assert!(matches!(bin_to_str(bin).unwrap_err(), SysxError::InvalidSyntax(_)));
}


// str_to_bin tests

#[test]
// Тест конвертации простого текста в бинарную строку.
// Проверяет базовое преобразование в двоичный формат.
fn test_str_to_bin_basic() {
    let text = "Hi";
    assert_eq!(str_to_bin(text), "01001000 01101001");
}

#[test]
// Тест конвертации пустой строки.
// Проверяет корректность обработки пустого ввода.
fn test_str_to_bin_empty() {
    let text = "";
    assert_eq!(str_to_bin(text), "");
}

#[test]
// Тест конвертации специальных символов.
// Проверяет корректность преобразования не-ASCII символов.
fn test_str_to_bin_special_chars() {
    let text = "!@#";
    assert_eq!(str_to_bin(text), "00100001 01000000 00100011");
}


// is_valid_bin tests

#[test]
// Тест проверки корректной бинарной строки.
// Проверяет валидацию правильного формата.
fn test_is_valid_bin_correct() {
    assert!(is_valid_bin("0100 1000"));
    assert!(is_valid_bin("01001000"));
}

#[test]
// Тест проверки некорректной бинарной строки.
// Проверяет обнаружение недопустимых символов.
fn test_is_valid_bin_incorrect() {
    assert!(!is_valid_bin("0100x1000"));
    assert!(!is_valid_bin(""));
}


// is_valid_bin_strict tests

#[test]
// Тест строгой проверки корректной бинарной строки.
// Проверяет валидацию в строгом режиме.
fn test_is_valid_bin_strict_correct() {
    assert!(is_valid_bin_strict("01001000"));
    assert!(is_valid_bin_strict("0100100001101001"));
}

#[test]
// Тест строгой проверки некорректной бинарной строки.
// Проверяет обнаружение недопустимых форматов в строгом режиме.
fn test_is_valid_bin_strict_incorrect() {
    assert!(!is_valid_bin_strict("0100 1000")); // пробелы не допускаются
    assert!(!is_valid_bin_strict("0100100")); // длина не кратна 8
    assert!(!is_valid_bin_strict("")); // пустая строка
}


// fmt_bin tests

#[test]
// Тест форматирования корректной бинарной строки.
// Проверяет правильное разделение на группы по 8 бит.
fn test_fmt_bin_correct() {
    let input = "0100100001101001";
    assert_eq!(fmt_bin(input).unwrap(), "01001000 01101001");
}

#[test]
// Тест форматирования строки с некорректными символами.
// Проверяет удаление недопустимых символов и форматирование.
fn test_fmt_bin_with_invalid_chars() {
    let input = "01001000xyz01101001";
    assert_eq!(fmt_bin(input).unwrap(), "01001000 01101001");
}

#[test]
// Тест обработки ошибки при неверной длине.
// Проверяет возврат ошибки, если длина не кратна 8.
fn test_fmt_bin_invalid_length() {
    let input = "0100100";
    assert!(matches!(fmt_bin(input).unwrap_err(), SysxError::InvalidSyntax(_)));
}

#[test]
// Тест форматирования пустой строки.
// Проверяет обработку пустого ввода.
fn test_fmt_bin_empty() {
    let input = "";
    assert!(matches!(fmt_bin(input).unwrap_err(), SysxError::InvalidSyntax(_)));
}
