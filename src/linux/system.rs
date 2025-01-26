use std::collections::HashMap;
use crate::fs::File;


pub fn get_info() -> HashMap<String, String> {
    let os_info = File::new("/etc/os-release").read();
    let mut os_map = HashMap::new();

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
