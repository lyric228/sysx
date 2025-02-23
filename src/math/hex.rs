use crate::{
    Result,
    SysxError,
};


/// Преобразует строку из шестнадцатеричных значений в обычную строку UTF-8.
/// 
/// # Пример
/// ```
/// let hex = "48 65 6C 6C 6F";
/// let s = hex_to_str(hex).unwrap();
/// assert_eq!(s, "Hello");
/// ```
pub fn hex_to_str(hex: &str) -> Result<String> {
    let bytes = hex
        .split_whitespace()
        .map(|s| u8::from_str_radix(s, 16))
        .collect::<std::result::Result<Vec<u8>, _>>()
        .map_err(|e| SysxError::ParseIntError(e))?;

    String::from_utf8(bytes)
        .map_err(|e| SysxError::InvalidSyntax(format!("Invalid UTF-8 sequence: {}", e)))
}

/// Преобразует строку в шестнадцатеричное представление.
/// 
/// # Пример
/// ```
/// let text = "Hello";
/// let hex = str_to_hex(text);
/// assert_eq!(hex, "48 65 6C 6C 6F");
/// ```
pub fn str_to_hex(text: &str) -> String {
    text.as_bytes()
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<String>>()
        .join(" ")
}

/// Проверяет, содержит ли строка только шестнадцатеричные символы и пробельные символы.
/// 
/// # Пример
/// ```
/// assert!(is_valid_hex("48 65 6C 6C 6F"));
/// assert!(!is_valid_hex("48 65 6C GG"));
/// ```
pub fn is_valid_hex(hex: &str) -> bool {
    !hex.is_empty() && hex
        .chars()
        .all(|c| c.is_whitespace() || c.is_ascii_hexdigit())
}

/// Строгая проверка шестнадцатеричной строки.
/// 
/// # Пример
/// ```
/// assert!(is_valid_hex_strict("48656C6C6F"));
/// assert!(!is_valid_hex_strict("48656C6C6")); // нечётное количество символов
/// ```
pub fn is_valid_hex_strict(hex: &str) -> bool {
    if hex.is_empty() {
        return false;
    }
    
    let trimmed: String = hex
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    trimmed.len() % 2 == 0 && trimmed.chars().all(|c| c.is_ascii_hexdigit())
}

/// Форматирует шестнадцатеричную строку, добавляя пробелы каждые 2 символа и удаляя некорректные символы.
/// 
/// # Пример
/// ```
/// let dirty = "48xyz65...6C6C6F";
/// let formatted = fmt_hex(dirty).unwrap();
/// assert_eq!(formatted, "48 65 6C 6C 6F");
/// ```
///
/// Если длина очищенной строки не чётная, возвращается ошибка:
/// ```
/// assert!(fmt_hex("48A").is_err());
/// ```
pub fn fmt_hex(hex: &str) -> Result<String> {
    let cleaned: String = hex
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect();

    if cleaned.is_empty() || cleaned.len() % 2 != 0 {
        return Err(SysxError::InvalidSyntax(
            "Hexadecimal string length must be multiple of 2".to_string()
        ));
    }

    let formatted = cleaned
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ");

    Ok(formatted)
}
