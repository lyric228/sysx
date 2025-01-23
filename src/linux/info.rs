use crate::types::BHashMap;
use std::fs;


pub fn get_info() -> BHashMap<String, String> {
    let os_info = fs::read_to_string("/etc/os-release")
        .expect("Не удалось прочитать файл /etc/os-release");

    let mut os_map = BHashMap::new();

    for line in os_info.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let mut parts = line.splitn(2, "=");
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            let value = value.trim_matches('"');
            os_map.insert(key.to_string(), value.to_string());
        }
    }

    os_map
}

pub fn get_info_by_key<T: AsRef<str>>(key: T) -> String {
    if let Some(value) = get_info().get(key.as_ref()) {
        value.to_string()
    } else {
        String::from("undefined")
    }
}
