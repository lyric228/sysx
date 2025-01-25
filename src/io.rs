use std::process::Command;
use std::io::stdin;


pub fn silent_cmd(command: &str) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect(format!("{}", command).as_str());

    String::from_utf8_lossy(    
        if output.status.success() {
            &output.stdout
        } else {
            &output.stderr
        }
    ).to_string()
}

pub fn cmd(command: &str) -> String {
    let out = silent_cmd(command);
    println!("{}", out);
    out
}

pub fn input() -> String {
    let mut input_text = String::new();
    stdin()
        .read_line(&mut input_text)
        .expect("Input reading error");

    input_text.trim().to_string()
}
