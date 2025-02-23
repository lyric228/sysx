use sysx::{
    io::cmd::*,
    SysxError,
};


// silent_run tests

#[test]
// Тест успешного выполнения команды echo.
// Ожидается успешное выполнение и получение вывода.
fn test_silent_run_echo_success() {
    let (output, _) = silent_run("echo test").unwrap();
    assert_eq!(output.trim(), "test");
}

#[test]
// Тест с пустой командной строкой.
// Должен вернуть ошибку с сообщением о пустой команде.
fn test_silent_run_empty_command() {
    let result = silent_run("   ");
    assert!(matches!(result.unwrap_err(), SysxError::AnyhowError(_)));
}

#[test]
// Тест с некорректной командой.
// Должен вернуть ошибку выполнения.
fn test_silent_run_invalid_command() {
    let result = silent_run("nonexistentcommand");
    assert!(matches!(result.unwrap_err(), SysxError::AnyhowError(_)));
}


// run tests

#[test]
// Тест успешного выполнения команды через run.
// Аналогичен silent_run, но с выводом в консоль.
fn test_run_echo_success() {
    let (output, _) = run("echo test").unwrap();
    assert_eq!(output.trim(), "test");
}

#[test]
// Тест обработки ошибки в run.
// При некорректной команде должна вернуться ошибка.
fn test_run_error_handling() {
    let result = run("invalidcommand");
    assert!(matches!(result.unwrap_err(), SysxError::AnyhowError(_)));
}


// input_buf tests

#[test]
// Тест чтения строки без символа новой строки.
// Проверяет корректность удаления символа новой строки.
fn test_input_buf_removes_newline() {
    let mut buffer = String::from("test\n");
    input_buf(&mut buffer).unwrap();
    assert_eq!(buffer, "test");
}

#[test]
// Тест чтения строки без символа новой строки.
// Проверяет что строка остается неизменной.
fn test_input_buf_without_newline() {
    let mut buffer = String::from("test");
    input_buf(&mut buffer).unwrap();
    assert_eq!(buffer, "test");
}


// macro tests

#[test]
// Тест макроса silent_runf с форматированием.
// Проверяет работу форматирования аргументов.
fn test_silent_runf_macro() {
    let arg = "world";
    let (output, _) = silent_runf!("echo hello {}", arg).unwrap();
    assert_eq!(output.trim(), "hello world");
}

#[test]
// Тест макроса runf с форматированием.
// Проверяет работу форматирования аргументов.
fn test_runf_macro() {
    let arg = "world";
    let (output, _) = runf!("echo hello {}", arg).unwrap();
    assert_eq!(output.trim(), "hello world");
}
