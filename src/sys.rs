use std::process::Command;
use std::ffi::OsStr;
use std::io::stdin;


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
