use std::any::Any;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::Error;

/// Статическая переменная, содержащая скомпилированное регулярное выражение для удаления квалификаторов namespace.
/// Lazy гарантирует, что regex будет скомпилирован только один раз.
static QUALIFIER_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"([a-zA-Z_][a-zA-Z0-9_]*::)+")
        .expect("Не удалось скомпилировать регулярное выражение для квалификаторов")
});

/// Упрощает строковое представление типа для случая, когда оно не содержит параметров обобщённости.
/// Если строка содержит символы '<' или '>', функция возвращает ошибку валидации.
///
/// Аргументы:
/// - type_str: Строка, представляющая тип.
///
/// Возвращает:
/// - Упрощённое имя типа без квалификаторов namespace, либо ошибку.
///
/// Пример:
/// ```
/// let simple = simplify_nonlist_type("std::string::String").unwrap();
/// assert_eq!(simple, "String");
/// ```
pub fn simplify_nonlist_type(type_str: &str) -> Result<String, Error> {
    if is_list_like(type_str) {
        return Err(Error::ValidationError {
            expected: "необобщённый тип",
            actual: type_str.to_string(),
            context: Some("упрощение типа требует базовый тип".into()),
        });
    }

    Ok(type_str.split("::").last().unwrap_or("unknown").to_string())
}

/// Возвращает строку с именем типа переданного значения.
///
/// Используется механизм std::any для получения имени типа T.
///
/// Аргументы:
/// - _: Ссылка на значение типа T.
///
/// Пример:
/// ```
/// let t = 10;
/// let type_name = get_type(&t);
/// assert_eq!(type_name, "i32");
/// ```
pub fn get_type<T: Any>(_: &T) -> String {
    std::any::type_name::<T>().to_owned()
}

/// Проверяет, указывает ли строка на обобщённый или коллекционно-подобный тип.
///
/// Помимо проверки на наличие символов '<' или '>', функция также ищет шаблоны,
/// характерные для коллекций, например, префикс "Vec<" или квадратные скобки для массивов/срезов.
///
/// Аргументы:
/// - type_str: Строка, представляющая тип.
///
/// Возвращает:
/// - true, если тип выглядит как обобщённый или коллекция; иначе false.
///
/// Пример:
/// ```
/// assert!(is_list_like("Vec<i32>"));
/// assert!(is_list_like("[i32; 5]"));
/// assert!(!is_list_like("std::string::String"));
/// ```
pub fn is_list_like(type_str: &str) -> bool {
    if type_str.contains('<') || type_str.contains('>') {
        return true;
    }

    let trimmed = type_str.trim();

    trimmed.starts_with("Vec<") || (trimmed.starts_with('[') && trimmed.ends_with(']'))
}

/// Упрощает строковое представление типа, удаляя квалификаторы namespace и обрабатывая параметры обобщённости.
///
/// Если переданный тип не содержит параметров ('<' или '>'), вызывается функция simplify_nonlist_type.
/// Для типов с обобщёнными параметрами функция разделяет строку по запятым вне вложенных скобок
/// и для каждого токена удаляет квалификаторы с помощью регулярного выражения.
///
/// Аргументы:
/// - type_str: Строка, представляющая тип.
///
/// Возвращает:
/// - Упрощённое строковое представление типа, либо ошибку.
///
/// Пример:
/// ```
/// let simplified = simplify_type("std::vec::Vec<my::custom::Type>").unwrap();
/// assert_eq!(simplified, "Vec<Type>");
/// ```
pub fn simplify_type<'a>(type_str: &'a str) -> Result<String, Error> {
    if !is_list_like(type_str) {
        return simplify_nonlist_type(type_str);
    }

    let mut current_token = String::new();
    let mut result = String::new();
    let mut inside_angle_brackets: i32 = 0;

    for c in type_str.chars() {
        match c {
            '<' => {
                inside_angle_brackets += 1;
                current_token.push(c);
            }
            '>' => {
                inside_angle_brackets = inside_angle_brackets.saturating_sub(1);
                current_token.push(c);
            }
            ',' if inside_angle_brackets == 0 => {
                let simplified_token = QUALIFIER_RE.replace_all(&current_token, "");
                if !result.is_empty() {
                    result.push_str(", ");
                }
                result.push_str(&simplified_token);
                current_token.clear();
            }
            _ => {
                current_token.push(c);
            }
        }
    }

    if !current_token.is_empty() {
        let simplified_token = QUALIFIER_RE.replace_all(&current_token, "");
        if !result.is_empty() {
            result.push_str(", ");
        }
        result.push_str(&simplified_token);
    }

    Ok(result.trim().to_string())
}
