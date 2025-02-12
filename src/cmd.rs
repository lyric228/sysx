use std::process::{Command, Output, Stdio};
use std::io::{self, Read, Write};

pub use std::io::Error;


/// Выполняет команду в режиме "silent": результат команды возвращается в виде строки.  
/// Если команда завершается с ошибкой, возвращается stderr.
/// Для ошибок при выполнении команды возвращается Err(io::Error).
///
/// # Аргументы
/// * `command` - Команда для выполнения.
///
/// # Пример:
/// ```rust
/// let result = cmd::silent_cmd("ls -la")?;
/// println!("{}", result);
/// ```
pub fn silent_cmd(command: &str) -> io::Result<String> {
    #[cfg(unix)]
    let output: Output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()?;

    #[cfg(windows)]
    let output: Output = Command::new("cmd")
        .arg("/c")
        .arg(command)
        .output()?;
        
    let result = if output.status.success() {
        output.stdout
    } else {
        output.stderr
    };

    Ok(String::from_utf8_lossy(&result).to_string())
}

/// Выполняет команду (без дополнительных аргументов) и выводит результат в консоль, затем возвращает строковое представление результата.
/// В случае ошибки, выводится сообщение об ошибке и возвращается Err(io::Error).
///
/// # Пример
/// ```rust
/// let out = cmd::cmd("echo \"Hello\"");
/// ```
pub fn cmd(command: &str) -> io::Result<String> {
    let output = silent_cmd(command)?;
    println!("{output}");
    Ok(output)
}

/// Выполняет команду с произвольным списком аргументов.
/// Если команда завершается успешно, возвращает stdout, иначе возвращает stderr.
/// Поддерживает кроссплатформенность, определяя оболочку по умолчанию для каждой ОС.
/// Если требуется именно выполнение команды без схемы оболочки, можно использовать Command::new с аргументами напрямую.
///
/// # Аргументы
/// * `command` - Команда для выполнения.
/// * `args` - Срез аргументов команды.
///
/// # Пример
/// ```rust
/// let output = cmd::cmd_with_args("echo", &["Hello,", "world!"])?;
/// println!("{}", output);
/// ```
pub fn cmd_with_args(command: &str, args: &[&str]) -> io::Result<String> {
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

/// Читает полностью стандартный ввод и возвращает его как String.
///
/// # Пример
/// ```rust
/// let input_text = cmd::input()?;
/// println!("Введено: {}", input_text);
/// ```
pub fn input() -> io::Result<String> {
    let mut input_text = String::new();
    io::stdin().read_to_string(&mut input_text)?;
    Ok(input_text)
}

/// Выполняет команду в интерактивном режиме: позволяет передавать данные во входной поток команды
/// и считывать её вывод. Функция полезна, если нужно взаимодействовать с процессом во время его работы.
///
/// # Аргументы
/// * `command` - Команда для выполнения через оболочку (sh/cmd).
/// * `input_data` - Данные, которые будут переданы в стандартный ввод процесса.
///
/// # Пример
/// ```rust
/// let output = cmd::run_interactive("grep foo", "foo\nbar\nfoo bar\n")?;
/// println!("{}", output);
/// ```
pub fn run_interactive(command: &str, input_data: &str) -> io::Result<String> {
    #[cfg(unix)]
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    #[cfg(windows)]
    let mut child = Command::new("cmd")
        .arg("/c")
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
