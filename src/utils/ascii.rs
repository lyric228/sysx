use crate::types::error::SysxError;
use image::{GenericImageView, Pixel, DynamicImage, imageops::FilterType};
use std::{path::Path, io};

// Character sets ordered from LIGHTEST to DARKEST for better mapping
/// Ultra detailed character set (94 characters) - Ordered Lightest to Darkest
pub const CHAR_SET_VERY_DETAILED: &str =
    " `-.,'_:;^r*?/\\|()[]{}1LctvunxrjfmewpqaokSZEPX69RdHBMN#WQ@";

/// Detailed character set (70 characters) - Ordered Lightest to Darkest
pub const CHAR_SET_DETAILED: &str =
    " `.'\",:;!ilI><~+_-?][}{)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
/// Medium density character set (11 characters) - Ordered Lightest to Darkest
pub const CHAR_SET_MEDIUM: &str = " .,:;-+=*#%@";

/// Minimal character set (7 characters) - Ordered Lightest to Darkest
pub const CHAR_SET_SIMPLE: &str = " .:*#=@";

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
            aspect_ratio_compensation: 2.0, // Default aspect ratio compensation for typical terminal character aspect
            resize_filter: FilterType::Lanczos3,
            char_set: CHAR_SET_DETAILED.chars().collect::<Vec<char>>(), // Default uses the detailed set
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
    // Aspect ratio compensation is applied to height calculation
    let calculated_scaled_height = (config.width as f32 / aspect_ratio / config.aspect_ratio_compensation).round() as u32;
    // Ensure height doesn't exceed the configured maximum height and is at least 1
    let scaled_height = std::cmp::max(1, std::cmp::min(calculated_scaled_height, config.height));

    let resized_img = img.resize_exact(
        scaled_width,
        scaled_height,
        config.resize_filter,
    );

    let mut result =
        String::with_capacity(((scaled_width + 1) * scaled_height) as usize);

    let num_chars = config.char_set.len();
    if num_chars == 0 {
        return Err(SysxError::ValidationError {
            expected: "Non-empty character set".to_string(),
            actual: "Empty character set".to_string(),
            context: Some("Internal check failed: character set became empty unexpectedly after initial validation.".to_string()),
        });
    }
    let num_chars_f = num_chars as f32;

    for y in 0..scaled_height {
        for x in 0..scaled_width {
            let pixel = resized_img.get_pixel(x, y);
            let brightness = pixel_brightness(pixel);

            let char_f_index = (1.0 - brightness) * num_chars_f;
            let mut char_index = char_f_index.floor() as usize;

            if char_index >= num_chars {
                char_index = num_chars - 1;
            }

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
/// Uses default aspect ratio compensation (2.0) and Lanczos3 filter.
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

/// Calculate brightness of a pixel using standard luminance formula.
/// Returns a value between 0.0 (black) and 1.0 (white).
pub fn pixel_brightness<P: Pixel<Subpixel = u8>>(pixel: P) -> f32 {
    let channels = pixel.to_rgb();
        let r = channels[0] as f32 / 255.0;
        let g = channels[1] as f32 / 255.0;
        let b = channels[2] as f32 / 255.0;

    (0.2126 * r + 0.7152 * g + 0.0722 * b).min(1.0)
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
    let chars = char_set.as_ref();
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
        char_set: chars.to_vec(),
        ..Default::default()
    };
    image_to_ascii_configurable(path, &config)
}
