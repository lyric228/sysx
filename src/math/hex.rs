use crate::{Result, SysxError};

const HEX_CHARS_UPPER: [u8; 16] = *b"0123456789ABCDEF";
const HEX_CHARS_LOWER: [u8; 16] = *b"0123456789abcdef";
const TO_UPPER_MASK: u8 = 0b1101_1111;
const TO_LOWER_MASK: u8 = 0b0010_0000;

/// Fast conversion of hex string to uppercase
pub fn to_uppercase_fast(hex: &str) -> String {
    let bytes = hex.as_bytes();
    let mut result = String::with_capacity(bytes.len());
    
    unsafe {
        let out = result.as_mut_vec();
        out.set_len(bytes.len());
        
        for i in 0..bytes.len() {
            let mut b = bytes[i];
            if (b >= b'a') && (b <= b'f') {
                b &= TO_UPPER_MASK;
            }
            out[i] = b;
        }
    }
    result
}

/// Fast conversion of hex string to lowercase
pub fn to_lowercase_fast(hex: &str) -> String {
    let bytes = hex.as_bytes();
    let mut result = String::with_capacity(bytes.len());
    
    unsafe {
        let out = result.as_mut_vec();
        out.set_len(bytes.len());
        
        for i in 0..bytes.len() {
            let mut b = bytes[i];
            if (b >= b'A') && (b <= b'F') {
                b |= TO_LOWER_MASK;
            }
            out[i] = b;
        }
    }
    result
}

/// Lookup table for ultra-fast case conversion
static HEX_CASE_TABLE: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i: usize = 0;
    while i < 256 {
        let byte = i as u8;
        table[i] = match byte {
            b'0'..=b'9' => byte,
            b'a'..=b'f' => byte - 32,
            b'A'..=b'F' => byte + 32,
            _ => byte,
        };
        i += 1;
    }
    table
};

/// Fastest case conversion method using lookup table
pub fn convert_hex_case(hex: &str, to_upper: bool) -> String {
    let bytes = hex.as_bytes();
    let mut result = String::with_capacity(bytes.len());
    
    unsafe {
        let out = result.as_mut_vec();
        out.set_len(bytes.len());
        
        for i in 0..bytes.len() {
            let b = bytes[i] as usize;
            out[i] = if to_upper {
                HEX_CASE_TABLE[b] & TO_UPPER_MASK
            } else {
                HEX_CASE_TABLE[b] | TO_LOWER_MASK
            };
        }
    }
    result
}

/// Returns a string containing only hex characters from the input
pub fn clean(input: &str) -> String {
    let mut cleaned = String::with_capacity(input.len());
    for c in input.chars() {
        if c.is_ascii_hexdigit() {
            cleaned.push(c);
        }
    }
    cleaned
}

/// Converts hex string to UTF-8 string with proper error handling
pub fn decode(hex: &str) -> Result<String> {
    let mut cleaned = String::with_capacity(hex.len());
    let mut is_valid = true;
    
    for c in hex.chars() {
        if c.is_ascii_hexdigit() {
            cleaned.push(c);
        } else if !c.is_whitespace() {
            is_valid = false;
        }
    }

    let len = cleaned.len();
    if !is_valid {
        return Err(SysxError::InvalidSyntax("Non-hex character detected".into()));
    }
    if len % 2 != 0 {
        return Err(SysxError::InvalidSyntax("Hex string must have even length".into()));
    }

    let mut bytes = Vec::with_capacity(len / 2);
    let mut chars = cleaned.chars();
    
    while let (Some(c1), Some(c2)) = (chars.next(), chars.next()) {
        let hi = c1.to_digit(16).ok_or_else(|| {
            SysxError::InvalidSyntax(format!("Invalid hex character: {}", c1))
        })? as u8;
        let lo = c2.to_digit(16).ok_or_else(|| {
            SysxError::InvalidSyntax(format!("Invalid hex character: {}", c2))
        })? as u8;
        bytes.push((hi << 4) | lo);
    }

    String::from_utf8(bytes).map_err(|e| SysxError::InvalidSyntax(format!("Invalid UTF-8: {e}")))
}

/// Converts string to space-separated hexadecimal string
pub fn encode(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut result = String::with_capacity(bytes.len() * 3);
    
    for (i, &byte) in bytes.iter().enumerate() {
        if i > 0 {
            result.push(' ');
        }
        result.push(HEX_CHARS_UPPER[(byte >> 4) as usize] as char);
        result.push(HEX_CHARS_UPPER[(byte & 0x0F) as usize] as char);
    }
    result
}

/// Checks if a string contains only hex characters and whitespace
pub fn check(hex: &str) -> bool {
    !hex.is_empty() && hex.chars().all(|c| c.is_whitespace() || c.is_ascii_hexdigit())
}

/// Checks if a whitespace-cleaned hex string has even length and valid hex digits
pub fn check_strict(hex: &str) -> bool {
    let mut length = 0;
    let mut is_valid = true;
    
    for c in hex.chars() {
        if c.is_ascii_hexdigit() {
            length += 1;
        } else if !c.is_whitespace() {
            is_valid = false;
            break;
        }
    }
    
    is_valid && length > 0 && length % 2 == 0
}

/// Formats hex digits into space-separated hex string
pub fn format(hex: &str) -> Result<String> {
    let cleaned = clean(hex);
    let len = cleaned.len();
    
    if len == 0 {
        return Err(SysxError::InvalidSyntax("Empty hex string".into()));
    }
    if len % 2 != 0 {
        return Err(SysxError::InvalidSyntax(
            "Hexadecimal string length must be a multiple of 2".into(),
        ));
    }
    
    let mut result = String::with_capacity(len + len / 2);
    let mut chars = cleaned.chars();
    
    for i in 0..(len / 2) {
        if i > 0 {
            result.push(' ');
        }
        result.push(chars.next().unwrap());
        result.push(chars.next().unwrap());
    }
    
    Ok(result)
}
