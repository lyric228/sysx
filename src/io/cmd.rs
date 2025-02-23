use std::process::{
    Command,
    Output,
    Stdio,
};

use anyhow::Context;

pub use crate::{
    Result,
    SysxError,
};

/// Выполняет команду в «тихом» режиме, не выводя результат на экран.
/// 
/// Принимает строку команды, разбирает её на программу и аргументы, затем выполняет команду.
/// Если команда завершается успешно, возвращает стандартный вывод, иначе - стандартную ошибку.
/// 
/// # Возвращаемое значение
/// Кортеж, где первый элемент - строка результата выполнения (stdout или stderr), а второй - полный объект Output.
/// 
/// # Пример
/// ```
/// // Передаём команду в виде строки
/// let (output_str, output) = silent_run("echo Hello").unwrap();
/// // output_str будет "Hello\n", output содержит детали исполнения команды.
/// ```
pub fn silent_run(command_line: &str) -> Result<(String, Output)> {
    let trimmed = command_line.trim();

    if trimmed.is_empty() {
        return Err(SysxError::AnyhowError(
            anyhow::anyhow!("Empty command line"),
        ));
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

    let output_str = String::from_utf8(result).map_err(|e| SysxError::FromUtf8Error(e))?;

    Ok((output_str, output))
}

/// Выполняет команду и выводит результат в стандартный вывод.
/// 
/// Вызывает silent_run для выполнения команды, затем печатает результат на экран.
/// 
/// # Возвращаемое значение
/// Кортеж, содержащий строку результата и объект Output.
/// 
/// # Пример
/// ```
/// // Выполнение команды и печать результата в терминал
/// let (output_str, output) = run("echo Hello").unwrap();
/// // На экране появится "Hello\n"
/// ```
pub fn run(command: &str) -> Result<(String, Output)> {
    let output = silent_run(command)?;
    println!("{}", output.0);
    Ok(output)
}

/// Макрос для вызова функции silent_run с форматированием строки команды.
/// 
/// Принимает набор аргументов для форматирования строки, вызывает silent_run с полученной командной строкой.
/// 
/// # Пример
/// ```
/// // Форматирование команды с подстановкой переменной
/// let result = silent_runf!("echo {}", "Hello");
/// // Выполнится команда "echo Hello", вернётся результат выполнения
/// ```
#[macro_export]
macro_rules! silent_runf {
    ($($arg:tt)*) => {
        silent_run(&format!($($arg)*))
            .map_err(SysxError::from)
    }
}
pub use silent_runf;

/// Макрос для вызова функции run с форматированием строки команды.
/// 
/// Принимает набор аргументов, форматирует команду с помощью format! и вызывает run.
/// 
/// # Пример
/// ```
/// // Форматирование команды и вывод результата
/// let result = runf!("echo {}", "World");
/// // Выполнится команда "echo World" и результат будет выведен на экран.
/// ```
#[macro_export]
macro_rules! runf {
    ($($arg:tt)*) => {
        run(&format!($($arg)*))
            .map_err(SysxError::from)
    }
}
pub use runf;

/// Считывает строку из стандартного ввода в предоставленный буфер.
/// 
/// Функция читает одну строку из stdin и записывает её в buffer.
/// Если строка заканчивается символом новой строки, последний символ удаляется.
/// 
/// # Возвращаемое значение
/// Возвращает Ok(()) при успешном чтении или ошибку типа SysxError.
/// 
/// # Пример
/// ```
/// let mut buf = String::new();
/// input_buf(&mut buf).unwrap();
/// // Допустим, введена строка "test\n", buf станет равен "test"
/// ```
pub fn input_buf(buffer: &mut String) -> Result<()> {
    std::io::stdin().read_line(buffer)
        .map_err(|e| SysxError::AnyhowError(
            anyhow::anyhow!("Failed to read line: {}", e)))
        .and_then(|_| {
            if buffer.ends_with('\n') {
                buffer.pop();
            }
            Ok(())
        })
}

/// Считывает строку из стандартного ввода и возвращает её.
/// 
/// Функция оборачивает input_buf, создавая новый buffer, считывая в него строку и возвращая значение.
/// 
/// # Возвращаемое значение
/// Возвращает Ok(String) с содержимым строки, или ошибку типа SysxError.
/// 
/// # Пример
/// ```
/// let user_input = input().unwrap();
/// // Если введено "data", user_input будет "data"
/// ```
pub fn input() -> Result<String> {
    let mut input_text = String::new();
    input_buf(&mut input_text)?;
    Ok(input_text)
}
