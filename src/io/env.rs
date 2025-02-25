use std::collections::HashMap;

use lazy_static::lazy_static;
use parking_lot::Mutex;

use crate::{
    Result,
    SysxError,
};


lazy_static! {
    /// Глобальный кэш переменных окружения, защищённый мьютексом.
    ///
    /// Инициализируется текущими переменными окружения из std::env::vars().
    static ref ENV_VARS: Mutex<HashMap<String, String>> = Mutex::new(std::env::vars().collect());
}

/// Устанавливает переменную окружения и сохраняет её в глобальном кэше.
///
/// Принимает ключ и значение переменной в виде строковых срезов.
/// Вызывает std::env::set_var для обновления переменной окружения,
/// а затем обновляет глобальный кэш ENV_VARS.
///
/// # Возвращаемое значение
/// Возвращает Ok(()) при успешном выполнении или ошибку типа SysxError.
///
/// # Пример
/// ```
/// // Установить переменную окружения "KEY" в значение "value"
/// set_env("KEY", "value").unwrap();
/// ```
pub fn set_env(key: &str, value: &str) -> Result<()> {
    // Обновляем переменную окружения на уровне ОС.
    unsafe {
        std::env::set_var(key, value);
    }
    // Сохраняем переменную в кэше.
    ENV_VARS.lock()
        .insert(key.to_string(), value.to_string());
    Ok(())
}

/// Возвращает значение переменной окружения по заданному ключу.
///
/// Сначала пытается получить переменную через std::env::var, при неудаче обращается к локальному кэшу.
///
/// # Возвращаемое значение
/// Возвращает Some(value) если переменная найдена или None, если переменная отсутствует.
///
/// # Пример
/// ```
/// // Пусть ранее была установлена переменная "KEY" в "value"
/// let value = get_env("KEY");
/// // value будет Some("value".to_string())
/// ```
pub fn get_env(key: &str) -> Result<String> {
    std::env::var(key)
        .or_else(|_| {
            ENV_VARS.lock()
            .get(key)
            .cloned()
            .ok_or(SysxError::EnvVarNotFound(key.to_string()))
        }
    )
}

/// Возвращает копию всех переменных окружения из глобального кэша.
///
/// # Возвращаемое значение
/// Возвращает HashMap, где ключи и значения - строки, представляющие переменные окружения.
///
/// # Пример
/// ```
/// // Получить все переменные окружения
/// let envs = get_envs();
/// // envs содержит копию кэша переменных
/// ```
pub fn get_envs() -> HashMap<String, String> {
    ENV_VARS.lock().clone()
}

/// Возвращает вектор аргументов командной строки, включая имя программы.
///
/// # Возвращаемое значение
/// Вектор строк, полученных из std::env::args().
///
/// # Пример
/// ```
/// // Если программа запущена как "myprog arg1 arg2"
/// let args = get_full_args();
/// // args будет vec!["myprog".to_string(), "arg1".to_string(), "arg2".to_string()]
/// ```
pub fn get_full_args() -> Vec<String> {
    std::env::args().collect()
}

/// Возвращает вектор аргументов командной строки, исключая имя программы.
///
/// # Возвращаемое значение
/// Вектор строк, содержащих аргументы, переданные программе.
///
/// # Пример
/// ```
/// // Если программа запущена как "myprog arg1 arg2"
/// let args = get_args();
/// // args будет vec!["arg1".to_string(), "arg2".to_string()]
/// ```
pub fn get_args() -> Vec<String> {
    let mut args = get_full_args();
    let _ = args.remove(0);
    args
}

/// Возвращает все аргументы командной строки в виде одной строки.
///
/// Аргументы объединяются пробелами в одну строку.
///
/// # Возвращаемое значение
/// Строка, содержащая все аргументы командной строки.
///
/// # Пример
/// ```
/// // Если программа запущена как "myprog arg1 arg2"
/// let args_str = get_full_str_args();
/// // args_str будет "myprog arg1 arg2"
/// ```
pub fn get_full_str_args() -> String {
    get_full_args().join(" ")
}

/// Возвращает аргументы командной строки (без имени программы) в виде одной строки.
///
/// Аргументы объединяются пробелами в одну строку.
///
/// # Возвращаемое значение
/// Строка, содержащая аргументы командной строки, исключая имя программы.
///
/// # Пример
/// ```
/// // Если программа запущена как "myprog arg1 arg2"
/// let args_str = get_str_args();
/// // args_str будет "arg1 arg2"
/// ```
pub fn get_str_args() -> String {
    get_args().join(" ")
}
