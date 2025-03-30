use crate::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use std::any::Any;

/// Regex for removing namespace qualifiers
static QUALIFIER_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"([a-zA-Z_][a-zA-Z0-9_]*::)+").expect("Failed to compile qualifier regex")
});

/// Simplifies a non-generic type by removing namespace qualifiers
pub fn simplify_nonlist_type(type_str: &str) -> Result<String> {
    Ok(type_str.split("::").last().unwrap_or(type_str).to_string())
}

/// Gets the type name of a value using std::any
pub fn get_type<T: Any>(_: &T) -> String {
    std::any::type_name::<T>().to_owned()
}

/// Checks if a type string represents a generic or collection type
pub fn is_list_like(type_str: &str) -> bool {
    if type_str.contains('<') || type_str.contains('>') {
        return true;
    }

    let trimmed = type_str.trim();
    trimmed.starts_with('[') && trimmed.ends_with(']')
}

/// Simplifies a type string by removing namespace qualifiers, handling generics
pub fn simplify_type(type_str: &str) -> Result<String> {
    if !is_list_like(type_str) {
        return simplify_nonlist_type(type_str);
    }

    let mut result: String = String::with_capacity(type_str.len());
    let mut token: String = String::with_capacity(type_str.len() / 2);
    let mut bracket_depth: i32 = 0;

    for c in type_str.chars() {
        match c {
            '<' => {
                bracket_depth += 1;
                token.push(c);
            }
            '>' => {
                bracket_depth = bracket_depth.saturating_sub(1);
                token.push(c);
            }
            ',' if bracket_depth == 0 => {
                if !result.is_empty() {
                    result.push_str(", ");
                }
                result.push_str(&QUALIFIER_RE.replace_all(&token, ""));
                token.clear();
            }
            _ => token.push(c),
        }
    }
    if !token.is_empty() {
        if !result.is_empty() {
            result.push_str(", ");
        }
        result.push_str(&QUALIFIER_RE.replace_all(&token, ""));
    }

    Ok(result)
}
