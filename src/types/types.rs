use std::any::Any;
use regex::Regex;


pub fn simplify_nonlist_type(type_str: &str) -> &str {
    type_str
        .split("::")
        .last()
        .unwrap_or("unknown")
}

pub fn get_type<T: Any>(_: &T) -> String {
    String::from(std::any::type_name::<T>())
}

pub fn is_list_like(type_str: &str) -> bool {
    type_str.contains("<") || type_str.contains(">")
}

pub fn simplify_type<'a>(type_str: &'a str) -> String {
    if !is_list_like(&type_str) {
        return simplify_nonlist_type(type_str).to_string();
    }
    
    let re = Regex::new(r"([a-zA-Z_][a-zA-Z0-9_]*::)+").unwrap();
    let mut result = String::new();
    let mut inside_angle_brackets = 0;
    let mut current_token = String::new();

    for c in type_str.chars() {
        if c == '<' {
            inside_angle_brackets += 1;
            current_token.push(c);
        } else if c == '>' {
            inside_angle_brackets -= 1;
            current_token.push(c);
        } else if c == ',' && inside_angle_brackets == 0 {
            let simplified_token = re.replace_all(&current_token, "");
            if !result.is_empty() {
                result.push(' ');
            }
            result.push_str(&simplified_token);
            current_token.clear();
        } else {
            current_token.push(c);
        }
    }

    if !current_token.is_empty() {
        let simplified_token = re.replace_all(&current_token, "");
        if !result.is_empty() {
            result.push(' ');
        }
        result.push_str(&simplified_token);
    }

    result.trim().to_string()
}
