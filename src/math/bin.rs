use crate::{
    Result,
    SysxError,
};


/// Преобразует строку из бинарных значений в обычную строку UTF-8.
///
/// # Пример
/// ```
/// let bin = "01001000 01100101 01101100 01101100 01101111";
/// let s = bin_to_str(bin).unwrap();
/// assert_eq!(s, "Hello");
/// ```
pub fn bin_to_str(bin: &str) -> Result<String> {
    let bytes = bin
        .split_whitespace()
        .map(|s| u8::from_str_radix(s, 2))
        .collect::<std::result::Result<Vec<u8>, _>>()
        .map_err(|e| SysxError::ParseIntError(e))?;

    String::from_utf8(bytes)
        .map_err(|e| SysxError::InvalidSyntax(format!("Invalid UTF-8 sequence: {}", e)))
}

/// Преобразует строку в бинарное представление.
///
/// # Пример
/// ```
/// let text = "Hello";
/// let bin = str_to_bin(text);
/// assert_eq!(bin, "01001000 01100101 01101100 01101100 01101111");
/// ```
pub fn str_to_bin(text: &str) -> String {
    text.as_bytes()
        .iter()
        .map(|b| format!("{:08b}", b))
        .collect::<Vec<String>>()
        .join(" ")
}

/// Проверяет, содержит ли строка только символы '0', '1' и пробельные символы.
///
/// # Пример
/// ```
/// assert!(is_valid_bin("01001000 01100101"));
/// assert!(!is_valid_bin("01001000 01100101x"));
/// ```
pub fn is_valid_bin(bin: &str) -> bool {
    !bin.is_empty() && bin
        .chars()
        .all(|c| c.is_whitespace() || c == '0' || c == '1')
}

/// Строгая проверка бинарной строки.
///
/// # Пример
/// ```
/// assert!(is_valid_bin_strict("0100100001100101"));
/// assert!(!is_valid_bin_strict("010010000110010")); // число символов не кратно 8
/// assert!(!is_valid_bin_strict("01001000 01100101")); // пробелы не допускаются
/// ```
pub fn is_valid_bin_strict(bin: &str) -> bool {
    if bin.is_empty() {
        return false;
    }
    
    let trimmed: String = bin
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();

    trimmed.len() % 8 == 0 && trimmed.chars().all(|c| c == '0' || c == '1')
}

/// Форматирует двоичную строку, добавляя пробелы каждые 8 бит и удаляя некорректные символы.
///
/// # Пример
/// ```
/// let dirty = "01001000xyz01100101...01101100   0110110001101111";
/// let formatted = fmt_bin(dirty).unwrap();
/// assert_eq!(formatted, "01001000 01100101 01101100 01101100 01101111");
/// ```
///
/// Если длина очищенной строки не кратна 8, возвращается ошибка:
/// ```
/// assert!(fmt_bin("0100100").is_err());
/// ```
pub fn fmt_bin(bin: &str) -> Result<String> {
    let cleaned: String = bin
        .chars()
        .filter(|c| *c == '0' || *c == '1')
        .collect();

    if cleaned.is_empty() || cleaned.len() % 8 != 0 {
        return Err(SysxError::InvalidSyntax(
            "Binary string length must be multiple of 8".to_string()
        ));
    }

    let formatted = cleaned
        .chars()
        .collect::<Vec<char>>()
        .chunks(8)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ");

    Ok(formatted)
}
