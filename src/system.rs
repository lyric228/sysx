use std::collections::HashMap;
use std::process::Command;
use std::ffi::OsStr;
use std::io::stdin;
use std::fs;


pub fn silent_cmd<S: AsRef<OsStr>>(command: S) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Не удалось выполнить команду");

    String::from_utf8_lossy(    
        if output.status.success() {
            &output.stdout
        } else {
            &output.stderr
        }
    ).to_string()
}

pub fn cmd<S: AsRef<OsStr>>(command: S) -> String {
    let out = silent_cmd(command);
    println!("{}", out);
    out
}

pub fn input() -> String {
    let mut input_text = String::new();
    stdin()
        .read_line(&mut input_text)
        .expect("Не удалось получить ввод");

    input_text.trim().to_string()
}

pub fn info() -> HashMap<String, String> {
    let os_info = fs::read_to_string("/etc/os-release")
        .expect("Не удалось прочитать файл /etc/os-release");

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
