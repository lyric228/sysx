use crate::types::error::SysxError;
use image::{DynamicImage, GenericImageView, Pixel, imageops::FilterType};
use std::{io, path::Path};

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
///
/// Defines parameters for converting an image to ASCII art.
pub struct AsciiArtConfig {
    /// Target width for the ASCII art in characters.
    pub width: u32,
    /// Maximum target height for the ASCII art in characters.
    /// The actual height is calculated based on the `width`, the original image's aspect ratio,
    /// and `aspect_ratio_compensation`. This value serves as an upper limit.
    pub height: u32,
    /// Compensation factor for character aspect ratio (typically height/width).
    /// Default is 2.0, assuming terminal characters are roughly twice as tall as they are wide.
    /// Must be greater than 0.
    pub aspect_ratio_compensation: f32,
    /// Filter type used for resizing the image.
    pub resize_filter: FilterType,
    /// Character set used for mapping brightness levels. Ordered from lightest to darkest.
    pub char_set: Vec<char>,
    /// Exponent applied to the normalized pixel brightness (0.0-1.0) before mapping to characters.
    /// Lower values (e.g., 0.25) bias towards lighter characters (index 0) for brighter pixels.
    /// A value of 1.0 results in a linear mapping. Default is 0.25.
    pub brightness_exponent: f32,
}

impl Default for AsciiArtConfig {
    fn default() -> Self {
        AsciiArtConfig {
            width: 100,
            height: 50,
            aspect_ratio_compensation: 2.0,
            resize_filter: FilterType::Lanczos3,
            char_set: CHAR_SET_DETAILED.chars().collect::<Vec<char>>(),
            brightness_exponent: 0.25,
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
    if config.aspect_ratio_compensation <= 0.0 {
        return Err(SysxError::ValidationError {
            expected: "Positive aspect ratio compensation".to_string(),
            actual: format!("Compensation factor: {}", config.aspect_ratio_compensation),
            context: Some("Aspect ratio compensation must be greater than 0".to_string()),
        });
    }
    if config.width == 0 {
        return Err(SysxError::ValidationError {
            expected: "Positive width".to_string(),
            actual: "Width: 0".to_string(),
            context: Some("Target width must be greater than 0".to_string()),
        });
    }
     if img.height() == 0 {
        return Err(SysxError::ValidationError {
            expected: "Non-zero image height".to_string(),
            actual: "Image height: 0".to_string(),
            context: Some("Input image height cannot be zero".to_string()),
        });
    }

    let aspect_ratio = img.width() as f32 / img.height() as f32;
    let scaled_width = config.width;
    let calculated_scaled_height =
        (config.width as f32 / aspect_ratio / config.aspect_ratio_compensation).round() as u32;
    let scaled_height = std::cmp::max(1, std::cmp::min(calculated_scaled_height, config.height));

    let resized_img = img.resize_exact(
        scaled_width,
        scaled_height,
        config.resize_filter,
    );

    let mut result =
        String::with_capacity(((scaled_width + 1) * scaled_height) as usize);
    let num_chars = config.char_set.len();
    let num_chars_f = num_chars as f32;

    for y in 0..scaled_height {
        for x in 0..scaled_width {
            let pixel = resized_img.get_pixel(x, y);
            let brightness = pixel_brightness(pixel);
            let adjusted_brightness = brightness.powf(config.brightness_exponent);
            let char_f_index = (1.0 - adjusted_brightness) * num_chars_f;
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

pub fn image_to_ascii_configurable<P>(
    path: P,
    config: &AsciiArtConfig,
) -> Result<String, SysxError>
where
    P: AsRef<Path>,
{
    let img_path = path.as_ref();
    let img = image::open(img_path).map_err(|e| {
        SysxError::Io(io::Error::new(
            io::ErrorKind::Other,
            format!("Could not open or decode image file at path '{}': {}", img_path.display(), e),
        ))
    })?;
    _image_to_ascii_core(img, config)
}

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
    let config = AsciiArtConfig {
        width,
        height,
        char_set: chars,
        ..Default::default()
    };
    image_to_ascii_configurable(path, &config)
}

pub fn pixel_brightness<P: Pixel<Subpixel = u8>>(pixel: P) -> f32 {
    let channels = pixel.to_rgb();
        let r = channels[0] as f32 / 255.0;
        let g = channels[1] as f32 / 255.0;
        let b = channels[2] as f32 / 255.0;
    (0.2126 * r + 0.7152 * g + 0.0722 * b).min(1.0)
    }

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
    let config = AsciiArtConfig {
        width, 
        height,
        char_set: chars.to_vec(),
        ..Default::default()
    };
    image_to_ascii_configurable(path, &config)
}
