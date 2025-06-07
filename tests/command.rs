use std::string::String;

use sysx::{SysxError, io::cmd::*}; // Import String

#[test]
fn test_command_execution() {
    // slrun
    let output = slrun("echo test").unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Command failed: {:?}",
        output.status
    );
    assert_eq!(stdout.trim(), "test");
    assert!(stderr.is_empty(), "Stderr was not empty: {stderr}");

    let res = slrun("nonexistentcommand");
    assert!(
        matches!(res.unwrap_err(), SysxError::AnyhowError(_)),
        "Expected AnyhowError for nonexistent command"
    );

    // run
    let output = run("echo test").unwrap(); // run prints stdout internally
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Command failed: {:?}",
        output.status
    );
    assert_eq!(stdout.trim(), "test"); // Check stdout from the returned Output
    assert!(stderr.is_empty(), "Stderr was not empty: {stderr}");
}

#[test]
fn test_command_macros() {
    let arg = "world";

    // slrunf
    let output = slrunf!("echo hello {}", arg).unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Macro command failed: {:?}",
        output.status
    );
    assert_eq!(stdout.trim(), "hello world");
    assert!(stderr.is_empty(), "Stderr was not empty: {stderr}");

    // runf
    let output = runf!("echo hello {}", arg).unwrap(); // runf calls run, which prints stdout
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "Macro command failed: {:?}",
        output.status
    );
    assert_eq!(stdout.trim(), "hello world"); // Check stdout from the returned Output
    assert!(stderr.is_empty(), "Stderr was not empty: {stderr}");
}
