use std::collections::HashMap;

use sysx::io::env::*;


// set_env tests

#[test]
// Тест установки и получения переменной окружения.
// Устанавливает новую переменную и проверяет её значение.
fn test_set_and_get_env() {
    set_env("TEST_KEY", "test_value").unwrap();
    assert_eq!(get_env("TEST_KEY").unwrap(), "test_value");
}

#[test]
// Тест перезаписи существующей переменной.
// Проверяет, что новое значение заменяет старое.
fn test_set_env_override() {
    set_env("TEST_OVERRIDE", "old_value").unwrap();
    set_env("TEST_OVERRIDE", "new_value").unwrap();
    assert_eq!(get_env("TEST_OVERRIDE").unwrap(), "new_value");
}


// get_env tests

#[test]
// Тест получения несуществующей переменной.
// Должен вернуть None.
fn test_get_nonexistent_env() {
    assert_eq!(get_env("NONEXISTENT_KEY"), None);
}

#[test]
// Тест получения переменной из кэша.
// Устанавливает переменную и проверяет её наличие в кэше.
fn test_get_env_from_cache() {
    set_env("CACHED_KEY", "cached_value").unwrap();
    assert_eq!(get_env("CACHED_KEY").unwrap(), "cached_value");
}


// get_envs tests

#[test]
// Тест получения всех переменных окружения.
// Проверяет, что возвращаемый HashMap содержит установленные переменные.
fn test_get_all_envs() {
    set_env("TEST_ENV1", "value1").unwrap();
    set_env("TEST_ENV2", "value2").unwrap();
    
    let envs = get_envs();
    assert_eq!(envs.get("TEST_ENV1").unwrap(), "value1");
    assert_eq!(envs.get("TEST_ENV2").unwrap(), "value2");
}


// get_args tests

#[test]
// Тест получения аргументов без имени программы.
// Проверяет, что первый аргумент (имя программы) исключается.
fn test_get_args() {
    let args = get_args();
    assert!(args.len() < get_full_args().len());
}


// get_full_args tests

#[test]
// Тест получения всех аргументов.
// Проверяет, что список включает имя программы.
fn test_get_full_args() {
    let args = get_full_args();
    assert!(!args.is_empty());
}


// get_str_args tests

#[test]
// Тест получения строки аргументов без имени программы.
// Проверяет формат объединения аргументов в строку.
fn test_get_str_args() {
    let args_str = get_str_args();
    let args_vec = get_args();
    assert_eq!(args_str, args_vec.join(" "));
}


// get_full_str_args tests

#[test]
// Тест получения полной строки аргументов.
// Проверяет формат объединения всех аргументов в строку.
fn test_get_full_str_args() {
    let full_str = get_full_str_args();
    let full_vec = get_full_args();
    assert_eq!(full_str, full_vec.join(" "));
}
