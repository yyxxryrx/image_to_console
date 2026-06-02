mod sixel;
mod unicode;

use crate::{
    DisplayMode::{self, *},
    ProcessedImage,
    error::{ConvertError, ConvertErrorContext, ConvertErrorContextSource, ConvertResult},
};
use base64::{Engine, engine::general_purpose::STANDARD};
use rayon::iter::*;
use std::io::Cursor;

/// Represents a pixel color with RGBA components
#[derive(Copy, Clone)]
struct PixelColor {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl PixelColor {
    /// Create a PixelColor from a channel array
    fn from_channels(channels: [u8; 4]) -> Self {
        Self {
            r: channels[0],
            g: channels[1],
            b: channels[2],
            a: channels[3],
        }
    }

    /// Get the background color escape sequence
    fn bg(&self) -> String {
        format!("\x1b[48;2;{};{};{}m", self.r, self.g, self.b)
    }

    /// Get the foreground color escape sequence
    fn fg(&self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.r, self.g, self.b)
    }
}

/// Options for the image converter
#[derive(Debug, Clone)]
pub struct ImageConverterOption {
    /// Whether to center the image
    pub center: bool,
    /// Width of the image
    pub width: u32,
    /// Height of the image
    pub height: u32,
    /// Whether to enable dithering (requires `sixel` feature)
    #[cfg(feature = "sixel")]
    pub dither: bool,
    /// Initial line string
    pub line_init: String,
    /// Display mode
    pub mode: DisplayMode,
    /// Whether to use a black background
    pub black_background: bool,
    /// Whether to enable compression
    pub enable_compression: bool,
    /// Maximum number of colors (requires `sixel` feature)
    #[cfg(feature = "sixel")]
    pub max_colors: u16,
    /// Quantize method to use (requires `sixel` feature)
    #[cfg(feature = "sixel")]
    pub quantize_method: quantette::QuantizeMethod,
    /// Color space to use (requires `sixel` feature)
    #[cfg(feature = "sixel")]
    pub color_space: quantette::ColorSpace,
    /// Terminal size
    pub terminal_size: (u32, u32),
}

impl Default for ImageConverterOption {
    fn default() -> Self {
        Self {
            center: true,
            width: 0,
            height: 0,
            #[cfg(feature = "sixel")]
            dither: false,
            line_init: String::new(),
            mode: Default::default(),
            black_background: false,
            enable_compression: true,
            #[cfg(feature = "sixel")]
            max_colors: 256,
            #[cfg(feature = "sixel")]
            quantize_method: quantette::QuantizeMethod::wu(),
            #[cfg(feature = "sixel")]
            color_space: quantette::ColorSpace::Srgb,
            terminal_size: (0, 0),
        }
    }
}

impl ImageConverterOption {
    /// Creates a new `ImageConverterOption` with the specified center, width, and height
    ///
    /// # Arguments
    ///
    /// * `center` - Whether to center the image
    /// * `width` - Width of the image
    /// * `height` - Height of the image
    ///
    /// # Returns
    ///
    /// Returns a new `ImageConverterOption` instance
    pub fn new(center: bool, width: u32, height: u32) -> Self {
        Self {
            center,
            width,
            height,
            ..Default::default()
        }
    }

    /// Sets whether to enable dithering (requires `sixel` feature)
    ///
    /// # Arguments
    ///
    /// * `dither` - Whether to enable dithering
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to self for chaining
    #[cfg(feature = "sixel")]
    pub fn dither(&mut self, dither: bool) -> &mut Self {
        self.dither = dither;
        self
    }

    /// Sets the initial line string
    ///
    /// # Arguments
    ///
    /// * `line_init` - The initial line string
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to self for chaining
    pub fn line_init(&mut self, line_init: String) -> &mut Self {
        self.line_init = line_init;
        self
    }

    /// Sets the display mode
    ///
    /// # Arguments
    ///
    /// * `mode` - The display mode to use
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to self for chaining
    pub fn mode(&mut self, mode: DisplayMode) -> &mut Self {
        self.mode = mode;
        self
    }

    /// Sets whether to use a black background
    ///
    /// # Arguments
    ///
    /// * `black_background` - Whether to use a black background
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to self for chaining
    pub fn black_background(&mut self, black_background: bool) -> &mut Self {
        self.black_background = black_background;
        self
    }

    /// Sets whether to enable compression
    ///
    /// # Arguments
    ///
    /// * `enable_compression` - Whether to enable compression
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to self for chaining
    pub fn enable_compression(&mut self, enable_compression: bool) -> &mut Self {
        self.enable_compression = enable_compression;
        self
    }

    /// Sets the maximum number of colors (requires `sixel` feature)
    ///
    /// # Arguments
    ///
    /// * `max_colors` - Maximum number of colors
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to self for chaining
    #[cfg(feature = "sixel")]
    pub fn max_colors(&mut self, max_colors: u16) -> &mut Self {
        self.max_colors = max_colors;
        self
    }

    /// Sets the width of the image
    ///
    /// # Arguments
    ///
    /// * `width` - Width of the image
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to self for chaining
    pub fn width(&mut self, width: u32) -> &mut Self {
        self.width = width;
        self
    }

    /// Sets the height of the image
    ///
    /// # Arguments
    ///
    /// * `height` - Height of the image
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to self for chaining
    pub fn height(&mut self, height: u32) -> &mut Self {
        self.height = height;
        self
    }

    /// Sets whether to center the image
    ///
    /// # Arguments
    ///
    /// * `center` - Whether to center the image
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to self for chaining
    pub fn center(&mut self, center: bool) -> &mut Self {
        self.center = center;
        self
    }

    /// Sets the dither method to use (requires `sixel` feature)
    ///
    /// # Arguments
    ///
    /// * `dither_method` - The dither method to use
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to self for chaining
    #[cfg(feature = "sixel")]
    pub fn quantize_method(&mut self, quantize_method: quantette::QuantizeMethod) -> &mut Self {
        self.quantize_method = quantize_method;
        self
    }

    /// Sets the color space to use (requires `sixel` feature)
    ///
    /// # Arguments
    ///
    /// * `color_space` - The color space to use
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to self for chaining
    #[cfg(feature = "sixel")]
    pub fn color_space(&mut self, color_space: quantette::ColorSpace) -> &mut Self {
        self.color_space = color_space;
        self
    }

    /// Returns a copy of the current converter options
    ///
    /// # Returns
    ///
    /// Returns a cloned `ImageConverterOption` instance with the same settings
    pub fn get_options(&self) -> Self {
        self.clone()
    }
}

/// Converts images to terminal-friendly formats
pub struct ImageConverter {
    /// Whether to use full height mode
    full: bool,
    /// Processed image
    img: ProcessedImage,
    /// Converter options
    pub option: ImageConverterOption,
}

impl ImageConverter {
    /// Create a new image converter
    ///
    /// # Arguments
    ///
    /// * `img` - Processed image to convert
    /// * `option` - Converter options
    ///
    /// # Returns
    ///
    /// Returns a new image converter instance
    pub fn new(img: ProcessedImage, option: ImageConverterOption) -> Self {
        Self {
            img,
            full: option.mode.is_full(),
            option,
        }
    }

    /// Convert the image to terminal-friendly format
    ///
    /// # Returns
    ///
    /// Returns a vector of strings representing the converted image
    pub fn convert(&self) -> ConvertResult<Vec<String>> {
        if !self.option.mode.check_image_type(&self.img) {
            return Err(ConvertError::WrongImageType {
                actual_type: self.img.mode().to_string(),
                expect_type: self.option.mode.expect_image_type().to_string(),
            });
        }
        match self.option.mode {
            Kitty | KittyNoColor => self.kitty_convert(),
            Iterm2 | Iterm2NoColor => self.iterm2_convert(),
            WezTerm | WezTermNoColor => self.wezterm_convert(),
            #[cfg(feature = "sixel")]
            SixelHalf | SixelFull => self.sixel_convert(),
            _ => {
                let chunk_size = std::cmp::max(1, self.option.height / num_cpus::get() as u32);

                let convert_pixel = |x, y| match self.option.mode {
                    FullColor => self.full_convert(x, y, false),
                    HalfColor => self.unfull_convert(x, y, false),
                    FullNoColor => self.no_color_convert(x, y),
                    Ascii => self.ascii_convert(x, y),
                    _ => String::new(),
                };
                let mut lines = (if self.full {
                    0..self.option.height / 2
                } else {
                    0..self.option.height
                })
                .into_par_iter()
                .chunks(chunk_size as usize)
                .flat_map(|chunk| {
                    chunk
                        .iter()
                        .map(move |&y| {
                            let mut line = self.option.line_init.clone();
                            if self.option.black_background {
                                line.push_str("\x1b[40m");
                            }
                            let c = (0..self.option.width)
                                .into_par_iter()
                                .map(move |x| convert_pixel(x, y))
                                .collect::<String>();
                            line.push_str(&c);
                            if self.option.mode.is_color() {
                                line.push_str("\x1b[0m");
                            }
                            line
                        })
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<String>>();
                // Maybe the last line is not converted
                if self.full && self.option.height % 2 == 1 {
                    let mut line = self.option.line_init.clone();
                    let c = (0..self.option.width)
                        .into_par_iter()
                        .map(|x| self.full_convert_pixel(x, self.option.height - 1))
                        .collect::<String>();
                    line.push_str(&c);
                    if self.option.mode.is_color() {
                        line.push_str("\x1b[0m");
                    }
                    lines.push(line);
                }
                Ok(lines)
            }
        }
    }

    /// Convert a pixel in half-height color mode
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate
    /// * `y` - Y coordinate
    /// * `only_color` - Whether to return only the color information
    ///
    /// # Returns
    ///
    /// Returns a string representing the converted pixel
    fn unfull_convert(&self, x: u32, y: u32, only_color: bool) -> String {
        if let ProcessedImage::Color(rgba_img) = &self.img {
            let pixel = rgba_img.get_pixel(x, y);
            let color = PixelColor::from_channels(pixel.0);
            let cur_color = if color.a >= 128 {
                color.bg()
            } else {
                "\x1b[0m".to_string()
            };
            if only_color {
                return cur_color;
            }
            let last_color = if x > 0 && self.option.enable_compression {
                self.unfull_convert(x - 1, y, true)
            } else {
                String::new()
            };
            if last_color == cur_color {
                "  ".to_string()
            } else {
                format!("{}  ", cur_color)
            }
        } else {
            panic!("Invalid image type")
        }
    }

    /// Convert a pixel in full-height color mode
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate
    /// * `y` - Y coordinate
    /// * `only_color` - Whether to return only the color information
    ///
    /// # Returns
    ///
    /// Returns a string representing the converted pixel
    fn full_convert(&self, x: u32, y: u32, only_color: bool) -> String {
        if let ProcessedImage::Both(rgba_img, luma_img) = &self.img {
            let pixel1 = rgba_img.get_pixel(x, y * 2);
            let pixel2 = rgba_img.get_pixel(x, y * 2 + 1);
            let p1 = luma_img.get_pixel(x, y * 2).0[0];
            let p2 = luma_img.get_pixel(x, y * 2 + 1).0[0];
            let pixel1_color = PixelColor::from_channels(pixel1.0);
            let pixel2_color = PixelColor::from_channels(pixel2.0);
            let cur_color = if pixel1_color.a < 128 && pixel2_color.a < 128 {
                "\x1b[0m".to_string()
            } else if pixel1_color.a < 128 {
                format!("\x1b[0m{}", pixel2_color.fg())
            } else if pixel2_color.a < 128 {
                format!("\x1b[0m{}", pixel1_color.fg())
            } else if p1 > p2 {
                format!("{}{}", pixel1_color.fg(), pixel2_color.bg())
            } else if p2 > p1 {
                format!("{}{}", pixel1_color.bg(), pixel2_color.fg())
            } else {
                if self.option.enable_compression {
                    pixel1_color.bg()
                } else {
                    pixel1_color.fg()
                }
            };
            if only_color {
                return cur_color;
            }
            let last_color = if x > 0 && self.option.enable_compression {
                self.full_convert(x - 1, y, true)
            } else {
                String::new()
            };
            let cur_char = if pixel1_color.a < 128 && pixel2_color.a < 128 {
                " "
            } else if pixel1_color.a < 128 {
                "▄"
            } else if p1 > p2 {
                "▀"
            } else if p2 > p1 {
                "▄"
            } else if self.option.enable_compression {
                " "
            } else {
                "█"
            };
            if cur_color == last_color
                || (cur_char == " "
                    && last_color.contains(&cur_color)
                    && self.option.enable_compression)
            {
                cur_char.to_string()
            } else {
                format!("{}{}", cur_color, cur_char)
            }
        } else {
            panic!("Invalid image type")
        }
    }

    /// Convert a single pixel at the bottom row in full mode
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate
    /// * `y` - Y coordinate
    ///
    /// # Returns
    ///
    /// Returns a string representing the converted pixel
    fn full_convert_pixel(&self, x: u32, y: u32) -> String {
        if let ProcessedImage::Both(rgba_img, _) = &self.img {
            let pixel = rgba_img.get_pixel(x, y);
            let color = PixelColor::from_channels(pixel.0);
            format!("{}▀", color.fg())
        } else if let ProcessedImage::NoColor(luma_img) = &self.img {
            let pixel = luma_img.get_pixel(x, y);
            if pixel.0[0] > 128 {
                "▀".to_string()
            } else {
                " ".to_string()
            }
        } else {
            panic!("Invalid image type")
        }
    }

    /// Convert pixels in no-color mode
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate
    /// * `y` - Y coordinate
    ///
    /// # Returns
    ///
    /// Returns a string representing the converted pixel
    fn no_color_convert(&self, x: u32, y: u32) -> String {
        if let ProcessedImage::NoColor(luma_img) = &self.img {
            unicode::luma_convert(luma_img, x, y)
        } else {
            panic!("Invalid image type")
        }
    }

    /// Get image data as bytes
    ///
    /// # Returns
    ///
    /// Returns a vector of bytes representing the image data
    fn get_image_data(&self) -> ConvertResult<Vec<u8>> {
        let mut buffer = Vec::new();
        let mut writer = Cursor::new(&mut buffer);
        if self.img.is_color() {
            self.img
                .rgba()
                .unwrap()
                .write_to(&mut writer, image::ImageFormat::Png)
                .map_err(|e| {
                    ConvertError::ImageError(ConvertErrorContext::new(
                        ConvertErrorContextSource::Function("get_image_data".to_string()),
                        e.to_string(),
                    ))
                })?;
        } else {
            self.img
                .luma()
                .unwrap()
                .write_to(&mut writer, image::ImageFormat::Png)
                .map_err(|e| {
                    ConvertError::ImageError(ConvertErrorContext::new(
                        ConvertErrorContextSource::Function("get_image_data".to_string()),
                        e.to_string(),
                    ))
                })?;
        }
        Ok(buffer)
    }

    /// Convert pixels in ASCII mode
    ///
    /// # Arguments
    ///
    /// * `x` - X coordinate
    /// * `y` - Y coordinate
    ///
    /// # Returns
    ///
    /// Returns a string representing the converted pixel using ASCII characters
    fn ascii_convert(&self, x: u32, y: u32) -> String {
        if let ProcessedImage::NoColor(luma_img) = &self.img {
            const ASCII_CHARS: [&str; 12] =
                [" ", ".", ",", ":", ";", "+", "*", "?", "%", "S", "#", "@"];
            let pixel = luma_img.get_pixel(x, y);
            let p = pixel.0[0] as usize;
            let unit = 256 / ASCII_CHARS.len();
            for i in (0..ASCII_CHARS.len()).rev() {
                if i * unit <= p {
                    return ASCII_CHARS[i].repeat(2);
                }
            }
            "  ".to_string()
        } else {
            panic!("Invalid image type")
        }
    }

    /// Convert image using WezTerm protocol
    ///
    /// # Returns
    ///
    /// Returns a vector of strings representing the converted image
    fn wezterm_convert(&self) -> ConvertResult<Vec<String>> {
        let image_data = self.get_image_data()?;
        // Add space to prevent misalignment
        let mut lines: Vec<String> = vec![String::new()];
        lines[0] = if !self.option.center {
            format!(
                "\x1b]1337;File=size={};inline=1:{}\x1b\\",
                image_data.len(),
                STANDARD.encode(image_data)
            )
        } else {
            let (w, h) = self.option.terminal_size;
            let r = self.option.width as f32 / self.option.height as f32;
            let tr = w as f32 / h as f32;
            format!(
                "{}\x1b]1337;File=size={};{}={};inline=1:{}\x1b\\",
                self.option.line_init,
                image_data.len(),
                if r < tr { "height" } else { "width" },
                if r < tr { h } else { w },
                STANDARD.encode(image_data)
            )
        };
        Ok(lines)
    }

    /// Convert image using Kitty protocol
    ///
    /// # Returns
    ///
    /// Returns a vector of strings representing the converted image
    fn kitty_convert(&self) -> ConvertResult<Vec<String>> {
        /// Base64 encodes 3 raw bytes → 4 ASCII bytes.
        ///
        /// 3072 raw bytes / 3 * 4 = 4096 encoded bytes.
        ///
        /// Thus CHUNK_SIZE = 3072 keeps every Kitty payload ≤ 4096 bytes after encoding.
        ///
        /// This matches the Python example, which splits *after* encoding.
        const CHUNK_SIZE: usize = 3072;
        let image_data = self.get_image_data()?;
        let mut chunks = image_data.chunks(CHUNK_SIZE);
        let mut line = format!(
            "{}\x1b_Gm={},a=T,f=100,s={},v={},S={};",
            self.option.line_init,
            chunks.len().saturating_sub(1).min(1),
            self.option.width,
            self.option.height,
            image_data.len()
        );

        line.push_str(&STANDARD.encode(chunks.nth(0).ok_or(ConvertError::EmptyData)?));
        line.push_str("\x1b\\");
        if chunks.len() > 0 {
            for chunk in chunks.clone().take(chunks.len() - 1) {
                line.push_str(&format!("\x1b_Gm=1;{}\x1b\\", STANDARD.encode(chunk)));
            }
            // It is definitely available here, so there is no need to check
            line.push_str(&format!(
                "\x1b_Gm=0;{}\x1b\\",
                STANDARD.encode(unsafe { chunks.last().unwrap_unchecked() })
            ));
        }
        Ok(vec![line])
    }

    /// Convert image using ITerm2 protocol
    ///
    /// # Returns
    ///
    /// Returns a vector of strings representing the converted image
    fn iterm2_convert(&self) -> ConvertResult<Vec<String>> {
        let image_data = self.get_image_data()?;
        Ok(vec![if !self.option.center {
            format!(
                "\x1b]1337;File=size={};inline=1:{}\x07",
                image_data.len(),
                STANDARD.encode(image_data)
            )
        } else {
            let (w, h) = self.option.terminal_size;
            let r = self.option.width as f32 / self.option.height as f32;
            let tr = w as f32 / h as f32;
            format!(
                "{}\x1b]1337;File=size={};{}={};inline=1:{}\x07",
                self.option.line_init,
                image_data.len(),
                if r < tr { "height" } else { "width" },
                if r < tr { h } else { w },
                STANDARD.encode(image_data)
            )
        }])
    }

    /// Convert image using Sixel protocol
    ///
    /// # Returns
    ///
    /// Returns a vector of strings representing the converted image
    #[cfg(feature = "sixel")]
    fn sixel_convert(&self) -> ConvertResult<Vec<String>> {
        sixel::convert(self.img.rgb().unwrap(), self.full, &self.option)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_color_creation() {
        let color = PixelColor::from_channels([255, 128, 64, 255]);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 64);
        assert_eq!(color.a, 255);
    }

    #[test]
    fn test_pixel_color_bg() {
        let color = PixelColor::from_channels([255, 128, 64, 255]);
        let bg = color.bg();
        assert_eq!(bg, "\x1b[48;2;255;128;64m");
    }

    #[test]
    fn test_pixel_color_fg() {
        let color = PixelColor::from_channels([255, 128, 64, 255]);
        let fg = color.fg();
        assert_eq!(fg, "\x1b[38;2;255;128;64m");
    }
}
