use crate::{Result, SysxError};

/// Returns a string containing only binary characters ('0' and '1')
pub fn clean(input: &str) -> String {
    let mut cleaned = String::with_capacity(input.len());
    for c in input.chars() {
        if c == '0' || c == '1' {
            cleaned.push(c);
        }
    }
    cleaned
}

/// Converts a binary string to a UTF-8 string
pub fn decode(bin: &str) -> Result<String> {
    let mut cleaned = String::with_capacity(bin.len());
    let mut is_valid = true;
    
    for c in bin.chars() {
        if c == '0' || c == '1' {
            cleaned.push(c);
        } else if !c.is_whitespace() {
            is_valid = false;
        }
    }

    let len = cleaned.len();
    if !is_valid {
        return Err(SysxError::InvalidSyntax("Non-binary character detected".into()));
    }
    if len % 8 != 0 {
        return Err(SysxError::InvalidSyntax(
            "Binary string must have length multiple of 8".into(),
        ));
    }

    let mut bytes = Vec::with_capacity(len / 8);
    let mut chars = cleaned.chars();
    
    while let Some(_) = chars.next() {
        let mut byte = 0;
        for i in 0..8 {
            if let Some(c) = chars.next() {
                let bit = match c {
                    '1' => 1,
                    '0' => 0,
                    _ => unreachable!(),
                };
                byte |= bit << (7 - i);
            } else {
                break;
            }
        }
        bytes.push(byte);
    }

    String::from_utf8(bytes).map_err(|e| SysxError::InvalidSyntax(format!("Invalid UTF-8: {e}")))
}

/// Converts a string to a space-separated binary string
pub fn encode(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut result = String::with_capacity(bytes.len() * 9);
    
    for (i, &byte) in bytes.iter().enumerate() {
        if i > 0 {
            result.push(' ');
        }
        for shift in (0..8).rev() {
            result.push(if (byte >> shift) & 1 == 1 { '1' } else { '0' });
        }
    }
    result
}

/// Checks if a string contains only '0', '1', and whitespace characters
pub fn is_valid(bin: &str) -> bool {
    !bin.is_empty() && bin.chars().all(|c| c.is_whitespace() || c == '0' || c == '1')
}

/// Strictly validates a binary string
pub fn is_valid_strict(bin: &str) -> bool {
    let mut count = 0;
    let mut is_valid = true;
    
    for c in bin.chars() {
        if c == '0' || c == '1' {
            count += 1;
        } else if !c.is_whitespace() {
            is_valid = false;
            break;
        }
    }
    
    is_valid && count > 0 && count % 8 == 0
}

/// Formats a binary string by adding spaces between bytes
pub fn format(bin: &str) -> Result<String> {
    let cleaned = clean(bin);
    let len = cleaned.len();
    
    if len == 0 {
        return Err(SysxError::InvalidSyntax("Empty binary string".into()));
    }
    if len % 8 != 0 {
        return Err(SysxError::InvalidSyntax(
            "Binary string length must be multiple of 8".into(),
        ));
    }
    
    let mut result = String::with_capacity(len + len / 8);
    let mut chars = cleaned.chars();
    
    for i in 0..(len / 8) {
        if i > 0 {
            result.push(' ');
        }
        for _ in 0..8 {
            result.push(chars.next().unwrap());
        }
    }
    
    Ok(result)
}
