use std::{io::Read, process::{Command, Output}};
use anyhow::Context;

use crate::error::Result;


pub fn silent_run(command_line: &str) -> Result<(String, Output)> {
    let trimmed = command_line.trim();

    if trimmed.is_empty() {
        return Err(anyhow::anyhow!("Empty command line").into());
    }

    let mut parts = shell_words::split(trimmed)
        .context("Failed to parse command line")?;

    let program = parts.remove(0);
    let args = parts;

    let output: Output = Command::new(&program)
        .args(&args)
        .output()
        .with_context(|| format!(
            "Failed to execute command '{}'", 
            command_line
        ))?;

    let result = if output.status.success() {
        output.clone().stdout
    } else {
        output.clone().stderr
    };

    let output_str = String::from_utf8(result)
        .map_err(|e| anyhow::anyhow!(
            "Invalid UTF-8 in output: {}", e
        ))?;

    Ok((output_str, output))
}

pub fn run(command: &str) -> Result<(String, Output)> {
    let output = silent_run(command)?;
    println!("{}", output.0);
    Ok(output)
}

#[macro_export]
macro_rules! silent_runf {
    ($($arg:tt)*) => {
        silent_run(&format!($($arg)*))
    }
}

#[macro_export]
macro_rules! runf {
    ($($arg:tt)*) => {
        run(&format!($($arg)*))
    }
}

pub fn input_buf(buffer: &mut String) -> Result<()> {
    std::io::stdin().read_line(buffer)
        .map_err(|e| anyhow::anyhow!("Failed to read line: {}", e))?;
    if buffer.ends_with('\n') {
        buffer.pop();
    }
    Ok(())
}

pub fn input() -> Result<String> {
    let mut input_text = String::new();
    input_buf(&mut input_text)?;
    Ok(input_text)
}

pub fn get_args() -> Vec<String> {
    std::env::args().collect()
}
