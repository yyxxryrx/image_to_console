use crate::ResizeMode::{Auto, Custom, None};
use crate::converter::{ImageConverter, ImageConverterOption};
use crate::error::ConvertResult;
use crate::{AutoResizeOption, DisplayMode, ProcessedImage, ResizeMode};
use image::{GenericImageView, imageops::FilterType};
use std::default::Default;

/// Image processor options
///
/// Configures various parameters for image processing
#[derive(Debug, Copy, Clone)]
pub struct ImageProcessorOptions {
    /// Whether to use full height mode
    pub full: bool,
    /// Whether to center the display
    pub center: bool,
    /// Whether to enable dithering (requires `sixel` feature)
    #[cfg(feature = "sixel")]
    pub dither: bool,
    /// Display mode
    pub mode: DisplayMode,
    /// Whether to use a black background
    pub black_background: bool,
    /// Resize mode
    pub resize_mode: ResizeMode,
    /// Whether to enable compression
    pub enable_compression: bool,
    /// Maximum number of colors (requires `sixel` feature)
    #[cfg(feature = "sixel")]
    pub max_colors: u16,
}

impl Default for ImageProcessorOptions {
    fn default() -> Self {
        Self {
            full: true,
            center: false,
            #[cfg(feature = "sixel")]
            dither: true,
            mode: DisplayMode::FullColor,
            black_background: false,
            resize_mode: Auto(AutoResizeOption {
                width: true,
                height: true,
            }),
            enable_compression: true,
            #[cfg(feature = "sixel")]
            max_colors: 256,
        }
    }
}

pub trait ImageProcessorOptionsCreate<T> {
    /// Create a new image processor
    ///
    /// # Arguments
    ///
    /// * `image` - Image to be processed
    ///
    /// # Returns
    ///
    /// Returns a new image processor instance

    fn create_processor(&self, image: T) -> ImageProcessor;
}

impl ImageProcessorOptionsCreate<image::DynamicImage> for ImageProcessorOptions {
    fn create_processor(&self, image: image::DynamicImage) -> ImageProcessor {
        ImageProcessor::new(image, self.clone())
    }
}

impl ImageProcessorOptionsCreate<&image::DynamicImage> for ImageProcessorOptions {
    fn create_processor(&self, image: &image::DynamicImage) -> ImageProcessor {
        ImageProcessor::new(image.clone(), self.clone())
    }
}

impl ImageProcessorOptions {
    /// Create a new ImageProcessorOptions instance
    ///
    /// # Arguments
    ///
    /// * `mode` - Display mode
    /// * `resize` - Resize mode
    /// * `center` - Whether to center the display
    ///
    /// # Returns
    ///
    /// Returns a configured ImageProcessorOptions instance

    pub fn new(mode: DisplayMode, resize: ResizeMode, center: bool) -> Self {
        Self {
            mode,
            center,
            #[cfg(feature = "sixel")]
            dither: true,
            full: mode.is_full(),
            resize_mode: resize,
            black_background: false,
            enable_compression: true,
            #[cfg(feature = "sixel")]
            max_colors: 256,
        }
    }

    /// Set display mode option
    ///
    /// # Arguments
    ///
    /// * `mode` - Display mode to set
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to self for method chaining

    pub fn option_display_mode(&mut self, mode: DisplayMode) -> &mut Self {
        self.mode = mode;
        self.full = mode.is_full();
        self
    }

    /// Set resize mode option
    ///
    /// # Arguments
    ///
    /// * `resize` - Resize mode to set
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to self for method chaining

    pub fn option_resize(&mut self, resize: ResizeMode) -> &mut Self {
        self.resize_mode = resize;
        self
    }

    /// Set center option
    ///
    /// # Arguments
    ///
    /// * `center` - Whether to center the display
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to self for method chaining

    pub fn option_center(&mut self, center: bool) -> &mut Self {
        self.center = center;
        self
    }

    /// Set black background option
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether to enable black background
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to self for method chaining

    pub fn option_black_background(&mut self, enabled: bool) -> &mut Self {
        self.black_background = enabled;
        self
    }

    /// Set compression option
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether to enable compression
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to self for method chaining

    pub fn option_compression(&mut self, enabled: bool) -> &mut Self {
        self.enable_compression = enabled;
        self
    }

    /// Set maximum colors option (requires `sixel` feature)
    ///
    /// # Arguments
    ///
    /// * `max_colors` - Maximum number of colors
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to self for method chaining

    #[cfg(feature = "sixel")]
    pub fn option_max_colors(&mut self, max_colors: u16) -> &mut Self {
        self.max_colors = max_colors;
        self
    }

    /// Set dithering option (requires `sixel` feature)
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether to enable dithering
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to self for method chaining

    #[cfg(feature = "sixel")]
    pub fn option_dither(&mut self, enabled: bool) -> &mut Self {
        self.dither = enabled;
        self
    }

    pub fn get_options(&self) -> ImageProcessorOptions {
        self.clone()
    }
}

/// Image processing result
///
/// Contains the processed image data and related information
#[derive(Debug)]
pub struct ImageProcessorResult {
    /// Image width
    pub width: u32,
    /// Image height
    pub height: u32,
    /// Number of empty lines
    pub air_lines: usize,
    /// Processed line data
    pub lines: Vec<String>,
    /// Processing time
    pub time: std::time::Instant,
    /// Processing options
    pub option: ImageProcessorOptions,
}

/// A display wrapper for ImageProcessorResult
///
/// This struct is responsible for formatting and displaying the processed image result
/// It handles the vertical spacing (air lines) and line formatting for terminal output
pub struct ImageProcessorResultDisplay {
    air_lines: usize,
    lines: Vec<String>,
}

impl std::fmt::Display for ImageProcessorResultDisplay {
    /// Formats the image result for display in terminal
    ///
    /// # Arguments
    ///
    /// * `f` - Formatter instance
    ///
    /// # Returns
    ///
    /// Formatting result
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            "\n".repeat(self.air_lines),
            self.lines.join("\n")
        )
    }
}

impl ImageProcessorResultDisplay {
    /// Creates a new ImageProcessorResultDisplay instance
    ///
    /// # Arguments
    ///
    /// * `air_lines` - Number of empty lines to prepend
    /// * `lines` - Processed image lines to display
    ///
    /// # Returns
    ///
    /// New ImageProcessorResultDisplay instance
    fn new(air_lines: usize, lines: Vec<String>) -> Self {
        Self { lines, air_lines }
    }
}

impl ImageProcessorResult {
    /// Creates a display wrapper for the image result
    ///
    /// This method creates an ImageProcessorResultDisplay that can be used
    /// to format the image for terminal output with proper spacing
    ///
    /// # Returns
    ///
    /// ImageProcessorResultDisplay instance for formatting the result
    pub fn display(&self) -> ImageProcessorResultDisplay {
        ImageProcessorResultDisplay::new(self.air_lines, self.lines.clone())
    }
}

/// Image processor
///
/// Responsible for processing images into terminal-friendly formats
pub struct ImageProcessor {
    /// Image to be processed
    pub image: image::DynamicImage,
    /// Processing options
    pub option: ImageProcessorOptions,
}

impl ImageProcessor {
    /// Create a new image processor
    ///
    /// # Arguments
    ///
    /// * `image` - Image to be processed
    /// * `option` - Processing options
    ///
    /// # Returns
    ///
    /// Returns a new image processor instance
    pub fn new(image: image::DynamicImage, option: ImageProcessorOptions) -> Self {
        Self { image, option }
    }

    /// Process the image
    ///
    /// Processes the image according to configuration options and returns the result
    ///
    /// # Returns
    ///
    /// Returns the processed result
    pub fn process(&mut self) -> ConvertResult<ImageProcessorResult> {
        let time = std::time::Instant::now();
        let mut air_line: usize = 0;
        let (mut w, mut h) = self.image.dimensions();
        let (width, height) = terminal_size::terminal_size().unwrap();
        match self.option.resize_mode {
            Auto(option) => {
                if self.option.mode.is_normal() {
                    if option.width {
                        if w > (width.0 / if self.option.full { 1 } else { 2 }) as u32 {
                            let new_img = self.image.resize(
                                (width.0 as f32 / if self.option.full { 1f32 } else { 2f32 })
                                    .round() as u32,
                                h,
                                FilterType::Lanczos3,
                            );
                            (w, h) = new_img.dimensions();
                            self.image = new_img;
                        }
                    }
                    if option.height {
                        if h > (height.0 * if self.option.full { 2 } else { 1 }) as u32 {
                            let new_img = self.image.resize(
                                w,
                                (height.0 * if self.option.full { 2 } else { 1 }) as u32,
                                FilterType::Lanczos3,
                            );
                            (w, h) = new_img.dimensions();
                            self.image = new_img;
                        }
                    }
                }
                #[cfg(feature = "sixel")]
                if self.option.mode.is_sixel() {
                    if option.width {
                        if w > width.0 as u32 * if self.option.full { 12 } else { 6 } {
                            let new_img = self.image.resize(
                                width.0 as u32 * if self.option.full { 12 } else { 6 },
                                h,
                                FilterType::Lanczos3,
                            );
                            (w, h) = new_img.dimensions();
                            self.image = new_img;
                        }
                    }
                    if option.height {
                        if h > height.0 as u32 * if self.option.full { 21 } else { 10 } {
                            let new_img = self.image.resize(
                                w,
                                height.0 as u32 * if self.option.full { 21 } else { 10 },
                                FilterType::Lanczos3,
                            );
                            (w, h) = new_img.dimensions();
                            self.image = new_img;
                        }
                    }
                }
            }
            Custom(option) => {
                if option.width.is_some() || option.height.is_some() {
                    let width = option.width.unwrap_or(w);
                    let height = option.height.unwrap_or(h);
                    let new_img = self.image.resize_exact(width, height, FilterType::Lanczos3);
                    (w, h) = new_img.dimensions();
                    self.image = new_img;
                }
            }
            None => {}
        }
        let mut line_init = String::new();
        if self.option.center {
            if self.option.mode.is_normal() {
                if !self.option.full && h < height.0 as u32
                    || self.option.full && h < height.0 as u32 / 2
                {
                    for _ in 0..(height.0 / 2) as u32 - h / if self.option.full { 4 } else { 2 } {
                        air_line += 1;
                    }
                }

                if w < width.0 as u32 / if self.option.full { 1 } else { 2 } {
                    let len = (width.0 as f32 / 2f32
                        - w as f32 / if self.option.full { 2f32 } else { 1f32 })
                    .round() as usize;
                    line_init.push_str(&" ".repeat(len));
                }
            }

            #[cfg(not(feature = "sixel"))]
            let not_normal = !self.option.mode.is_normal();
            #[cfg(feature = "sixel")]
            let not_normal = !(self.option.mode.is_normal() || self.option.mode.is_sixel());
            if not_normal {
                air_line = height.0 as usize;
                let terminal_rate = width.0 as f64 / height.0 as f64 / 2f64;
                let rate = w as f64 / h as f64;
                // println!("{} {}", terminal_rate, rate);
                if rate < terminal_rate {
                    let w = (height.0 as f64 * rate).floor() as u16;
                    let offset = width.0 / 2 - w;
                    line_init.push_str(&format!(
                        "\x1b[1;{}H",
                        offset - if rate > 1.0 { offset / 2 } else { 0 }
                    ));
                } else if rate == terminal_rate {
                    if width.0 > height.0 {
                        let w = height.0;
                        let offset = (width.0 - w) / 2;
                        line_init.push_str(&format!("\x1b[1;{}H", offset - offset / 2));
                    } else if width.0 < height.0 {
                        let h = width.0 / 2;
                        let offset = (height.0 * 2 - h) / 2;
                        line_init.push_str(&format!("\x1b[{};1H", offset - offset / 2));
                    }
                } else {
                    let h = (width.0 as f64 / 2f64 / rate).floor() as u16;
                    let offset = height.0 * 2 - h;
                    line_init.push_str(&format!("\x1b[{};1H", offset - offset / 2));
                }
            }
        }
        let converter = ImageConverter::new(
            ProcessedImage::new(self.option.mode, &self.image),
            ImageConverterOption {
                center: self.option.center,
                width: w,
                height: h,
                line_init,
                mode: self.option.mode,
                #[cfg(feature = "sixel")]
                dither: self.option.dither,
                black_background: self.option.black_background,
                enable_compression: self.option.enable_compression,
                #[cfg(feature = "sixel")]
                max_colors: self.option.max_colors,
                ..ImageConverterOption::default()
            },
        );
        Ok(ImageProcessorResult {
            time,
            width: w,
            height: h,
            air_lines: air_line,
            lines: converter.convert()?,
            option: self.option,
        })
    }
}
