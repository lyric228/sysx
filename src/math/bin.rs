use crate::{Result, SysxError};

/// Returns a string containing only binary characters ('0' and '1').
pub fn clean_bin(input: &str) -> String {
    input.chars().filter(|c| *c == '0' || *c == '1').collect()
}

/// Converts a binary string to a UTF-8 string.
/// The binary string must have a length that is a multiple of 8 after cleaning.
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

/// Converts a string to a binary string (byte by byte, separated by spaces).
pub fn str_to_bin(text: &str) -> String {
    text.as_bytes()
        .iter()
        .map(|b| format!("{:08b}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Checks if a string contains only '0', '1', and whitespace characters.
pub fn is_valid_bin(bin: &str) -> bool {
    !bin.is_empty()
        && bin
            .chars()
            .all(|c| c.is_whitespace() || c == '0' || c == '1')
}

/// Strictly validates a binary string: non-empty, contains only '0' and '1' (no whitespace), and length is a multiple of 8.
pub fn is_valid_bin_strict(bin: &str) -> bool {
    let mut count = 0;
    for c in bin.chars() {
        if c.is_whitespace() {
            continue;
        }
        if c == '0' || c == '1' {
            count += 1;
        } else {
            return false;
        }
    }
    count > 0 && count % 8 == 0
}

/// Formats a binary string by adding spaces between bytes.
/// Input string must contain only '0' and '1' (ignoring whitespace) and have a length multiple of 8 after cleaning.
pub fn fmt_bin(bin: &str) -> Result<String> {
    let cleaned = clean_bin(bin);
    if cleaned.is_empty() || cleaned.len() % 8 != 0 {
        return Err(SysxError::InvalidSyntax(
            "Binary string length must be multiple of 8".into(),
        ));
    }

    let formatted = cleaned
        .as_bytes()
        .chunks(8)
        .map(|chunk| std::str::from_utf8(chunk).expect("Cleaned binary string should be valid UTF-8"))
        .collect::<Vec<&str>>()
        .join(" ");

    Ok(formatted)
}
