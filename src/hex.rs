pub use crate::error::{Result, SysxError};


pub fn hex_to_str(hex: &str) -> Result<String> {
    let hex = hex.to_uppercase();

    hex.split_whitespace()
        .map(|s| u8::from_str_radix(s, 16).map_err(SysxError::from)) // Преобразуем ParseIntError в SysxError
        .collect::<Result<Vec<u8>>>()
        .and_then(|bytes| String::from_utf8(bytes).map_err(SysxError::from))
}

pub fn str_to_hex(text: &str) -> String {
    text.as_bytes()
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn is_valid_hex(hex: &str) -> bool {
    hex.chars()
        .filter(|c| !c.is_whitespace())
        .all(|c| c.is_digit(16))
}

pub fn is_valid_hex_strict(hex: &str) -> bool {
    let trimmed = hex.replace(" ", "");
    trimmed.len() % 2 == 0 && trimmed.chars().all(|c| c.is_digit(16))
}
