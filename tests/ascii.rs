use std::fs;
use std::path::PathBuf;

use sysx::utils::ascii::{
    CHAR_SET_DETAILED, CHAR_SET_MEDIUM, CHAR_SET_SIMPLE, image_to_ascii, image_to_ascii_chars,
    image_to_ascii_configurable, pixel_brightness, AsciiArtConfig,
};

use image::{Rgb, Rgba, imageops::FilterType};
fn create_test_image(path: &PathBuf) {
    let parent = path.parent().unwrap();
    fs::create_dir_all(parent).expect("Failed to create test directory");
    let mut img = image::RgbImage::new(10, 10);
    for y in 0..10 {
        for x in 0..10 {
            let brightness = (x + y) as u8 * 12;
            img.put_pixel(x, y, Rgb([brightness, brightness, brightness]));
        }
    }
    img.save(path).expect("Failed to save test image");
}

#[test]
fn test_pixel_brightness() {
    let black_pixel = Rgb([0u8, 0u8, 0u8]);
    assert!(pixel_brightness(black_pixel) < 0.001);
    let white_pixel = Rgb([255u8, 255u8, 255u8]);
    assert!(pixel_brightness(white_pixel) > 0.999);
    let red_pixel = Rgb([255u8, 0u8, 0u8]);
    let green_pixel = Rgb([0u8, 255u8, 0u8]);
    assert!(pixel_brightness(red_pixel) < pixel_brightness(green_pixel));
    let rgba_pixel = Rgba([128u8, 128u8, 128u8, 255u8]);
    let brightness = pixel_brightness(rgba_pixel);
    assert!(
        (brightness - 0.5).abs() < 0.01,
        "RGBA pixel with 50% brightness: got {}",
        brightness
    );
}

#[test]
fn test_image_to_ascii_default_charset() {
    let test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources");
    let test_image = test_dir.join("test_gradient_default_charset.png");
    create_test_image(&test_image);

    let detailed = image_to_ascii(&test_image, 20, 10, CHAR_SET_DETAILED).unwrap();
    let medium = image_to_ascii(&test_image, 20, 10, CHAR_SET_MEDIUM).unwrap();
    let simple = image_to_ascii(&test_image, 20, 10, CHAR_SET_SIMPLE).unwrap();

    assert!(!detailed.is_empty(), "Detailed charset output should not be empty");
    assert!(!medium.is_empty(), "Medium charset output should not be empty");
    assert!(!simple.is_empty(), "Simple charset output should not be empty");

    assert!(detailed.contains('\n'), "Output should contain newlines");
    let lines: Vec<&str> = detailed.lines().collect();
    assert!(lines.len() <= 10, "Output should have at most 10 lines");

    if let Some(first_line) = lines.first() {
        assert!(
            first_line.chars().count() <= 20,
            "Line should have at most 20 characters"
        );
    }

    let output_dir = test_dir.join("output");
    fs::create_dir_all(&output_dir).expect("Failed to create output directory");

    fs::write(output_dir.join("detailed.txt"), &detailed).unwrap();
    fs::write(output_dir.join("medium.txt"), &medium).unwrap();
    fs::write(output_dir.join("simple.txt"), &simple).unwrap();
}
#[test]
fn test_image_to_ascii_custom_charset() {
    let test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources");
    let test_image = test_dir.join("test_custom_charset.png");
    create_test_image(&test_image);
    let custom_charset = "XO-.";
    let result = image_to_ascii(&test_image, 20, 10, custom_charset).unwrap();
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
    fs::write(test_dir.join("output/custom_charset.txt"), &result).unwrap();
}

#[test]
fn test_image_to_ascii_chars_vector() {
    let test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources");
    let test_image = test_dir.join("test_chars_vector.png");
    create_test_image(&test_image);
    let chars_vec = vec!['#', '=', '-', '.', ' '];
    let result = image_to_ascii_chars(&test_image, 20, 10, &chars_vec).unwrap();
    assert!(!result.is_empty(), "Chars vector output should not be empty");
    for c in result.chars() {
        if c != '\n' {
            assert!(
                chars_vec.contains(&c),
                "Character '{}' not in chars vector",
                c
            );
        }
    }
    fs::write(test_dir.join("output/chars_vector.txt"), &result).unwrap();
}

#[test]
fn test_different_image_dimensions() {
    let test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources");
    let test_image = test_dir.join("test_dimensions.png");
    create_test_image(&test_image);

    let small = image_to_ascii(&test_image, 10, 5, CHAR_SET_SIMPLE).unwrap();
    let medium = image_to_ascii(&test_image, 30, 15, CHAR_SET_SIMPLE).unwrap();
    let large = image_to_ascii(&test_image, 50, 25, CHAR_SET_SIMPLE).unwrap();
    assert_eq!(small.lines().count(), 5, "Small output should have 5 lines");
    assert_eq!(medium.lines().count(), 15, "Medium output should have 15 lines");
    assert_eq!(large.lines().count(), 25, "Large output should have 25 lines");

    if let Some(first_line) = small.lines().next() {
        assert_eq!(first_line.chars().count(), 10, "Small line should have 10 chars");
    }
    if let Some(first_line) = medium.lines().next() {
        assert_eq!(first_line.chars().count(), 30, "Medium line should have 30 chars");
    }
    if let Some(first_line) = large.lines().next() {
        assert_eq!(first_line.chars().count(), 50, "Large line should have 50 chars");
    }
}

#[test]
fn test_empty_charset_error() {
    let test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources");
    let test_image = test_dir.join("test_empty_charset_error.png");
    create_test_image(&test_image);

    let result_str = image_to_ascii(&test_image, 20, 10, "");
    assert!(result_str.is_err(), "Str charset: Error expected for empty charset");
    if let Err(err) = result_str {
        let err_string = format!("{:?}", err);
        assert!(
            err_string.contains("Empty character set"),
            "Str charset: Error message should mention empty character set"
        );
    }

    let result_chars = image_to_ascii_chars(&test_image, 20, 10, &Vec::<char>::new());
    assert!(result_chars.is_err(), "Chars vector: Error expected for empty charset");
    if let Err(err) = result_chars {
        let err_string = format!("{:?}", err);
        assert!(
            err_string.contains("Empty character set"),
            "Chars vector: Error message should mention empty character set"
        );
    }
}

#[test]
fn test_image_to_ascii_configurable_aspect_ratio() {
    let test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources");
    let test_image = test_dir.join("test_aspect_ratio.png");
    create_test_image(&test_image);

    let config_aspect_1 = AsciiArtConfig {
        width: 20,
        height: 20,
        aspect_ratio_compensation: 1.0,
        ..Default::default()
    };
    let result_aspect_1 = image_to_ascii_configurable(&test_image, &config_aspect_1).unwrap();
    let lines_aspect_1 = result_aspect_1.lines().count();

    let config_aspect_2 = AsciiArtConfig {
        width: 20,
        height: 20,
        aspect_ratio_compensation: 2.0,
        ..config_aspect_1
    };
    let result_aspect_2 = image_to_ascii_configurable(&test_image, &config_aspect_2).unwrap();
    let lines_aspect_2 = result_aspect_2.lines().count();

    println!("lines_aspect_1: {}", lines_aspect_1);
    println!("lines_aspect_2: {}", lines_aspect_2);

    assert!(lines_aspect_2 < lines_aspect_1, "Lines with aspect 2.0 should be less than with 1.0");
    fs::write(test_dir.join("output/aspect_ratio_1.txt"), &result_aspect_1).unwrap();
    fs::write(test_dir.join("output/aspect_ratio_2.txt"), &result_aspect_2).unwrap();
}

#[test]
fn test_image_to_ascii_configurable_resize_filter() {
    let test_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/resources");
    let test_image = test_dir.join("test_resize_filter.png");
    create_test_image(&test_image);
    let config_lanczos3 = AsciiArtConfig {
        resize_filter: FilterType::Lanczos3,
        width: 20,
        height: 10,
        ..Default::default()
    };
    let result_lanczos3 = image_to_ascii_configurable(&test_image, &config_lanczos3).unwrap();

    let config_nearest = AsciiArtConfig {
        resize_filter: FilterType::Nearest,
        ..config_lanczos3
    };
    let result_nearest = image_to_ascii_configurable(&test_image, &config_nearest).unwrap();

    assert_ne!(result_lanczos3, result_nearest, "Outputs with different filters should be different");

    fs::write(test_dir.join("output/resize_lanczos3.txt"), &result_lanczos3).unwrap();
    fs::write(test_dir.join("output/resize_nearest.txt"), &result_nearest).unwrap();
}
