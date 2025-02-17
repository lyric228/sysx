use std::process::{Command, Output, Stdio};
use std::io::{Write, Read};
use crate::error::Result;


pub fn raw_run(program: &str, arg: &str, command: &str) -> Result<String> {
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

#[cfg(feature = "sh")]
pub fn sh(command: &str) -> Result<String> {
    let out = raw_run("sh", "-c", command)?;
    #[cfg(not(feature = "silent"))]
    {
        println!("{out}");
    }
    Ok(out)
}

#[cfg(feature = "bash")]
pub fn bash(command: &str) -> Result<String> {
    let out = raw_run("bash", "-c", command)?;
    #[cfg(not(feature = "silent"))]
    {
        println!("{out}");
    }
    Ok(out)
}

#[cfg(feature = "zsh")]
pub fn zsh(command: &str) -> Result<String> {
    let out = raw_run("zsh", "-c", command)?;
    #[cfg(not(feature = "silent"))]
    {
        println!("{out}");
    }
    Ok(out)
}

#[cfg(feature = "cmd")]
pub fn cmd(command: &str) -> Result<String> {
    let out = raw_run("cmd", "/c", command)?;
    println!("{out}");
    Ok(out)
}

pub fn input() -> Result<String> {
    let mut input_text = String::new();
    std::io::stdin().read_to_string(&mut input_text)?;
    Ok(input_text)
}

pub fn run_with_args(command: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(command)
        .args(args)
        .output()?;
    
    let result = if output.status.success() {
        output.stdout
    } else {
        output.stderr
    };

    Ok(String::from_utf8_lossy(&result).to_string())
}


pub fn raw_run_interactive(program: &str, arg: &str, command: &str, input_data: &str) -> Result<String> {
    let mut child = Command::new(program)
        .arg(arg)
        .arg(command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
        
    if let Some(ref mut stdin) = child.stdin {
        stdin.write_all(input_data.as_bytes())?;
    }

    let output = child.wait_with_output()?;
    let result = if output.status.success() {
        output.stdout
    } else {
        output.stderr
    };

    Ok(String::from_utf8_lossy(&result).to_string())
}

#[cfg(feature = "sh")]
pub fn sh_interactive(command: &str, input_data: &str) -> Result<String> {
    let out = raw_run_interactive("sh", "-c", command, input_data)?;
    #[cfg(not(feature = "silent"))]
    {
        println!("{out}");
    }
    Ok(out)
}

#[cfg(feature = "bash")]
pub fn bash_interactive(command: &str, input_data: &str) -> Result<String> {
    let out = raw_run_interactive("bash", "-c", command, input_data)?;
    #[cfg(not(feature = "silent"))]
    {
        println!("{out}");
    }
    Ok(out)
}

#[cfg(feature = "zsh")]
pub fn zsh_interactive(command: &str, input_data: &str) -> Result<String> {
    let out = raw_run_interactive("zsh", "-c", command, input_data)?;
    #[cfg(not(feature = "silent"))]
    {
        println!("{out}");
    }
    Ok(out)
}

#[cfg(feature = "cmd")]
pub fn cmd_interactive(command: &str, input_data: &str) -> Result<String> {
    let out = raw_run_interactive("cmd", "/c", command, input_data)?;
    #[cfg(not(feature = "silent"))]
    {
        println!("{out}");
    }
    Ok(out)
}
