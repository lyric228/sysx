use std::{
    io,
    process::{Command, Output, Stdio},
};

use anyhow::Context;

use crate::{Result, SysxError};

/// Executes a command silently and returns its Output.
pub fn slrun(command_line: &str) -> Result<Output> {
    let trimmed = command_line.trim();

    if trimmed.is_empty() {
        return Err(SysxError::AnyhowError(anyhow::anyhow!(
            "Empty command line"
        )));
    }

    let mut parts = shell_words::split(trimmed)
        .context("Failed to parse command line")
        .map_err(SysxError::AnyhowError)?;

    let program = parts.remove(0);
    let args = parts;

    let output: Output = Command::new(&program)
        .args(&args)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .output()
        .with_context(|| format!("Failed to execute command '{command_line}'"))
        .map_err(SysxError::AnyhowError)?;

    Ok(output)
}

/// Executes a command, prints stdout, and returns its Output.
pub fn run(command: &str) -> Result<Output> {
    let output = slrun(command)?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("{output_str}");

    Ok(output)
}

/// Macro to call `slrun` with a formatted command string.
#[macro_export]
macro_rules! slrunf {
    ($($arg:tt)*) => {
        slrun(&format!($($arg)*))
            .map_err(SysxError::from)
    }
}
pub use slrunf;

/// Macro to call `run` with a formatted command string.
#[macro_export]
macro_rules! runf {
    ($($arg:tt)*) => {
        run(&format!($($arg)*))
            .map_err(SysxError::from)
    }
}
pub use runf;

/// Reads a line from stdin into the provided buffer, removing the newline.
pub fn input_buf(buffer: &mut String) -> Result<()> {
    io::stdin()
        .read_line(buffer)
        .map_err(|e| SysxError::AnyhowError(anyhow::anyhow!("Failed to read line: {}", e)))
        .map(|_| {
            if buffer.ends_with('\n') {
                buffer.pop();
            }
        })
}

/// Reads a line from stdin and returns a new String.
pub fn input() -> Result<String> {
    let mut input_text = String::new();
    input_buf(&mut input_text)?;
    Ok(input_text)
}
