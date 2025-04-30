use sysx::{SysxError, io::cmd::*};

#[test]
fn test_command_execution() {
    // slrun
    let (output, _) = slrun("echo test").unwrap();
    assert_eq!(output.trim(), "test");

    let res = slrun("nonexistentcommand");
    assert!(matches!(res.unwrap_err(), SysxError::AnyhowError(_)));

    // run
    let (output, _) = run("echo test").unwrap();
    assert_eq!(output.trim(), "test");
}

#[test]
fn test_command_macros() {
    let arg = "world";
    let (output, _) = slrunf!("echo hello {}", arg).unwrap();
    assert_eq!(output.trim(), "hello world");

    let (output, _) = runf!("echo hello {}", arg).unwrap();
    assert_eq!(output.trim(), "hello world");
}
