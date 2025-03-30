//! ASCII art module for converting images to ASCII representation
//!
//! This module provides functionality to convert images of various formats
//! into ASCII art, with customizable character sets and output dimensions.

use crate::types::error::SysxError;
use image::{GenericImageView, Pixel};
use std::path::Path;

/// Default character set with high density - many different brightness levels
pub const CHAR_SET_DETAILED: &str =
    "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";

/// Medium density character set - fewer characters but still good detail
pub const CHAR_SET_MEDIUM: &str = "@%#*+=-:. ";

/// Minimal character set for simple ASCII art
pub const CHAR_SET_SIMPLE: &str = "@#*+:. ";

/// Convert an image to ASCII art
pub fn image_to_ascii<P, C>(
    path: P,
    width: u32,
    height: u32,
    char_set: C,
) -> Result<String, SysxError>
where
    P: AsRef<Path>,
    C: AsRef<str>,
{
    let img = image::open(path.as_ref())
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to open image: {}", e),
            )
        })
        .map_err(SysxError::from)?;

    let aspect_ratio = img.width() as f32 / img.height() as f32;
    let scaled_width = width;
    let scaled_height = (width as f32 / aspect_ratio / 2.0) as u32;

    let scaled_height = std::cmp::min(scaled_height, height);

    let img = img.resize_exact(
        scaled_width,
        scaled_height,
        image::imageops::FilterType::Lanczos3,
    );

    let char_set = char_set.as_ref();
    let chars: Vec<char> = char_set.chars().collect();

    if chars.is_empty() {
        return Err(SysxError::ValidationError {
            expected: "Non-empty character set".to_string(),
            actual: "Empty character set".to_string(),
            context: Some("ASCII conversion requires at least one character".to_string()),
        });
    }

    let mut result =
        String::with_capacity((scaled_width * scaled_height) as usize + scaled_height as usize);

    for y in 0..scaled_height {
        for x in 0..scaled_width {
            let pixel = img.get_pixel(x, y);

            let brightness = pixel_brightness(pixel);

            let char_index =
                ((brightness * (chars.len() as f32 - 1.0)) as usize).min(chars.len() - 1);
            result.push(chars[char_index]);
        }
        result.push('\n');
    }

    Ok(result)
}

/// Calculate brightness of a pixel, taking into account human perception of colors
pub fn pixel_brightness<P: Pixel<Subpixel = u8>>(pixel: P) -> f32 {
    let channels = pixel.channels();

    if channels.len() >= 3 {
        let r = channels[0] as f32 / 255.0;
        let g = channels[1] as f32 / 255.0;
        let b = channels[2] as f32 / 255.0;

        return (0.299 * r + 0.587 * g + 0.114 * b).min(1.0);
    }

    channels[0] as f32 / 255.0
}

/// Alternative function that allows using a collection of chars
pub fn image_to_ascii_with_chars<P, C>(
    path: P,
    width: u32,
    height: u32,
    char_set: C,
) -> Result<String, SysxError>
where
    P: AsRef<Path>,
    C: AsRef<[char]>,
{
    let img = image::open(path.as_ref())
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to open image: {}", e),
            )
        })
        .map_err(SysxError::from)?;

    let aspect_ratio = img.width() as f32 / img.height() as f32;
    let scaled_width = width;
    let scaled_height = (width as f32 / aspect_ratio / 2.0) as u32;

    let scaled_height = std::cmp::min(scaled_height, height);

    let img = img.resize_exact(
        scaled_width,
        scaled_height,
        image::imageops::FilterType::Lanczos3,
    );

    let chars = char_set.as_ref();

    if chars.is_empty() {
        return Err(SysxError::ValidationError {
            expected: "Non-empty character set".to_string(),
            actual: "Empty character set".to_string(),
            context: Some("ASCII conversion requires at least one character".to_string()),
        });
    }

    let mut result =
        String::with_capacity((scaled_width * scaled_height) as usize + scaled_height as usize);

    for y in 0..scaled_height {
        for x in 0..scaled_width {
            let pixel = img.get_pixel(x, y);

            let brightness = pixel_brightness(pixel);

            let char_index =
                ((brightness * (chars.len() as f32 - 1.0)) as usize).min(chars.len() - 1);
            result.push(chars[char_index]);
        }
        result.push('\n');
    }

    Ok(result)
}
