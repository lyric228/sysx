use crate::{Result, SysxError};

/// Возвращает строку, содержащую только hex-символы из входной строки.
pub fn clean_hex(input: &str) -> String {
    input.chars().filter(|c| c.is_ascii_hexdigit()).collect()
}
// TODO: Error enum for hex and bin and other fixes and num mod
/// Преобразует hex-строку в UTF-8 строку.
/// Поддерживаются строки с разделителями или без них.
pub fn hex_to_str(hex: &str) -> Result<String> {
    let cleaned = clean_hex(hex);

    if cleaned.len() % 2 != 0 {
        return Err(SysxError::InvalidSyntax(
            "Hex string must have even length".into(),
        ));
    }

    let bytes = (0..cleaned.len())
        .step_by(2)
        .map(|i| {
            let byte_str = &cleaned[i..i + 2];
            u8::from_str_radix(byte_str, 16)
        })
        .collect::<std::result::Result<Vec<u8>, _>>()
        .map_err(SysxError::ParseIntError)?;
    String::from_utf8(bytes).map_err(|e| SysxError::InvalidSyntax(format!("Invalid UTF-8: {}", e)))
}

/// Преобразует строку в шестнадцатеричный формат (байт за байтом, разделённые пробелами).
pub fn str_to_hex(text: &str) -> String {
    text.as_bytes()
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Проверяет, что строка содержит только hex-символы и пробельные символы.
pub fn is_valid_hex(hex: &str) -> bool {
    !hex.is_empty()
        && hex
            .chars()
            .all(|c| c.is_whitespace() || c.is_ascii_hexdigit())
}

/// Проверяет hex-строку без пробелов на чётное количество символов.
pub fn is_valid_hex_strict(hex: &str) -> bool {
    let trimmed: String = hex.chars().filter(|c| !c.is_whitespace()).collect();
    !trimmed.is_empty() && trimmed.len() % 2 == 0 && trimmed.chars().all(|c| c.is_ascii_hexdigit())
}

/// Форматирует строку, оставляя только hex-символы и разделяя байты пробелами.
pub fn fmt_hex(hex: &str) -> Result<String> {
    let cleaned = clean_hex(hex);
    if cleaned.is_empty() || cleaned.len() % 2 != 0 {
        return Err(SysxError::InvalidSyntax(
            "Hexadecimal string length must be a multiple of 2".into(),
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
