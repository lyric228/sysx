use once_cell::sync::Lazy;
use crate::error::Error;
use std::any::Any;
use regex::Regex;

/// A static variable that compiles a regular expression to remove namespace qualifiers.
/// Using Lazy ensures that the regex is compiled only once.
static QUALIFIER_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"([a-zA-Z_][a-zA-Z0-9_]*::)+")
        .expect("Failed to compile Regex for qualifiers")
});

/// Transforms a type string for cases when it does not contain generic parameters.
///
/// If the string contains generic indicators ("<" or ">"), the function returns 
/// a `ValidationError`.
///
/// # Arguments
///
/// * `type_str` - The type represented as a string.
///
/// # Returns
///
/// Returns the simplified type name without namespace qualifiers.
///
/// # Example
/// ```
/// use sysx::types::{simplify_nonlist_type, Error};
/// 
/// let simple = simplify_nonlist_type("std::string::String").unwrap();
/// assert_eq!(simple, "String");
/// ```
pub fn simplify_nonlist_type(type_str: &str) -> Result<String, Error> {
    if is_list_like(type_str) {
        return Err(Error::ValidationError {
            expected: "non-generic type",
            actual: type_str.to_string(),
            context: Some("type simplification requires base type".into()),
        });
    }

    Ok(type_str
        .split("::")
        .last()
        .unwrap_or("unknown")
        .to_string())
}

/// Returns the type name of the given instance.
///
/// Uses Rust's std::any mechanism to obtain a string representing the type `T`.
///
/// # Arguments
///
/// * `_` - A reference to a value of type `T`.
///
/// # Example
/// ```
/// use sysx::types::get_type;
/// 
/// let t = 10;
/// let type_name = get_type(&t);
/// assert_eq!(type_name, "i32");
/// ```
pub fn get_type<T: Any>(_: &T) -> String {
    std::any::type_name::<T>().to_owned()
}

/// Checks whether the type string indicates a generic or list-like type.
///
/// In addition to looking for generic markers ('<' or '>'), this improved version
/// also checks for common list-like patterns such as prefix "Vec<", presence of "[" and "]"
/// which are typical for arrays or slices.
///
/// # Arguments
///
/// * `type_str` - The type represented as a string.
///
/// # Returns
///
/// Returns `true` if the type appears to be list-like, otherwise `false`.
///
/// # Example
/// ```
/// use sysx::types::is_list_like;
/// 
/// assert!(is_list_like("Vec<i32>"));
/// assert!(is_list_like("[i32; 5]"));
/// assert!(!is_list_like("std::string::String"));
/// ```
pub fn is_list_like(type_str: &str) -> bool {
    // Check for generic markers
    if type_str.contains('<') || type_str.contains('>') {
        return true;
    }
    
    let trimmed = type_str.trim();
    
    if trimmed.starts_with("Vec<") || (trimmed.starts_with('[') && trimmed.ends_with(']')) {
        return true;
    }
    
    false
}

/// Simplifies a type string by removing namespace qualifiers and processing generic parameters.
///
/// If the supplied type string does not contain generic parameters ("<" or ">"),
/// it calls `simplify_nonlist_type`.
///
/// For types with generic parameters, the function splits by commas outside of nested
/// angle brackets and removes namespace qualifiers from each token using a regular expression.
///
/// In case nested generics become too complex, additional checks could be added
/// to return a `NestedGenerics` error.
///
/// # Arguments
///
/// * `type_str` - The type represented as a string.
///
/// # Returns
///
/// Returns the simplified string representation of the type.
///
/// # Example
/// ```
/// use sysx::types::simplify_type;
/// 
/// let simplified = simplify_type("std::vec::Vec<my::custom::Type>").unwrap();
/// assert_eq!(simplified, "Vec<Type>");
/// ```
pub fn simplify_type<'a>(type_str: &'a str) -> Result<String, Error> {
    if !is_list_like(type_str) {
        return simplify_nonlist_type(type_str);
    }

    let mut current_token = String::new();
    let mut result = String::new();
    let mut inside_angle_brackets: i32 = 0;

    for c in type_str.chars() {
        match c {
            '<' => {
                inside_angle_brackets += 1;
                current_token.push(c);
            }
            '>' => {
                inside_angle_brackets = inside_angle_brackets.saturating_sub(1);
                current_token.push(c);
            }
            ',' if inside_angle_brackets == 0 => {
                let simplified_token = QUALIFIER_RE.replace_all(&current_token, "");
                if !result.is_empty() {
                    result.push_str(", ");
                }
                result.push_str(&simplified_token);
                current_token.clear();
            }
            _ => {
                current_token.push(c);
            }
        }
    }

    if !current_token.is_empty() {
        let simplified_token = QUALIFIER_RE.replace_all(&current_token, "");
        if !result.is_empty() {
            result.push_str(", ");
        }
        result.push_str(&simplified_token);
    }

    Ok(result.trim().to_string())
}
