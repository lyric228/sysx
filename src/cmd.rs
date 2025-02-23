use std::process::{Command, Output, Stdio};
use anyhow::Context;

pub use crate::error::{Result, SysxError};


pub fn silent_run(command_line: &str) -> Result<(String, Output)> {
    let trimmed = command_line.trim();

    if trimmed.is_empty() {
        return Err(SysxError::AnyhowError(anyhow::anyhow!("Empty command line")));
    }

    let mut parts = shell_words::split(trimmed)
        .context("Failed to parse command line")
        .map_err(|e| SysxError::AnyhowError(e.into()))?;

    let program = parts.remove(0);
    let args = parts;

    let output: Output = Command::new(&program)
        .args(&args)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .output()
        .with_context(|| format!("Failed to execute command '{}'", command_line))
        .map_err(|e| SysxError::AnyhowError(e))?;

    let result = if output.status.success() {
        output.stdout.clone()
    } else {
        output.stderr.clone()
    };

    let output_str = String::from_utf8(result)
        .map_err(|e| SysxError::FromUtf8Error(e))?;

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
        silent_run(&format!($($arg)*)).map_err(SysxError::from)
    }
}

#[macro_export]
macro_rules! runf {
    ($($arg:tt)*) => {
        run(&format!($($arg)*)).map_err(SysxError::from)
    }
}

pub fn input_buf(buffer: &mut String) -> Result<()> {
    std::io::stdin().read_line(buffer)
        .map_err(|e| SysxError::AnyhowError(anyhow::anyhow!("Failed to read line: {}", e)))?;
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
