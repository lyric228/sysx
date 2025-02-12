use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;


#[derive(Debug)]
pub enum OsInfoError {
    IoError(std::io::Error),
    ParseError(String),
}

impl fmt::Display for OsInfoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OsInfoError::IoError(e) => write!(f, "IO error: {}", e),
            OsInfoError::ParseError(e) => write!(f, "Parse error: {}", e),
        }
    }
}

impl Error for OsInfoError {}

impl From<std::io::Error> for OsInfoError {
    fn from(e: std::io::Error) -> Self {
        OsInfoError::IoError(e)
    }
}


#[cfg(unix)]
/// Получает информацию об ОС из /etc/os-release
/// Возвращает Result с HashMap или OsInfoError
pub fn get_os_info() -> Result<HashMap<String, String>, OsInfoError> {
    let content = fs::read_to_string("/etc/os-release")?;
    parse_os_release(&content)
}

/// Парсит содержимое os-release файла
fn parse_os_release(content: &str) -> Result<HashMap<String, String>, OsInfoError> {
    let mut os_map = HashMap::new();
    
    for (line_num, line) in content.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() != 2 {
            return Err(OsInfoError::ParseError(format!(
                "Invalid format at line {}: '{}'",
                line_num + 1,
                line
            )));
        }

        let (key, value) = (parts[0].trim(), parts[1].trim().trim_matches('"'));
        if key.is_empty() {
            return Err(OsInfoError::ParseError(format!(
                "Empty key at line {}: '{}'",
                line_num + 1,
                line
            )));
        }

        os_map.insert(key.to_string(), value.to_string());
    }

    if os_map.is_empty() {
        Err(OsInfoError::ParseError("Empty os-release file".to_string()))
    } else {
        Ok(os_map)
    }
}
