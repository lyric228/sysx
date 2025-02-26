use crate::{Result, SysxError};

/// Возвращает строку, содержащую только бинарные символы (0 и 1)
pub fn clean_bin(input: &str) -> String {
    input.chars().filter(|c| *c == '0' || *c == '1').collect()
}

/// Преобразует бинарную строку в UTF-8 строку
pub fn bin_to_str(bin: &str) -> Result<String> {
    let cleaned = clean_bin(bin);

    if cleaned.len() % 8 != 0 {
        return Err(SysxError::InvalidSyntax(
            "Binary string must have length multiple of 8".into(),
        ));
    }

    let bytes = (0..cleaned.len())
        .step_by(8)
        .map(|i| {
            let byte_str = &cleaned[i..i + 8];
            u8::from_str_radix(byte_str, 2)
        })
        .collect::<std::result::Result<Vec<u8>, _>>()
        .map_err(SysxError::ParseIntError)?;

    String::from_utf8(bytes).map_err(|e| SysxError::InvalidSyntax(format!("Invalid UTF-8: {}", e)))
}

/// Преобразует строку в бинарный формат (байт за байтом, разделённые пробелами)
pub fn str_to_bin(text: &str) -> String {
    text.as_bytes()
        .iter()
        .map(|b| format!("{:08b}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Проверяет что строка содержит только 0, 1 и пробельные символы
pub fn is_valid_bin(bin: &str) -> bool {
    !bin.is_empty()
        && bin
            .chars()
            .all(|c| c.is_whitespace() || c == '0' || c == '1')
}

/// Строгая проверка бинарной строки (без пробелов и длина кратна 8)
pub fn is_valid_bin_strict(bin: &str) -> bool {
    let trimmed: String = bin.chars().filter(|c| !c.is_whitespace()).collect();
    !trimmed.is_empty() && trimmed.len() % 8 == 0 && trimmed.chars().all(|c| c == '0' || c == '1')
}

/// Форматирует бинарную строку, разделяя байты пробелами
pub fn fmt_bin(bin: &str) -> Result<String> {
    let cleaned = clean_bin(bin);
    if cleaned.is_empty() || cleaned.len() % 8 != 0 {
        return Err(SysxError::InvalidSyntax(
            "Binary string length must be multiple of 8".into(),
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
