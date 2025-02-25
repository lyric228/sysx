use sysx::{math::bin::*, math::hex::*};

#[test]
fn test_binary_conversions() {
    // Binary
    let bin = "01001000";
    assert_eq!(bin_to_str(bin).unwrap(), "H");
}

#[test]
fn test_hex_conversions() {
    // Тест с пробелами
    let hex_spaced = "48 65 6C 6C 6F";
    assert_eq!(hex_to_str(hex_spaced).unwrap(), "Hello");

    // Тест без пробелов
    let hex_continuous = "48656C6C6F";
    assert_eq!(hex_to_str(hex_continuous).unwrap(), "Hello");

    // Тест с мусором
    let hex_dirty = "48z65$6C\n6C_6F";
    assert_eq!(hex_to_str(hex_dirty).unwrap(), "Hello");

    // Тест ошибок
    assert!(hex_to_str("486").is_err()); // Нечётная длина
    assert!(!hex_to_str("48GG").is_err()); // Невалидные символы должны быть проигнорированы 
}

#[test]
fn test_hex_formatting() {
    let original = "Hello";
    let hex = str_to_hex(original);

    // Проверяем оба формата
    assert!(hex == "48 65 6C 6C 6F" || hex == "48656C6C6F");

    // Round-trip тест
    assert_eq!(hex_to_str(&hex).unwrap(), original);
}
