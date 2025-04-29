// src/utils/ascii.rs
use crate::types::error::SysxError;
use image::{GenericImageView, Pixel, DynamicImage, imageops::FilterType};
use std::{path::Path, io};

/// Ultra detailed character set with maximum brightness levels (94 characters)
pub const CHAR_SET_VERY_DETAILED: &str =
    "@QB#NgWM8RDHdOKq9$6khEPXwmeZaoS2yjufF]}{tx1zv7lciL/\\|?*>r^;:_\"~,'.-` ";

/// Detailed character set (70 characters) 
pub const CHAR_SET_DETAILED: &str =
    "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^ ";

/// Medium density character set (14 characters)
pub const CHAR_SET_MEDIUM: &str = "@%#*+=-:;,.~ ";

/// Minimal character set for simple ASCII art (7 characters)
pub const CHAR_SET_SIMPLE: &str = "@#*+:. ";

/// Configuration for ASCII art conversion.
pub struct AsciiArtConfig {
    pub width: u32,
    pub height: u32,
    pub aspect_ratio_compensation: f32,
    pub resize_filter: FilterType,
    pub char_set: Vec<char>,
}

impl Default for AsciiArtConfig {
    fn default() -> Self {
        AsciiArtConfig {
            width: 100,
            height: 50,
            aspect_ratio_compensation: 2.0,
            resize_filter: FilterType::Lanczos3,
            char_set: CHAR_SET_DETAILED.chars().collect::<Vec<char>>(),
        }
    }
}

fn _image_to_ascii_core(
    img: DynamicImage,
    config: &AsciiArtConfig,
) -> Result<String, SysxError> {
    if config.char_set.is_empty() {
        return Err(SysxError::ValidationError {
            expected: "Non-empty character set".to_string(),
            actual: "Empty character set".to_string(),
            context: Some("ASCII conversion requires at least one character".to_string()),
        });
    }

    let aspect_ratio = img.width() as f32 / img.height() as f32;
    let scaled_width = config.width;
    let calculated_scaled_height = (config.width as f32 / aspect_ratio / config.aspect_ratio_compensation) as u32;
    let scaled_height = std::cmp::min(calculated_scaled_height, config.height);

    println!("aspect_ratio_compensation: {}", config.aspect_ratio_compensation);
    println!("scaled_height (calculated): {}", calculated_scaled_height);
    println!("scaled_height (min applied): {}", scaled_height);

    let resized_img = img.resize_exact(
        scaled_width,
        scaled_height,
        config.resize_filter,
    );

    let mut result =
        String::with_capacity((scaled_width * scaled_height) as usize + scaled_height as usize);

    for y in 0..scaled_height {
        for x in 0..scaled_width {
            let pixel = resized_img.get_pixel(x, y);
            let brightness = pixel_brightness(pixel);
            let char_index =
                ((brightness * (config.char_set.len() as f32 - 1.0)) as usize).min(config.char_set.len() - 1);
            result.push(config.char_set[char_index]);
        }
        result.push('\n');
    }
    Ok(result)
}

/// Convert an image to ASCII art with customizable options.
pub fn image_to_ascii_configurable<P>(
    path: P,
    config: &AsciiArtConfig,
) -> Result<String, SysxError>
where
    P: AsRef<Path>,
{
    let img = image::open(path.as_ref()).map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Could not open image file at path '{}': {}", path.as_ref().display(), e),
        )
    }).map_err(SysxError::from)?;
    _image_to_ascii_core(img, config)
}

/// Convert an image to ASCII art using a character set string.
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
    let chars: Vec<char> = char_set.as_ref().chars().collect();
    if chars.is_empty() {
        return Err(SysxError::ValidationError {
            expected: "Non-empty character set".to_string(),
            actual: "Empty character set".to_string(),
            context: Some("ASCII conversion requires at least one character".to_string()),
        });
    }

    let config = AsciiArtConfig {
        width,
        height,
        char_set: chars,
        ..Default::default()
    };
    image_to_ascii_configurable(path, &config)
}

/// Calculate brightness of a pixel, taking into account human perception of colors
/// Uses standard luminance formula: 0.299*R + 0.587*G + 0.114*B
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

/// Alternative function that allows using a collection of chars.
pub fn image_to_ascii_chars<P, C>(
    path: P,
    width: u32,
    height: u32,
    char_set: C,
) -> Result<String, SysxError>
where
    P: AsRef<Path>,
    C: AsRef<[char]>,
{
    let config = AsciiArtConfig {
        width, 
        height,
        char_set: char_set.as_ref().to_vec(),
        ..Default::default()
    };
    image_to_ascii_configurable(path, &config)
}
