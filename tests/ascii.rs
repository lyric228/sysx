//! Integration tests for the ASCII art module

use std::fs;
use std::io::Write;
use std::path::PathBuf;

// Импортируем функции и константы из модуля ascii
use sysx::utils::ascii::{
    CHAR_SET_DETAILED, CHAR_SET_MEDIUM, CHAR_SET_SIMPLE, image_to_ascii, image_to_ascii_with_chars,
    pixel_brightness,
};

use image::{Rgb, Rgba};

// Вспомогательная функция для создания тестового изображения
fn create_test_image(path: &PathBuf) {
    // Создаем тестовую директорию, если её нет
    let parent = path.parent().unwrap();
    fs::create_dir_all(parent).expect("Failed to create test directory");

    // Создаем небольшое тестовое изображение (10x10 пикселей)
    let mut img = image::RgbImage::new(10, 10);

    // Заполняем его градиентом от черного к белому
    for y in 0..10 {
        for x in 0..10 {
            let brightness = (x + y) as u8 * 12;
            img.put_pixel(x, y, Rgb([brightness, brightness, brightness]));
        }
    }

    // Сохраняем изображение
    img.save(path).expect("Failed to save test image");
}

#[test]
fn test_pixel_brightness() {
    // Тестируем расчет яркости пикселей

    // Черный пиксель
    let black_pixel = Rgb([0u8, 0u8, 0u8]);
    assert!(pixel_brightness(black_pixel) < 0.001);

    // Белый пиксель
    let white_pixel = Rgb([255u8, 255u8, 255u8]);
    assert!(pixel_brightness(white_pixel) > 0.999);

    // Красный пиксель (должен быть темнее зеленого из-за взвешенного расчета)
    let red_pixel = Rgb([255u8, 0u8, 0u8]);
    let green_pixel = Rgb([0u8, 255u8, 0u8]);

    assert!(pixel_brightness(red_pixel) < pixel_brightness(green_pixel));

    // Проверка пикселя с альфа-каналом
    let rgba_pixel = Rgba([128u8, 128u8, 128u8, 255u8]);
    let brightness = pixel_brightness(rgba_pixel);
    assert!(
        (brightness - 0.5).abs() < 0.01,
        "RGBA pixel with 50% brightness: got {}",
        brightness
    );
}

#[test]
fn test_image_to_ascii_with_default_charset() {
    // Создаем путь к тестовому изображению
    let test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources");
    let test_image = test_dir.join("test_gradient.png");

    // Создаем тестовое изображение
    create_test_image(&test_image);

    // Преобразуем изображение в ASCII с разными наборами символов
    let detailed = image_to_ascii(&test_image, 20, 10, CHAR_SET_DETAILED).unwrap();
    let medium = image_to_ascii(&test_image, 20, 10, CHAR_SET_MEDIUM).unwrap();
    let simple = image_to_ascii(&test_image, 20, 10, CHAR_SET_SIMPLE).unwrap();

    // Проверяем, что результат не пуст
    assert!(
        !detailed.is_empty(),
        "ASCII art with detailed charset should not be empty"
    );
    assert!(
        !medium.is_empty(),
        "ASCII art with medium charset should not be empty"
    );
    assert!(
        !simple.is_empty(),
        "ASCII art with simple charset should not be empty"
    );

    // Проверяем, что результат содержит символы новой строки (т.е. он многострочный)
    assert!(detailed.contains('\n'), "ASCII art should contain newlines");

    // Подсчитываем количество строк и символов в строке
    let lines: Vec<&str> = detailed.lines().collect();
    assert!(lines.len() <= 10, "ASCII art should have at most 10 lines");

    if let Some(first_line) = lines.first() {
        assert!(
            first_line.chars().count() <= 20,
            "ASCII art line should have at most 20 characters"
        );
    }

    // Сохраняем результаты для визуальной проверки (опционально)
    let output_dir = test_dir.join("output");
    fs::create_dir_all(&output_dir).expect("Failed to create output directory");

    let mut detailed_file = fs::File::create(output_dir.join("detailed.txt")).unwrap();
    detailed_file.write_all(detailed.as_bytes()).unwrap();

    let mut medium_file = fs::File::create(output_dir.join("medium.txt")).unwrap();
    medium_file.write_all(medium.as_bytes()).unwrap();

    let mut simple_file = fs::File::create(output_dir.join("simple.txt")).unwrap();
    simple_file.write_all(simple.as_bytes()).unwrap();
}

#[test]
fn test_image_to_ascii_with_custom_charset() {
    // Создаем путь к тестовому изображению
    let test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources");
    let test_image = test_dir.join("test_custom.png");

    // Создаем тестовое изображение
    create_test_image(&test_image);

    // Создаем пользовательский набор символов
    let custom_charset = "XO-.";

    // Преобразуем изображение в ASCII с пользовательским набором символов
    let result = image_to_ascii(&test_image, 20, 10, custom_charset).unwrap();

    // Проверяем, что результат содержит только символы из пользовательского набора и новую строку
    for c in result.chars() {
        if c != '\n' {
            assert!(
                custom_charset.contains(c),
                "Character '{}' not in custom charset '{}'",
                c,
                custom_charset
            );
        }
    }
}

#[test]
fn test_image_to_ascii_with_chars_vector() {
    // Создаем путь к тестовому изображению
    let test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources");
    let test_image = test_dir.join("test_vector.png");

    // Создаем тестовое изображение
    create_test_image(&test_image);

    // Создаем вектор символов
    let chars_vec = vec!['#', '=', '-', '.', ' '];

    // Преобразуем изображение в ASCII с вектором символов
    let result = image_to_ascii_with_chars(&test_image, 20, 10, &chars_vec).unwrap();

    // Проверяем, что результат не пуст
    assert!(
        !result.is_empty(),
        "ASCII art with chars vector should not be empty"
    );

    // Проверяем, что результат содержит только символы из вектора и новую строку
    for c in result.chars() {
        if c != '\n' {
            assert!(
                chars_vec.contains(&c),
                "Character '{}' not in chars vector",
                c
            );
        }
    }
}

#[test]
fn test_different_image_dimensions() {
    // Создаем путь к тестовому изображению
    let test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources");
    let test_image = test_dir.join("test_dimensions.png");

    // Создаем тестовое изображение
    create_test_image(&test_image);

    // Тестируем разные размеры выходного ASCII-изображения
    let small = image_to_ascii(&test_image, 10, 5, CHAR_SET_SIMPLE).unwrap();
    let medium = image_to_ascii(&test_image, 30, 15, CHAR_SET_SIMPLE).unwrap();
    let large = image_to_ascii(&test_image, 50, 25, CHAR_SET_SIMPLE).unwrap();

    // Подсчитываем количество строк в каждом результате
    let small_lines = small.lines().count();
    let medium_lines = medium.lines().count();
    let large_lines = large.lines().count();

    // Проверяем, что количество строк не превышает указанные значения
    assert!(
        small_lines <= 5,
        "Small ASCII art should have at most 5 lines"
    );
    assert!(
        medium_lines <= 15,
        "Medium ASCII art should have at most 15 lines"
    );
    assert!(
        large_lines <= 25,
        "Large ASCII art should have at most 25 lines"
    );

    // Проверяем длину первой строки в каждом результате
    if let Some(first_line) = small.lines().next() {
        assert!(
            first_line.chars().count() <= 10,
            "Small ASCII art line should have at most 10 characters"
        );
    }

    if let Some(first_line) = medium.lines().next() {
        assert!(
            first_line.chars().count() <= 30,
            "Medium ASCII art line should have at most 30 characters"
        );
    }

    if let Some(first_line) = large.lines().next() {
        assert!(
            first_line.chars().count() <= 50,
            "Large ASCII art line should have at most 50 characters"
        );
    }
}

#[test]
fn test_empty_charset_error() {
    // Создаем путь к тестовому изображению
    let test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources");
    let test_image = test_dir.join("test_error.png");

    // Создаем тестовое изображение
    create_test_image(&test_image);

    // Пытаемся преобразовать изображение с пустым набором символов
    let result = image_to_ascii(&test_image, 20, 10, "");

    // Проверяем, что функция вернула ошибку
    assert!(
        result.is_err(),
        "Function should return error for empty charset"
    );

    // Проверяем, что ошибка имеет правильный тип (если можно получить доступ к типу ошибки)
    if let Err(err) = result {
        // Преобразуем ошибку в строку и проверяем содержимое
        let err_string = format!("{:?}", err);
        assert!(
            err_string.contains("Empty character set"),
            "Error should mention empty character set"
        );
    }

    // То же самое для функции image_to_ascii_with_chars
    let result = image_to_ascii_with_chars(&test_image, 20, 10, &Vec::<char>::new());
    assert!(
        result.is_err(),
        "Function should return error for empty chars vector"
    );
}
