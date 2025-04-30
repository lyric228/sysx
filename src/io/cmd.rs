use std::process::{Command, Output, Stdio};
use anyhow::Context;
use crate::{Result, SysxError};

/// Executes a command silently, without printing output to the console.
///
/// Parses the command string into a program and arguments.
/// Returns stdout on success, stderr on failure.
///
/// # Returns
/// A tuple containing the output string (stdout or stderr) and the full Output object.
pub fn slrun(command_line: &str) -> Result<(String, Output)> {
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

    let result = if output.status.success() {
        output.stdout.clone()
    } else {
        output.stderr.clone()
    };

    let output_str = String::from_utf8(result).map_err(SysxError::FromUtf8Error)?;

    Ok((output_str, output))
}

/// Executes a command and prints its output to stdout.
///
/// Internally calls `slrun` and then prints the result.
///
/// # Returns
/// A tuple containing the output string and the full Output object.
pub fn run(command: &str) -> Result<(String, Output)> {
    let output = slrun(command)?;
    println!("{}", output.0);
    Ok(output)
}

/// Macro to call `slrun` with formatted command string.
#[macro_export]
macro_rules! slrunf {
    ($($arg:tt)*) => {
        slrun(&format!($($arg)*))
            .map_err(SysxError::from)
    }
}
pub use slrunf;

/// Macro to call `run` with formatted command string.
#[macro_export]
macro_rules! runf {
    ($($arg:tt)*) => {
        run(&format!($($arg)*))
            .map_err(SysxError::from)
    }
}
pub use runf;

/// Reads a line from stdin into the provided buffer.
/// Removes the trailing newline character if present.
pub fn input_buf(buffer: &mut String) -> Result<()> {
    std::io::stdin()
        .read_line(buffer)
        .map_err(|e| SysxError::AnyhowError(anyhow::anyhow!("Failed to read line: {}", e)))
        .map(|_| {
            if buffer.ends_with('\n') {
                buffer.pop();
            }
            
        })
}

/// Reads a line from stdin and returns it as a new String.
/// Internally calls `input_buf`.
pub fn input() -> Result<String> {
    let mut input_text = String::new();
    input_buf(&mut input_text)?;
    Ok(input_text)
}
