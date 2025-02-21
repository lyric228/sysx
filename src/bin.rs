use crate::error::*;


/// Преобразует строку из бинарных значений (например, "01001000 01100101 01101100 01101100 01101111")
/// в обычную строку UTF-8.
pub fn bin_to_str(bin: &str) -> Result<String> {
    // Приводим строку к единому регистру не требуется, так как бинарные цифры '0' и '1' не чувствительны к регистру.
    // Если пробелы позволяют разделить байты, можно работать сразу.
    bin.split_whitespace()
        .map(|s| u8::from_str_radix(s, 2).map_err(SysxError::from)) // Преобразуем каждую подстроку из бинарного формата
        .collect::<Result<Vec<u8>>>() // собираем байты в вектор
        .and_then(|bytes| String::from_utf8(bytes).map_err(SysxError::from))
}

/// Преобразует строку в бинарное представление: для каждого байта выводится 8-битное значение,
/// разделённое пробелами.
pub fn str_to_bin(text: &str) -> String {
    text.as_bytes()
        .iter()
        .map(|b| format!("{:08b}", b)) // выводим байт в виде 8-битного бинарного числа
        .collect::<Vec<String>>()
        .join(" ")
}

/// Проверяет, содержит ли переданная строка только символы '0', '1' и пробельные символы.
pub fn is_valid_bin(bin: &str) -> bool {
    bin.chars()
        .filter(|c| !c.is_whitespace())
        .all(|c| c == '0' || c == '1')
}

/// Более строгая проверка бинарной строки: удаляем все пробелы и проверяем, что длина полученной строки
/// кратна 8 (т.е. полное количество бит) и что все символы – '0' или '1'.
pub fn is_valid_bin_strict(bin: &str) -> bool {
    let trimmed = bin.replace(char::is_whitespace, "");
    trimmed.len() % 8 == 0 && trimmed.chars().all(|c| c == '0' || c == '1')
}
