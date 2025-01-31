use std::io::{stdin, Read, Error};
use std::process::Command;


#[cfg(unix)]
pub fn silent_cmd(command: &str) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .unwrap();

    String::from_utf8_lossy(    
        if output.status.success() {
            &output.stdout
        } else {
            &output.stderr
        }
    ).to_string()
}

#[cfg(windows)]
pub fn silent_cmd(command: &str) -> String {
    let output = Command::new("cmd")
        .arg("/c")
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

#[cfg(not(any(unix, windows)))]
fn silent_cmd(command: &str) -> String {
    eprintln!("Sorry! Unsupported OS :(");
}

pub fn cmd(command: &str) -> String {
    let out = silent_cmd(command);
    println!("{out}");
    out
}

pub fn input() -> Result<String, Error> {
    let mut input_text = String::new();
    let _ = stdin()
        .read_to_string(&mut input_text);

    Ok(input_text)
}
