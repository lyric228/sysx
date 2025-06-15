use crate::{
    Result,
    SysxError,
    math::parity::{is_even, is_odd},
};

/// Returns a string containing only hex characters from the input.
pub fn clean(input: &str) -> String {
    input.chars().filter(|c| c.is_ascii_hexdigit()).collect()
}

/// Converts a hex string (with or without separators) to a UTF-8 string.
/// Returns an error if the hex string has odd length or invalid UTF-8.
pub fn decode(hex: &str) -> Result<String> {
    let cleaned = clean(hex);

    if is_odd(cleaned.len()) {
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

    String::from_utf8(bytes).map_err(|e| SysxError::InvalidSyntax(format!("Invalid UTF-8: {e}")))
}

/// Converts a string to a space-separated hexadecimal string.
pub fn encode(text: &str) -> String {
    text.as_bytes()
        .iter()
        .map(|b| format!("{b:02X}"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Checks if a string contains only hex characters and whitespace.
pub fn check(hex: &str) -> bool {
    !hex.is_empty()
        && hex
            .chars()
            .all(|c| c.is_whitespace() || c.is_ascii_hexdigit())
}

/// Checks if a whitespace-cleaned hex string has an even length and consists only of hex digits.
pub fn check_strict(hex: &str) -> bool {
    let trimmed: String = hex.chars().filter(|c| !c.is_whitespace()).collect();
    !trimmed.is_empty() && is_even(trimmed.len()) && trimmed.chars().all(|c| c.is_ascii_hexdigit())
}

/// Formats a string containing hex digits into a space-separated hex string.
/// Returns an error if the cleaned hex string has odd length or is empty.
pub fn format(hex: &str) -> Result<String> {
    let cleaned = clean(hex);
    if cleaned.is_empty() || is_odd(cleaned.len()) {
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
