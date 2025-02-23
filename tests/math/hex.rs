use sysx::SysxError;
use sysx::hex::*;


// hex_to_str tests

#[test]
// Тест конвертации простой шестнадцатеричной строки в текст.
// Проверяет базовое преобразование ASCII символов.
fn test_hex_to_str_basic() {
    let hex = "48 65 6C 6C 6F";
    assert_eq!(hex_to_str(hex).unwrap(), "Hello");
}

#[test]
// Тест обработки некорректной hex строки.
// Проверяет обработку ошибки при неверном формате.
fn test_hex_to_str_invalid_hex() {
    let hex = "48 6G";
    assert!(matches!(hex_to_str(hex).unwrap_err(), SysxError::ParseIntError(_)));
}

#[test]
// Тест обработки некорректной UTF-8 последовательности.
// Проверяет обработку ошибки при неверной UTF-8 кодировке.
fn test_hex_to_str_invalid_utf8() {
    let hex = "FF FF";
    assert!(matches!(hex_to_str(hex).unwrap_err(), SysxError::InvalidSyntax(_)));
}


// str_to_hex tests

#[test]
// Тест конвертации простого текста в hex строку.
// Проверяет базовое преобразование в шестнадцатеричный формат.
fn test_str_to_hex_basic() {
    let text = "Hi";
    assert_eq!(str_to_hex(text), "48 69");
}

#[test]
// Тест конвертации пустой строки.
// Проверяет корректность обработки пустого ввода.
fn test_str_to_hex_empty() {
    let text = "";
    assert_eq!(str_to_hex(text), "");
}

#[test]
// Тест конвертации специальных символов.
// Проверяет корректность преобразования не-ASCII символов.
fn test_str_to_hex_special_chars() {
    let text = "!@#";
    assert_eq!(str_to_hex(text), "21 40 23");
}


// is_valid_hex tests

#[test]
// Тест проверки корректной hex строки.
// Проверяет валидацию правильного формата.
fn test_is_valid_hex_correct() {
    assert!(is_valid_hex("48 65"));
    assert!(is_valid_hex("4865"));
}

#[test]
// Тест проверки некорректной hex строки.
// Проверяет обнаружение недопустимых символов.
fn test_is_valid_hex_incorrect() {
    assert!(!is_valid_hex("48 6G"));
    assert!(!is_valid_hex(""));
}


// is_valid_hex_strict tests

#[test]
// Тест строгой проверки корректной hex строки.
// Проверяет валидацию в строгом режиме.
fn test_is_valid_hex_strict_correct() {
    assert!(is_valid_hex_strict("4865"));
    assert!(is_valid_hex_strict("48656C6C6F"));
}

#[test]
// Тест строгой проверки некорректной hex строки.
// Проверяет обнаружение недопустимых форматов в строгом режиме.
fn test_is_valid_hex_strict_incorrect() {
    assert!(!is_valid_hex_strict("486")); // нечетная длина
    assert!(!is_valid_hex_strict("")); // пустая строка
    assert!(!is_valid_hex_strict("4G65")); // недопустимые символы
}


// fmt_hex tests

#[test]
// Тест форматирования корректной hex строки.
// Проверяет правильное разделение на группы по 2 символа.
fn test_fmt_hex_correct() {
    let input = "4865";
    assert_eq!(fmt_hex(input).unwrap(), "48 65");
}

#[test]
// Тест форматирования строки с некорректными символами.
// Проверяет удаление недопустимых символов и форматирование.
fn test_fmt_hex_with_invalid_chars() {
    let input = "48xyz65";
    assert_eq!(fmt_hex(input).unwrap(), "48 65");
}

#[test]
// Тест обработки ошибки при неверной длине.
// Проверяет возврат ошибки, если длина нечетная.
fn test_fmt_hex_invalid_length() {
    let input = "48A";
    assert!(matches!(fmt_hex(input).unwrap_err(), SysxError::InvalidSyntax(_)));
}

#[test]
// Тест форматирования пустой строки.
// Проверяет обработку пустого ввода.
fn test_fmt_hex_empty() {
    let input = "";
    assert!(matches!(fmt_hex(input).unwrap_err(), SysxError::InvalidSyntax(_)));
}


// Round-trip tests

#[test]
// Тест полного цикла преобразования: текст -> hex -> текст.
// Проверяет сохранение данных при двойном преобразовании.
fn test_hex_round_trip() {
    let original = "Hello, World!";
    let hex = str_to_hex(original);
    let result = hex_to_str(&hex).unwrap();
    assert_eq!(original, result);
}
