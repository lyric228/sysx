use sysx::types::*;
use sysx::Error;

  
// get_type tests
  
#[test]
// Тест возвращает имя типа для целого числа.
// Здесь передаётся значение 10, ожидается, что функция вернёт "i32".
pub fn test_get_type_with_integer() {
    let value = 10;
    let type_name = get_type(&value);
    assert_eq!(type_name, "i32");
}

#[test]
// Тест возвращает имя типа для строкового литерала.
// Ожидается, что результат будет содержать подстроку "str", поскольку тип строки в Rust представлен именно так.
fn test_get_type_with_str() {
    let text = "Hello";
    let type_name = get_type(&text);
    assert!(type_name.contains("str"));
}

  
// is_list_like tests
  
#[test]
// Тест проверяет, что обобщённый тип определяется как list-like.
// Передаём тип Vec<i32> и ожидаем, что функция вернёт true.
fn test_is_list_like_for_generic() {
    assert!(is_list_like("Vec<i32>"));
}

#[test]
// Тест проверяет, что массив определяется как list-like.
// Передаём тип [i32; 5] и ожидаем положительный результат.
fn test_is_list_like_for_array() {
    assert!(is_list_like("[i32; 5]"));
}

#[test]
// Тест проверяет, что простые (необобщённые) типы не считаются list-like.
// Передаём полный путь типа std::string::String, ожидаем false.
fn test_is_list_like_for_simple_type() {
    assert!(!is_list_like("std::string::String"));
}

#[test]
// Тест проверяет граничный случай для нестандартного имени без угловых скобок.
// Передаём произвольное имя "MyType" и ожидаем, что тип не будет считаться list-like.
fn test_is_list_like_edge_case() {
    assert!(!is_list_like("MyType"));
}

  
// simplify_nonlist_type tests
  
#[test]
// Тест на успешное упрощение не обобщённого типа.
// Из полного имени std::string::String должно быть получено упрощённое "String".
fn test_simplify_nonlist_type_success() {
    let input = "std::string::String";
    let simplified = simplify_nonlist_type(input).unwrap();
    assert_eq!(simplified, "String");
}

#[test]
// Тест должен вернуть ошибку, если передан обобщённый тип.
// Для типа "Vec<i32>" функция simplify_nonlist_type не подходит, поэтому ожидается ошибка валидации.
fn test_simplify_nonlist_type_error_for_generic() {
    let input = "Vec<i32>";
    let err = simplify_nonlist_type(input).unwrap_err();
    if let Error::ValidationError { expected, actual, .. } = err {
        assert_eq!(expected, "необобщённый тип");
        assert_eq!(actual, input);
    } else {
        panic!("Ожидалась ошибка Валидации");
    }
}

  
// simplify_type tests
  
#[test]
// Тест для не обобщённого типа: функция simplify_type должна делегировать вызов simplify_nonlist_type.
// Передаём "std::path::PathBuf", ожидаем получить "PathBuf".
fn test_simplify_type_for_non_generic() {
    let input = "std::path::PathBuf";
    let simplified = simplify_type(input).unwrap();
    assert_eq!(simplified, "PathBuf");
}

#[test]
// Тест для простого обобщённого типа: внутри угловых скобок должны быть удалены квалификаторы.
// Из "std::vec::Vec<my::custom::Type>" должно получиться "Vec<Type>".
fn test_simplify_type_with_generic_single() {
    let input = "std::vec::Vec<my::custom::Type>";
    let simplified = simplify_type(input).unwrap();
    assert_eq!(simplified, "Vec<Type>");
}

#[test]
// Тест для обобщённого типа с несколькими параметрами.
// Передаём "std::collections::HashMap<my::key::Key, my::value::Value>" и ожидаем, что квалификаторы уберутся.
fn test_simplify_type_with_generic_multiple() {
    let input = "std::collections::HashMap<my::key::Key, my::value::Value>";
    let simplified = simplify_type(input).unwrap();
    assert_eq!(simplified, "HashMap<Key, Value>");
}

#[test]
// Тест для вложенных обобщённых типов.
// Вложенные обобщённые типы должны быть упрощены:
// "std::option::Option<std::vec::Vec<my::custom::Type>>" должно превратиться в "Option<Vec<Type>>".
fn test_simplify_type_with_nested_generics() {
    let input = "std::option::Option<std::vec::Vec<my::custom::Type>>";
    let simplified = simplify_type(input).unwrap();
    assert_eq!(simplified, "Option<Vec<Type>>");
}
