use std::process::{Command, Output};
use crate::error::Result;
use std::io::Read;


pub fn run(program: &str, arg: &str, command: &str) -> Result<String> {
    let output: Output = Command::new(program)
        .arg(arg)
        .arg(command)
        .output()?;
        
    let result = if output.status.success() {
        output.stdout
    } else {
        output.stderr
    };

    Ok(String::from_utf8_lossy(&result).to_string())
}

pub fn sh(command: &str) -> Result<String> {
    let out = run("sh", "-c", command)?;
    println!("{out}");
    Ok(out)
}

pub fn input() -> Result<String> {
    let mut input_text = String::new();
    std::io::stdin().read_to_string(&mut input_text)?;
    Ok(input_text)
}

pub fn args() -> Vec<String> {
    std::env::args().collect()
}
