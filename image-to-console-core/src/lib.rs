/*!
* # image_to_console_core
*
* A library for converting images to terminal friendly format.
*
* ## Basic usage example
* ```rust
* use image::error::ImageResult;
* use image_to_console_core::processor::{ImageProcessor, ImageProcessorOptions};
*
* fn main() -> ImageResult<()> {
*     let img = image::open("path/to/image.png")?;
*
*     // Use default config
*     let option = ImageProcessorOptions::default();
*
*     let mut processor = ImageProcessor::new(img, option);
*     let result = processor.process();
*     // Exception handling (this is only shown, not handled, please refer to the actual use of the need)
*     let result = result.expect("Process image failed");
*     // result.lines contains the formatted terminal output
*     // you also can use display method to print
*     println!("{}", result.display());
*     Ok(())
* }
* ```
*/

pub mod converter;
pub mod error;
#[cfg(feature = "gif")]
pub mod gif_processor;
#[cfg(feature = "sixel")]
pub mod indexed_image;
pub mod processor;
pub mod protocol;

pub extern crate image;
#[cfg(feature = "sixel")]
pub extern crate quantette;
pub extern crate rayon;

use error::ConvertResult;
#[cfg(feature = "sixel")]
use image::RgbImage;
use image::{DynamicImage, GrayImage, RgbaImage};
use processor::ImageProcessorOptionsCreate;

/// The protocol of display
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    HalfColor,
    FullColor,
    FullNoColor,
    Ascii,
    WezTerm,
    WezTermNoColor,
    Kitty,
    KittyNoColor,
    Iterm2,
    Iterm2NoColor,
    #[cfg(feature = "sixel")]
    SixelHalf,
    #[cfg(feature = "sixel")]
    SixelFull,
}

impl Default for DisplayMode {
    fn default() -> Self {
        Self::HalfColor
    }
}

impl DisplayMode {
    /// Check if the display mode is a full block mode
    ///
    /// Returns `true` for modes that use full blocks to display pixels, which includes:
    /// - `DisplayMode::FullColor`
    /// - `DisplayMode::FullNoColor`
    /// - `DisplayMode::WezTerm`
    /// - `DisplayMode::WezTermNoColor`
    /// - `DisplayMode::Kitty`
    /// - `DisplayMode::KittyNoColor`
    /// - `DisplayMode::Iterm2`
    /// - `DisplayMode::Iterm2NoColor`
    /// - `DisplayMode::SixelFull`
    ///
    /// Returns `false` for modes that use half blocks or ASCII characters:
    /// - `DisplayMode::HalfColor`
    /// - `DisplayMode::Ascii`
    /// - `DisplayMode::SixelHalf`
    pub fn is_full(&self) -> bool {
        #[cfg(feature = "sixel")]
        return !matches!(self, Self::HalfColor | Self::Ascii | Self::SixelHalf);
        #[cfg(not(feature = "sixel"))]
        return !matches!(self, Self::HalfColor | Self::Ascii);
    }

    /// Check if the display mode supports color output
    ///
    /// Returns `true` for modes that can display colors:
    /// - `DisplayMode::FullColor`
    /// - `DisplayMode::HalfColor`
    /// - `DisplayMode::WezTerm`
    /// - `DisplayMode::Kitty`
    /// - `DisplayMode::Iterm2`
    /// - `DisplayMode::SixelHalf`
    /// - `DisplayMode::SixelFull`
    ///
    /// Returns `false` for modes that only support grayscale/luminance output:
    /// - `DisplayMode::FullNoColor`
    /// - `DisplayMode::Ascii`
    /// - `DisplayMode::WezTermNoColor`
    /// - `DisplayMode::KittyNoColor`
    /// - `DisplayMode::Iterm2NoColor`
    pub fn is_color(&self) -> bool {
        #[cfg(feature = "sixel")]
        return matches!(
            self,
            Self::FullColor
                | Self::HalfColor
                | Self::WezTerm
                | Self::Kitty
                | Self::Iterm2
                | Self::SixelHalf
                | Self::SixelFull
        );
        #[cfg(not(feature = "sixel"))]
        matches!(
            self,
            Self::FullColor | Self::HalfColor | Self::WezTerm | Self::Kitty | Self::Iterm2
        )
    }

    /// Check if the display mode supports only luminance/grayscale output
    ///
    /// This is the inverse of `is_color()`. Returns `true` for modes that only display
    /// grayscale images and `false` for color-capable modes.
    pub fn is_luma(&self) -> bool {
        !self.is_color()
    }

    /// Check if the display mode uses standard terminal capabilities
    ///
    /// Returns `true` for basic terminal display modes that don't require special
    /// terminal features:
    /// - `DisplayMode::HalfColor`
    /// - `DisplayMode::FullColor`
    /// - `DisplayMode::Ascii`
    /// - `DisplayMode::FullNoColor`
    ///
    /// Returns `false` for modes that require special terminal protocols:
    /// - WezTerm-specific modes
    /// - Kitty-specific modes
    /// - iTerm2-specific modes
    /// - Sixel modes (when `sixel` feature is enabled)
    pub fn is_normal(&self) -> bool {
        matches!(
            self,
            Self::HalfColor | Self::FullColor | Self::Ascii | Self::FullNoColor
        )
    }

    /// Check if the display mode is WezTerm-specific
    ///
    /// Returns `true` for both color and non-color WezTerm modes:
    /// - `DisplayMode::WezTerm`
    /// - `DisplayMode::WezTermNoColor`
    pub fn is_wezterm(&self) -> bool {
        matches!(self, Self::WezTerm | Self::WezTermNoColor)
    }

    /// Check if the display mode is iTerm2-specific
    ///
    /// Returns `true` for both color and non-color iTerm2 modes:
    /// - `DisplayMode::Iterm2`
    /// - `DisplayMode::Iterm2NoColor`
    pub fn is_iterm2(&self) -> bool {
        matches!(self, Self::Iterm2 | Self::Iterm2NoColor)
    }

    /// Check if the display mode uses Sixel graphics protocol
    ///
    /// Available only when the `sixel` feature is enabled.
    ///
    /// Returns `true` for Sixel modes:
    /// - `DisplayMode::SixelHalf`
    /// - `DisplayMode::SixelFull`
    #[cfg(feature = "sixel")]
    pub fn is_sixel(&self) -> bool {
        matches!(self, Self::SixelHalf | Self::SixelFull)
    }

    /// Get the name of the display mode as a static string slice
    ///
    /// This method returns a human-readable name for each display mode variant,
    /// which can be useful for debugging, logging, or user interface purposes.
    ///
    /// # Returns
    /// A static string slice representing the name of the display mode:
    /// * "HalfColor" - For half-block color mode
    /// * "FullColor" - For full-block color mode
    /// * "FullNoColor" - For full-block grayscale mode
    /// * "Ascii" - For ASCII character-based rendering
    /// * "WezTerm" - For WezTerm terminal protocol (color)
    /// * "WezTermNoColor" - For WezTerm terminal protocol (grayscale)
    /// * "Kitty" - For Kitty terminal protocol (color)
    /// * "KittyNoColor" - For Kitty terminal protocol (grayscale)
    /// * "Iterm2" - For iTerm2 terminal protocol (color)
    /// * "Iterm2NoColor" - For iTerm2 terminal protocol (grayscale)
    /// * "SixelHalf" - For Sixel graphics protocol (half-block, when `sixel` feature is enabled)
    /// * "SixelFull" - For Sixel graphics protocol (full-block, when `sixel` feature is enabled)
    ///
    /// # Examples
    /// ```
    /// use image_to_console_core::DisplayMode;
    ///
    /// assert_eq!(DisplayMode::HalfColor.mode(), "HalfColor");
    /// assert_eq!(DisplayMode::Ascii.mode(), "Ascii");
    /// ```
    pub fn mode(&self) -> &'static str {
        match self {
            Self::HalfColor => "HalfColor",
            Self::FullColor => "FullColor",
            Self::FullNoColor => "FullNoColor",
            Self::Ascii => "Ascii",
            Self::WezTerm => "WezTerm",
            Self::WezTermNoColor => "WezTermNoColor",
            Self::Kitty => "Kitty",
            Self::KittyNoColor => "KittyNoColor",
            Self::Iterm2 => "Iterm2",
            Self::Iterm2NoColor => "Iterm2NoColor",
            #[cfg(feature = "sixel")]
            Self::SixelHalf => "SixelHalf",
            #[cfg(feature = "sixel")]
            Self::SixelFull => "SixelFull",
        }
    }

    /// Check if the given processed image type is compatible with this display mode
    ///
    /// This method verifies whether a ProcessedImage has been processed in a format
    /// that is suitable for display with the current DisplayMode. Different display
    /// modes require different image formats - some need color information, others
    /// work with grayscale, and some need both.
    ///
    /// # Arguments
    /// * `img_type` - A reference to the ProcessedImage to check for compatibility
    ///
    /// # Returns
    /// * `true` - If the image type is compatible with this display mode
    /// * `false` - If the image type is not compatible with this display mode
    ///
    /// # Compatibility Rules
    /// * `FullColor` mode requires images with both color and grayscale data (`ProcessedImage::Both`)
    /// * `SixelHalf` and `SixelFull` modes require RGB image data (`ProcessedImage::Color2`)
    /// * `HalfColor`, `Kitty`, `Iterm2`, and `WezTerm` modes require color image data (`ProcessedImage::Color`)
    /// * `Ascii`, `FullNoColor`, `KittyNoColor`, `Iterm2NoColor`, and `WezTermNoColor` modes require grayscale data (`ProcessedImage::NoColor`)
    ///
    /// # Examples
    /// ```
    /// use image_to_console_core::{DisplayMode, ProcessedImage};
    /// use image::DynamicImage;
    ///
    /// let img = DynamicImage::new_rgba8(10, 10);
    /// let color_image = ProcessedImage::new(DisplayMode::HalfColor, &img);
    /// let ascii_image = ProcessedImage::new(DisplayMode::Ascii, &img);
    ///
    /// assert!(DisplayMode::HalfColor.check_image_type(&color_image));
    /// assert!(!DisplayMode::HalfColor.check_image_type(&ascii_image));
    /// assert!(DisplayMode::Ascii.check_image_type(&ascii_image));
    /// ```
    pub fn check_image_type(&self, img_type: &ProcessedImage) -> bool {
        match self {
            Self::FullColor => img_type.is_both(),
            #[cfg(feature = "sixel")]
            Self::SixelHalf | Self::SixelFull => img_type.is_color2(),
            Self::HalfColor | Self::Kitty | Self::Iterm2 | Self::WezTerm => img_type.is_color(),
            Self::Ascii
            | Self::FullNoColor
            | Self::KittyNoColor
            | Self::Iterm2NoColor
            | Self::WezTermNoColor => img_type.is_no_color(),
        }
    }

    /// Get the expected image type name for this display mode
    ///
    /// This method returns a human-readable string indicating what type of
    /// processed image data is required for this display mode. This can be
    /// useful for debugging, validation, or user interface purposes.
    ///
    /// # Returns
    /// A static string slice representing the expected image type:
    /// * "Both" - For modes requiring both color and grayscale data (FullColor)
    /// * "Color2" - For modes requiring RGB data (SixelHalf, SixelFull)
    /// * "Color" - For modes requiring RGBA color data (HalfColor, Kitty, Iterm2, WezTerm)
    /// * "NoColor" - For modes requiring grayscale data (Ascii, FullNoColor, KittyNoColor, Iterm2NoColor, WezTermNoColor)
    ///
    /// # Examples
    /// ```
    /// use image_to_console_core::DisplayMode;
    ///
    /// assert_eq!(DisplayMode::HalfColor.expect_image_type(), "Color");
    /// assert_eq!(DisplayMode::Ascii.expect_image_type(), "NoColor");
    /// #[cfg(feature = "sixel")]
    /// assert_eq!(DisplayMode::SixelHalf.expect_image_type(), "Color2");
    /// ```
    pub fn expect_image_type(&self) -> &'static str {
        match self {
            Self::FullColor => "Both",
            #[cfg(feature = "sixel")]
            Self::SixelHalf | Self::SixelFull => "Color2",
            Self::HalfColor | Self::Kitty | Self::Iterm2 | Self::WezTerm => "Color",
            Self::Ascii
            | Self::FullNoColor
            | Self::KittyNoColor
            | Self::Iterm2NoColor
            | Self::WezTermNoColor => "NoColor",
        }
    }
}

/// Represents an image that has been processed for display purposes
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessedImage {
    /// A color image represented as RGBA pixels
    Color(RgbaImage),
    /// A color image represented as RGB pixels, available only when sixel feature is enabled
    #[cfg(feature = "sixel")]
    Color2(RgbImage),
    /// A grayscale/luminance image
    NoColor(GrayImage),
    /// Both color (RGBA) and grayscale (Gray) representations of the same image
    Both(RgbaImage, GrayImage),
}

impl ProcessedImage {
    /// Create a new ProcessedImage based on the display mode and source image
    ///
    /// This method converts the input DynamicImage into the appropriate format(s)
    /// required by the specified DisplayMode.
    ///
    /// # Arguments
    /// * `mode` - The DisplayMode that determines how the image should be processed
    /// * `img` - The source DynamicImage to process
    ///
    /// # Returns
    /// A ProcessedImage variant containing the appropriately formatted image data
    pub fn new(mode: DisplayMode, img: &DynamicImage) -> Self {
        match mode {
            DisplayMode::Ascii => Self::NoColor(img.to_luma8()),
            DisplayMode::Kitty => Self::Color(img.to_rgba8()),
            DisplayMode::Iterm2 => Self::Color(img.to_rgba8()),
            DisplayMode::WezTerm => Self::Color(img.to_rgba8()),
            DisplayMode::HalfColor => Self::Color(img.to_rgba8()),
            DisplayMode::FullNoColor => Self::NoColor(img.to_luma8()),
            DisplayMode::KittyNoColor => Self::NoColor(img.to_luma8()),
            DisplayMode::Iterm2NoColor => Self::NoColor(img.to_luma8()),
            DisplayMode::WezTermNoColor => Self::NoColor(img.to_luma8()),
            DisplayMode::FullColor => Self::Both(img.to_rgba8(), img.to_luma8()),
            #[cfg(feature = "sixel")]
            DisplayMode::SixelHalf => Self::Color2(img.to_rgb8()),
            #[cfg(feature = "sixel")]
            DisplayMode::SixelFull => Self::Color2(img.to_rgb8()),
        }
    }

    /// Get a reference to the RGBA image data if available
    ///
    /// # Returns
    /// * `Some(&RgbaImage)` - If the processed image contains RGBA data (Color or Both variants)
    /// * `None` - If the processed image only contains grayscale data (NoColor variant)
    pub fn rgba(&self) -> Option<&RgbaImage> {
        match self {
            Self::Color(img) => Some(img),
            Self::Both(img, _) => Some(img),
            _ => None,
        }
    }

    /// Get a reference to the RGB image data if available
    ///
    /// Available only when the sixel feature is enabled.
    ///
    /// # Returns
    /// * `Some(&RgbImage)` - If the processed image contains RGB data (Color2 variant)
    /// * `None` - For other variants
    #[cfg(feature = "sixel")]
    pub fn rgb(&self) -> Option<&RgbImage> {
        match self {
            Self::Color2(img) => Some(img),
            _ => None,
        }
    }

    /// Get a reference to the grayscale image data if available
    ///
    /// # Returns
    /// * `Some(&GrayImage)` - If the processed image contains grayscale data (NoColor or Both variants)
    /// * `None` - Should not occur in normal usage
    pub fn luma(&self) -> Option<&GrayImage> {
        match self {
            Self::NoColor(img) => Some(img),
            Self::Both(_, img) => Some(img),
            _ => None,
        }
    }

    /// Get references to both RGBA and grayscale image data if available
    ///
    /// # Returns
    /// * `Some((&RgbaImage, &GrayImage))` - If the processed image contains both formats (Both variant)
    /// * `None` - For other variants
    pub fn both(&self) -> Option<(&RgbaImage, &GrayImage)> {
        match self {
            Self::Both(rgba, luma) => Some((rgba, luma)),
            _ => None,
        }
    }

    /// Check if the processed image contains color data
    ///
    /// # Returns
    /// * `true` - If the processed image only color data (Color variants)
    /// * `false` - If the processed image contains grayscale data (NoColor variant)
    pub fn is_color(&self) -> bool {
        matches!(self, Self::Color(_))
    }

    /// Check if the processed image contains no color data (grayscale only)
    ///
    /// # Returns
    /// * `true` - If the processed image contains only grayscale data (NoColor variant)
    /// * `false` - If the processed image contains color data (Color, Color2, or Both variants)
    pub fn is_no_color(&self) -> bool {
        matches!(self, Self::NoColor(_))
    }

    /// Check if the processed image contains both color and grayscale data
    ///
    /// # Returns
    /// * `true` - If the processed image contains both color and grayscale data (Both variant)
    /// * `false` - If the processed image contains only color or only grayscale data
    pub fn is_both(&self) -> bool {
        matches!(self, Self::Both(_, _))
    }

    /// Check if the processed image contains RGB color data (available only when sixel feature is enabled)
    ///
    /// # Returns
    /// * `true` - If the processed image contains RGB color data (Color2 variant)
    /// * `false` - If the processed image contains other data types
    #[cfg(feature = "sixel")]
    pub fn is_color2(&self) -> bool {
        matches!(self, Self::Color2(_))
    }

    /// Get the name of the processed image variant
    ///
    /// # Returns
    /// A static string slice representing the variant name:
    /// * "Color" - For RGBA image data
    /// * "Color2" - For RGB image data (when sixel feature is enabled)
    /// * "NoColor" - For grayscale image data
    /// * "Both" - For images containing both RGBA and grayscale data
    pub fn mode(&self) -> &'static str {
        match self {
            Self::Color(_) => "Color",
            #[cfg(feature = "sixel")]
            Self::Color2(_) => "Color2",
            Self::NoColor(_) => "NoColor",
            Self::Both(_, _) => "Both",
        }
    }
}

/// AutoResize Option struct
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutoResizeOption {
    /// Resize with terminal width
    pub width: bool,
    /// Resize with terminal height
    pub height: bool,
}

impl Default for AutoResizeOption {
    fn default() -> Self {
        Self {
            width: true,
            height: true,
        }
    }
}

impl AutoResizeOption {
    /// Create a new AutoResizeOption with the specified width and height settings
    ///
    /// # Arguments
    /// * `width` - Whether to resize based on terminal width
    /// * `height` - Whether to resize based on terminal height
    ///
    /// # Returns
    /// A new AutoResizeOption instance with the specified settings
    pub fn new(width: bool, height: bool) -> Self {
        Self { width, height }
    }

    /// Create an AutoResizeOption that resizes only based on terminal width
    ///
    /// This is useful when you want to maintain the original aspect ratio but
    /// adjust the image width to fit the terminal.
    ///
    /// # Returns
    /// An AutoResizeOption with width=true and height=false
    pub fn only_width() -> Self {
        Self {
            width: true,
            height: false,
        }
    }

    /// Create an AutoResizeOption that resizes only based on terminal height
    ///
    /// This is useful when you want to maintain the original aspect ratio but
    /// adjust the image height to fit the terminal.
    ///
    /// # Returns
    /// An AutoResizeOption with width=false and height=true
    pub fn only_height() -> Self {
        Self {
            width: false,
            height: true,
        }
    }

    /// Create an AutoResizeOption that disables automatic resizing
    ///
    /// This effectively disables auto-resizing functionality, meaning the image
    /// will retain its original dimensions regardless of terminal size.
    ///
    /// # Returns
    /// An AutoResizeOption with both width and height set to false
    pub fn none() -> Self {
        Self {
            width: false,
            height: false,
        }
    }
}

/// CustomResize Option struct
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CustomResizeOption {
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl CustomResizeOption {
    /// Create a new CustomResizeOption with both width and height specified
    ///
    /// # Arguments
    /// * `width` - The width to resize to
    /// * `height` - The height to resize to
    ///
    /// # Returns
    /// A new CustomResizeOption instance with both width and height set
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: Some(width),
            height: Some(height),
        }
    }

    /// Create a CustomResizeOption with only width specified
    ///
    /// This is useful when you want to resize the image to a specific width
    /// while maintaining the original aspect ratio.
    ///
    /// # Arguments
    /// * `width` - The width to resize to
    ///
    /// # Returns
    /// A new CustomResizeOption instance with only width set
    pub fn with_width(width: u32) -> Self {
        Self {
            width: Some(width),
            height: None,
        }
    }

    /// Create a CustomResizeOption with only height specified
    ///
    /// This is useful when you want to resize the image to a specific height
    /// while maintaining the original aspect ratio.
    ///
    /// # Arguments
    /// * `height` - The height to resize to
    ///
    /// # Returns
    /// A new CustomResizeOption instance with only height set
    pub fn with_height(height: u32) -> Self {
        Self {
            width: None,
            height: Some(height),
        }
    }
}

/// Resize Mode enum
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResizeMode {
    /// Resize with terminal size
    Auto(AutoResizeOption),
    /// Resize with given size
    Custom(CustomResizeOption),
    /// No resize
    None,
}

impl Default for ResizeMode {
    /// Returns to the default resize mode [Auto and the width and height are automatically adjusted
    fn default() -> Self {
        Self::Auto(AutoResizeOption::default())
    }
}

pub fn print(
    image: &image::DynamicImage,
    config: &crate::processor::ImageProcessorOptions,
) -> ConvertResult<()> {
    println!("{}", config.create_processor(image).process()?.display());
    Ok(())
}

#[macro_export]
/// A macro to display a single image to the terminal using default options or custom options.
///
/// This macro provides a convenient way to display images without manually setting up
/// the image processor and display protocol. It automatically detects the best terminal
/// protocol to use and processes the image accordingly.
///
/// # Arguments
///
/// * `$image` - An image of type `image::DynamicImage` to be displayed
/// * `$option` (optional) - Custom `ImageProcessorOptions` to control how the image is processed
///
/// # Examples
///
/// ```rust,ignore
/// // Display an image with default options
/// show_image!(my_image);
///
/// // Display an image with custom options
/// let options = ImageProcessorOptions::default()
///     .option_display_mode(DisplayMode::Ascii);
/// show_image!(my_image, options);
/// ```
#[cfg(not(feature = "auto_select"))]
macro_rules! show_image {
    ($image:expr) => {
        fn _show_image<T>(image: T)
        where
            $crate::processor::ImageProcessorOptions:
                $crate::processor::ImageProcessorOptionsCreate<T>,
        {
            use $crate::processor::ImageProcessorOptionsCreate;
            let display_mode = $crate::protocol::Protocol::default().builder().build();
            let result = $crate::processor::ImageProcessorOptions::default()
                .option_display_mode(display_mode)
                .create_processor(image)
                .process()
                .expect("Process image failed");
            println!("{}", result.display());
        }
        _show_image($image);
    };
    ($image: expr, $option: expr) => {
        fn _show_image<T>(image: T, option: $crate::processor::ImageProcessorOptions)
        where
            $crate::processor::ImageProcessorOptions:
                $crate::processor::ImageProcessorOptionsCreate<T>,
        {
            use $crate::processor::ImageProcessorOptionsCreate;
            let result = option
                .create_processor(image)
                .process()
                .expect("Process image failed");
            println!("{}", result.display());
        }
        _show_image($image, $option);
    };
}

#[macro_export]
#[cfg(feature = "auto_select")]
/// A macro to display a single image to the terminal using default options or custom options.
///
/// This macro provides a convenient way to display images without manually setting up
/// the image processor and display protocol. It automatically detects the best terminal
/// protocol to use and processes the image accordingly.
///
/// # Arguments
///
/// * `$image` - An image of type `image::DynamicImage` to be displayed
/// * `$option` (optional) - Custom `ImageProcessorOptions` to control how the image is processed
///
/// # Examples
///
/// ```rust,ignore
/// // Display an image with default options
/// show_image!(my_image);
///
/// // Display an image with custom options
/// let options = ImageProcessorOptions::default()
///     .option_display_mode(DisplayMode::Ascii);
/// show_image!(my_image, options);
/// ```
macro_rules! show_image {
    ($image:expr) => {
        fn _show_image<T>(image: T)
        where
            $crate::processor::ImageProcessorOptions:
                $crate::processor::ImageProcessorOptionsCreate<T>,
        {
            use $crate::processor::ImageProcessorOptionsCreate;
            let display_mode = $crate::protocol::Protocol::Auto.builder().build();
            let result = $crate::processor::ImageProcessorOptions::default()
                .option_display_mode(display_mode)
                .create_processor(image)
                .process()
                .expect("Process image failed");
            println!("{}", result.display());
        }
        _show_image($image);
    };
    ($image: expr, $option: expr) => {
        fn _show_image<T>(image: T, option: $crate::processor::ImageProcessorOptions)
        where
            $crate::processor::ImageProcessorOptions:
                $crate::processor::ImageProcessorOptionsCreate<T>,
        {
            use $crate::processor::ImageProcessorOptionsCreate;
            let result = option
                .create_processor(image)
                .process()
                .expect("Process image failed");
            println!("{}", result.display());
        }
        _show_image($image, $option);
    };
}

#[macro_export]
#[cfg(not(feature = "auto_select"))]
/// A macro to display multiple images to the terminal.
///
/// This macro allows displaying multiple images either with default options or with
/// shared custom options. It's useful when you want to display a series of images
/// with the same processing settings.
///
/// # Arguments
///
/// * `@vec $images` - A vector of images of type `Vec<image::DynamicImage>` to be displayed
/// * `$image` - One or more images of type `image::DynamicImage` to be displayed
/// * `@with_options $option` (optional) - Custom `ImageProcessorOptions` to control
///   how all images are processed
///
/// # Examples
///
/// ```rust,ignore
/// // Display multiple images with default options
/// show_images!(image1, image2, image3);
///
/// // Display multiple images with custom options
/// let options = ImageProcessorOptions::default()
///     .option_resize_mode(ResizeMode::Custom(CustomResizeOption::new(80, 40)))
///     .get_options();
/// show_images!(image1, image2, image3, @with_options options);
///
/// // Display images from a vector with default options
/// let image_vec = vec![image1, image2, image3];
/// show_images!(@vec image_vec);
///
/// // Display images from a vector with custom options
/// let options = ImageProcessorOptions::default()
///     .option_display_mode(DisplayMode::Ascii)
///     .get_options();
/// show_images!(@vec image_vec, @with_options options);
/// ```
macro_rules! show_images {
    (@vec $images:expr) => {
        use $crate::processor::ImageProcessorOptionsCreate;
        let display_mode = $crate::protocol::Protocol::default().builder().build();
        let option = $crate::processor::ImageProcessorOptions::default()
            .option_display_mode(display_mode)
            .get_options();
        fn _show_image<T>(image: T, option: $crate::processor::ImageProcessorOptions)
        where
            $crate::processor::ImageProcessorOptions:
                $crate::processor::ImageProcessorOptionsCreate<T>,
        {
            let result = option
                .create_processor(image)
                .process()
                .expect("Process image failed");
            println!("{}", result.display());
        }
        let images: Vec<$crate::image::DynamicImage> = $images;
        for image in images {
            _show_image(image, option);
        }
    };
    (@vec $images:expr, @with_options $option: expr) => {
        fn _show_image<T>(image: T, option: $crate::processor::ImageProcessorOptions)
        where
            $crate::processor::ImageProcessorOptions:
                $crate::processor::ImageProcessorOptionsCreate<T>,
        {
            use $crate::processor::ImageProcessorOptionsCreate;
            let result = option
                .create_processor(image)
                .process()
                .expect("Process image failed");
            println!("{}", result.display());
        }
        let option: $crate::processor::ImageProcessorOptions = $option;
        let images: Vec<$crate::image::DynamicImage> = $images;
        for image in images {
            _show_image(image, option);
        }
    };
    ($($image:expr),+, @with_options $option: expr) => {
        fn _show_image<T>(image: T, option: $crate::processor::ImageProcessorOptions)
        where
            $crate::processor::ImageProcessorOptions:
                $crate::processor::ImageProcessorOptionsCreate<T>,
        {
            use $crate::processor::ImageProcessorOptionsCreate;
            let result = option
                .create_processor(image)
                .process()
                .expect("Process image failed");
            println!("{}", result.display());
        }
        let option: $crate::processor::ImageProcessorOptions = $option;
        $(
            _show_image($image, option);
        )+
    };
    ($($image:expr),+) => {
        use $crate::processor::ImageProcessorOptionsCreate;
        let display_mode = $crate::protocol::Protocol::default().builder().build();
        let option = $crate::processor::ImageProcessorOptions::default()
            .option_display_mode(display_mode)
            .get_options();
        fn _show_image<T>(image: T, option: $crate::processor::ImageProcessorOptions)
        where
            $crate::processor::ImageProcessorOptions:
                $crate::processor::ImageProcessorOptionsCreate<T>,
        {
            println!("{}", option.create_processor(image).process().expect("Process image failed").display());
        }
        $(
            _show_image($image, option);
        )+
    };
}

#[macro_export]
#[cfg(feature = "auto_select")]
/// A macro to display multiple images to the terminal.
///
/// This macro allows displaying multiple images either with default options or with
/// shared custom options. It's useful when you want to display a series of images
/// with the same processing settings.
///
/// # Arguments
///
/// * `@vec $images` - A vector of images of type `Vec<image::DynamicImage>` to be displayed
/// * `$image` - One or more images of type `image::DynamicImage` to be displayed
/// * `@with_options $option` (optional) - Custom `ImageProcessorOptions` to control
///   how all images are processed
///
/// # Examples
///
/// ```rust,ignore
/// // Display multiple images with default options
/// show_images!(image1, image2, image3);
///
/// // Display multiple images with custom options
/// let options = ImageProcessorOptions::default()
///     .option_resize_mode(ResizeMode::Custom(CustomResizeOption::new(80, 40)))
///     .get_options();
/// show_images!(image1, image2, image3, @with_options options);
///
/// // Display images from a vector with default options
/// let image_vec = vec![image1, image2, image3];
/// show_images!(@vec image_vec);
///
/// // Display images from a vector with custom options
/// let options = ImageProcessorOptions::default()
///     .option_display_mode(DisplayMode::Ascii)
///     .get_options();
/// show_images!(@vec image_vec, @with_options options);
/// ```
macro_rules! show_images {
    (@vec $images:expr) => {
        use $crate::processor::ImageProcessorOptionsCreate;
        let display_mode = $crate::protocol::Protocol::Auto.builder().build();
        let option = $crate::processor::ImageProcessorOptions::default()
            .option_display_mode(display_mode)
            .get_options();
        fn _show_image<T>(image: T, option: $crate::processor::ImageProcessorOptions)
        where
            $crate::processor::ImageProcessorOptions:
                $crate::processor::ImageProcessorOptionsCreate<T>,
        {
            let result = option
                .create_processor(image)
                .process()
                .expect("Process image failed");
            println!("{}", result.display());
        }
        let images: Vec<$crate::image::DynamicImage> = $images;
        for image in images {
            _show_image(image, option);
        }
    };
    (@vec $images:expr, @with_options $option: expr) => {
        fn _show_image<T>(image: T, option: $crate::processor::ImageProcessorOptions)
        where
            $crate::processor::ImageProcessorOptions:
                $crate::processor::ImageProcessorOptionsCreate<T>,
        {
            use $crate::processor::ImageProcessorOptionsCreate;
            let result = option
                .create_processor(image)
                .process()
                .expect("Process image failed");
            println!("{}", result.display());
        }
        let option: $crate::processor::ImageProcessorOptions = $option;
        let images: Vec<$crate::image::DynamicImage> = $images;
        for image in images {
            _show_image(image, option);
        }
    };
    ($($image:expr),+, @with_options $option: expr) => {
        fn _show_image<T>(image: T, option: $crate::processor::ImageProcessorOptions)
        where
            $crate::processor::ImageProcessorOptions:
                $crate::processor::ImageProcessorOptionsCreate<T>,
        {
            use $crate::processor::ImageProcessorOptionsCreate;
            let result = option
                .create_processor(image)
                .process()
                .expect("Process image failed");
            println!("{}", result.display());
        }
        let option: $crate::processor::ImageProcessorOptions = $option;
        $(
            _show_image($image, option);
        )+
    };
    ($($image:expr),+) => {
        use $crate::processor::ImageProcessorOptionsCreate;
        let display_mode = $crate::protocol::Protocol::Auto.builder().build();
        let option = $crate::processor::ImageProcessorOptions::default()
            .option_display_mode(display_mode)
            .get_options();
        fn _show_image<T>(image: T, option: $crate::processor::ImageProcessorOptions)
        where
            $crate::processor::ImageProcessorOptions:
                $crate::processor::ImageProcessorOptionsCreate<T>,
        {
            println!("{}", option.create_processor(image).process().expect("Process image failed").display());
        }
        $(
            _show_image($image, option);
        )+
    };
}

#[doc(hidden)]
#[cfg(not(feature = "auto_select"))]
#[macro_export]
macro_rules! __vec_process_images {
    ($images: expr, $mode:ident, $var:ident, $(ty: $ty:ident,)?$(collect: $collect:ident$(,)?)?$(result: $result:ident$(,)?)?$(block: $block:block$(,)?)?$(end: $end:tt$(,)?)?) => {
        {
            use $crate::processor::ImageProcessorOptionsCreate;
            let display_mode = $crate::protocol::Protocol::default().builder().build();
            let option = $crate::processor::ImageProcessorOptions::default()
                    .option_display_mode(display_mode)
                    .get_options();
            let images: Vec<$crate::image::DynamicImage> = $images;
            if images.len() > 10 {
                use $crate::rayon::prelude::*;
                images
                    .into_par_iter()
                    .$mode(|mut image| {
                        let $var = option.create_processor(image).process();
                        $($block)?$($end)?
                        $($result)?
                    })
                    $(.$collect::<Vec<_>>())?
            } else {
                images
                    .iter()
                    .$mode(|image| {
                        let $var = option.create_processor(image).process();
                        $($block)?$($end)?
                        $($result)?
                    })
                    $(.$collect::<Vec<_>>())?
            }
        }
    };
    ($images: expr, $mode:ident, $var:ident, options: $options: expr, $(ty: $ty:ident,)?$(collect: $collect:ident$(,)?)?$(result: $result:ident$(,)?)?$(block: $block:block$(,)?)?$(end: $end:tt$(,)?)?) => {
        {
            use $crate::processor::ImageProcessorOptionsCreate;

            let options: $crate::processor::ImageProcessorOptions = $options;
            let images: Vec<$crate::image::DynamicImage> = $images;

            if images.len() > 10 {
                use $crate::rayon::prelude::*;
                images
                    .into_par_iter()
                    .$mode(|mut image| {
                        let $var = options.create_processor(image).process();
                        $($block)?$($end)?
                        $($result)?
                    })$($end)?
                    $(.$collect::<Vec<_>>())?
            } else {
                images
                    .iter()
                    .$mode(|image| {
                        let $var = options.create_processor(image).process();
                        $($block)?$($end)?
                        $($result)?
                    })$($end)?
                    $(.$collect::<Vec<_>>())?
            }
        }
    };
}

#[doc(hidden)]
#[cfg(feature = "auto_select")]
#[macro_export]
macro_rules! __vec_process_images {
    ($images: expr, $mode:ident, $var:ident, $(ty: $ty:ident,)?$(collect: $collect:ident$(,)?)?$(result: $result:ident$(,)?)?$(block: $block:block$(,)?)?$(end: $end:tt$(,)?)?) => {
        {
            use $crate::processor::ImageProcessorOptionsCreate;
            let display_mode = $crate::protocol::Protocol::Auto.builder().build();
            let option = $crate::processor::ImageProcessorOptions::default()
                    .option_display_mode(display_mode)
                    .get_options();
            let images: Vec<$crate::image::DynamicImage> = $images;
            if images.len() > 10 {
                use $crate::rayon::prelude::*;
                images
                    .into_par_iter()
                    .$mode(|mut image| {
                        let $var = option.create_processor(image).process();
                        $($block)?$($end)?
                        $($result)?
                    })
                    $(.$collect::<Vec<_>>())?
            } else {
                images
                    .iter()
                    .$mode(|image| {
                        let $var = option.create_processor(image).process();
                        $($block)?$($end)?
                        $($result)?
                    })
                    $(.$collect::<Vec<_>>())?
            }
        }
    };
    ($images: expr, $mode:ident, $var:ident, options: $options: expr, $(ty: $ty:ident,)?$(collect: $collect:ident$(,)?)?$(result: $result:ident$(,)?)?$(block: $block:block$(,)?)?$(end: $end:tt$(,)?)?) => {
        {
            use $crate::processor::ImageProcessorOptionsCreate;

            let options: $crate::processor::ImageProcessorOptions = $options;
            let images: Vec<$crate::image::DynamicImage> = $images;

            if images.len() > 10 {
                use $crate::rayon::prelude::*;
                images
                    .into_par_iter()
                    .$mode(|mut image| {
                        let $var = options.create_processor(image).process();
                        $($block)?$($end)?
                        $($result)?
                    })$($end)?
                    $(.$collect::<Vec<_>>())?
            } else {
                images
                    .iter()
                    .$mode(|image| {
                        let $var = options.create_processor(image).process();
                        $($block)?$($end)?
                        $($result)?
                    })$($end)?
                    $(.$collect::<Vec<_>>())?
            }
        }
    };
}

#[macro_export]
#[cfg(not(feature = "auto_select"))]
/// A macro to process one or more images and return the processed results.
///
/// This macro provides flexible ways to process images with various options and
/// automatically handles parallel processing for better performance when dealing
/// with multiple images. It supports both single image processing and batch
/// processing with optional custom options.
///
/// For batch processing of more than 10 images, this macro automatically uses
/// parallel processing via the rayon crate to improve performance. For smaller
/// batches or single images, sequential processing is used.
///
/// # Arguments
///
/// * `()` - Returns an empty vector when called with no arguments
/// * `$image:expr` - Process a single image with default options
/// * `$image:expr, @with_options $options:expr` - Process a single image with custom options
/// * `$($image:expr),+` - Process multiple images with default options
/// * `$($image:expr),+, @with_options $options:expr` - Process multiple images with custom options
/// * `@vec $images:expr` - Process a vector of images with default options
/// * `@vec $images:expr, @with_options $options:expr` - Process a vector of images with custom options
/// * `@vec $images:expr, @with_options $options:expr, @var $var:ident, @map $block:block` - Process a vector of images with custom options and map operation
/// * `@vec $images:expr, @with_options $options:expr, @var $var:ident, @for_each $block:block` - Process a vector of images with custom options and for_each operation
/// * `@vec $images:expr, @var $var:ident, @map $block:block` - Process a vector of images with default options and map operation
/// * `@vec $images:expr, @var $var:ident, @for_each $block:block` - Process a vector of images with default options and for_each operation
///
/// # Examples
///
/// ```rust,ignore
/// // Process a single image with default options
/// let result = process_images!(my_image);
///
/// // Process a single image with custom options
/// let options = ImageProcessorOptions::default()
///     .option_display_mode(DisplayMode::Ascii)
///     .get_options();
/// let result = process_images!(my_image, @with_options options);
///
/// // Process multiple images with default options
/// let results = process_images!(image1, image2, image3);
/// // or
/// let results = process_images![image1, image2, image3];
///
/// // Process multiple images with custom options
/// let options = ImageProcessorOptions::default()
///     .option_resize_mode(ResizeMode::Custom(CustomResizeOption::new(80, 40)))
///     .get_options();
/// let results = process_images!(image1, image2, image3, @with_options options);
///
/// // Process a vector of images
/// let image_vec = vec![image1, image2, image3];
/// let results = process_images!(@vec image_vec);
///
/// // Process a vector of images with custom options
/// let results = process_images!(@vec image_vec, @with_options options);
///
/// // Process a vector of images with custom options and map operation
/// let results = process_images!(@vec image_vec, @with_options options, @var img, @map {
///     let img = img.unwrap();
///     println!("Processed image size: {}x{}", img.width, img.height);
///     img
/// });
///
/// // Process a vector of images with default options and for_each operation
/// process_images!(@vec image_vec, @var img, @for_each {
///     let img = img.unwrap();
///     println!("Processed image size: {}x{}", img.width, img.height);
/// });
///
/// // Return an empty vector
/// let empty_results: Vec<ImageProcessorResult> = process_images!();
/// ```
///
/// # Returns
///
/// * For single image processing: `ImageProcessorResult`
/// * For multiple image processing: `Vec<ImageProcessorResult>`
/// * For empty invocation: `Vec<ImageProcessorResult>` (empty vector)
/// * For map operations: `Vec<T>` where T is the return type of the map block
macro_rules! process_images {
    () => {
        Vec::<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>::new()
    };
    ($(@with_options $options:expr,)?$([]=>$(@$mut:tt)?$name:ident),*$(,)?) => {
        $(let $($mut )?$name = $crate::process_images!();)*
    };
    (@with_options $options:expr,$([$($image:expr),*]=>$(@$mut:tt)?$name:ident),*$(,)?) => {
        use $crate::processor::ImageProcessorOptionsCreate;
        let options: $crate::processor::ImageProcessorOptions = $options;
        $(let $($mut )?$name = $crate::process_images!(@vec vec![$($image),*], @with_options options);)*
    };
    ($([$($image:expr),*]=>$(@$mut:tt)?$name:ident),*$(,)?) => {
        let display_mode = $crate::protocol::Protocol::default().builder().build();
        let options = $crate::processor::ImageProcessorOptions::default()
                .option_display_mode(display_mode)
                .get_options();
        $(let $($mut )?$name = $crate::process_images!(@vec vec![$($image),*], @with_options options);)*
    };
    ($($image:expr=>$(@$mut:tt)?$name:ident$(@$end:tt)?),*$(,)?) => {
        use $crate::processor::ImageProcessorOptionsCreate;
        let display_mode = $crate::protocol::Protocol::default().builder().build();
        let options = $crate::processor::ImageProcessorOptions::default()
                .option_display_mode(display_mode)
                .get_options();
        $(let $($mut )?$name = options.create_processor($image).process()$($end)?;)*
    };
    (@with_options $options:expr,$($image:expr=>$(@$mut:tt)?$name:ident$(@$end:tt)?),*$(,)?) => {
        use $crate::processor::ImageProcessorOptionsCreate;
        let options: $crate::processor::ImageProcessorOptions = $options;
        $(let $($mut )?$name = options.create_processor($image).process()$($end)?;)*
    };
    (@vec $images:expr) => {{
        fn _process_images(images: Vec<$crate::image::DynamicImage>) -> Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>> {
            use $crate::processor::ImageProcessorOptionsCreate;
            let display_mode = $crate::protocol::Protocol::default().builder().build();
            let option = $crate::processor::ImageProcessorOptions::default()
                    .option_display_mode(display_mode)
                    .get_options();

            if images.len() > 10 {
                use $crate::rayon::prelude::*;
                images
                    .into_par_iter()
                    .map(|image| option.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            } else {
                images
                    .into_iter()
                    .map(|image| option.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            }
        }
        _process_images($images)}
    };
    (@vec $images:expr$(,@with_options $options:expr)?,@var $var:ident,@map $block:block) => {
        $crate::__vec_process_images!($images, map, $var $(,options: $options)?, ty: Vec, collect: collect, block: $block)
    };
    (@vec $images:expr$(,@with_options $options:expr)?,@map $block:block) => {
        $crate::__vec_process_images!($images, map, result $(,options: $options)?, ty: Vec, collect: collect, result: result, block: $block)
    };
    (@vec $images:expr$(,@with_options $options:expr)?,@var $var:ident,@for_each $block:block) => {
        $crate::__vec_process_images!($images, for_each, $var $(,options: $options)?, block: $block, end: ;)
    };
    (@vec $images:expr$(,@with_options $options:expr)?,@for_each $block:block) => {
        $crate::__vec_process_images!($images, for_each, result $(,options: $options)?, block: $block, end: ;)
    };
    (@vec $images:expr,@with_options $options:expr) => {{
        fn _process_images(images: Vec<$crate::image::DynamicImage>, options: $crate::processor::ImageProcessorOptions) -> Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>> {
            use $crate::processor::ImageProcessorOptionsCreate;
            if images.len() > 10 {
                use $crate::rayon::prelude::*;
                images
                    .into_par_iter()
                    .map(|image| options.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            } else {
                images
                    .into_iter()
                    .map(|image| options.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            }
        }
        _process_images($images, $options)}
    };
    ($image:expr) => {{
        let display_mode = $crate::protocol::Protocol::default().builder().build();
        let option = $crate::processor::ImageProcessorOptions::default()
                .option_display_mode(display_mode)
                .get_options();
        fn _process_image(image: $crate::image::DynamicImage, option: $crate::processor::ImageProcessorOptions) -> $crate::error::ConvertResult<$crate::processor::ImageProcessorResult> {
            $crate::processor::ImageProcessor::new(image, option).process()
        }
        _process_image($image, option)}
    };
    ($image:expr,@with_options $options:expr) => {{
        fn _process_image(image: $crate::image::DynamicImage, option: $crate::processor::ImageProcessorOptions) -> $crate::error::ConvertResult<$crate::processor::ImageProcessorResult> {
            $crate::processor::ImageProcessor::new(image, option).process()
        }
        _process_image($image, $options)}
    };
    ($($image:expr),+,@with_options $options: expr) => {{
        fn _process_images(images: Vec<$crate::image::DynamicImage>, options: $crate::processor::ImageProcessorOptions) -> Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>> {
            use $crate::processor::ImageProcessorOptionsCreate;
            if images.len() > 10 {
                use $crate::rayon::prelude::*;
                images
                    .into_par_iter()
                    .map(|image| options.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            } else {
                images
                    .into_iter()
                    .map(|image| options.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            }
        }
        let options = $options;
        let images = vec![$($image),+];
        _process_images(images, options)
    }};
    ($($image:expr),+$(,)?) => {{
        fn _process_images(images: Vec<$crate::image::DynamicImage>) -> Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>> {
            use $crate::processor::ImageProcessorOptionsCreate;
            let display_mode = $crate::protocol::Protocol::default().builder().build();
            let option = $crate::processor::ImageProcessorOptions::default()
                    .option_display_mode(display_mode)
                    .get_options();

            if images.len() > 10 {
                use $crate::rayon::prelude::*;
                images
                    .into_par_iter()
                    .map(|image| option.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            } else {
                images
                    .into_iter()
                    .map(|image| option.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            }
        }
        let images = vec![$(
            $image
        ),+];
        _process_images(images)
        }
    }
}

#[macro_export]
#[cfg(feature = "auto_select")]
/// A macro to process one or more images and return the processed results.
///
/// This macro provides flexible ways to process images with various options and
/// automatically handles parallel processing for better performance when dealing
/// with multiple images. It supports both single image processing and batch
/// processing with optional custom options.
///
/// For batch processing of more than 10 images, this macro automatically uses
/// parallel processing via the rayon crate to improve performance. For smaller
/// batches or single images, sequential processing is used.
///
/// # Arguments
///
/// * `()` - Returns an empty vector when called with no arguments
/// * `$image:expr` - Process a single image with default options
/// * `$image:expr, @with_options $options:expr` - Process a single image with custom options
/// * `$($image:expr),+` - Process multiple images with default options
/// * `$($image:expr),+, @with_options $options:expr` - Process multiple images with custom options
/// * `@vec $images:expr` - Process a vector of images with default options
/// * `@vec $images:expr, @with_options $options:expr` - Process a vector of images with custom options
/// * `@vec $images:expr, @with_options $options:expr, @var $var:ident, @map $block:block` - Process a vector of images with custom options and map operation
/// * `@vec $images:expr, @with_options $options:expr, @var $var:ident, @for_each $block:block` - Process a vector of images with custom options and for_each operation
/// * `@vec $images:expr, @var $var:ident, @map $block:block` - Process a vector of images with default options and map operation
/// * `@vec $images:expr, @var $var:ident, @for_each $block:block` - Process a vector of images with default options and for_each operation
///
/// # Examples
///
/// ```rust,ignore
/// // Process a single image with default options
/// let result = process_images!(my_image);
///
/// // Process a single image with custom options
/// let options = ImageProcessorOptions::default()
///     .option_display_mode(DisplayMode::Ascii)
///     .get_options();
/// let result = process_images!(my_image, @with_options options);
///
/// // Process multiple images with default options
/// let results = process_images!(image1, image2, image3);
/// // or
/// let results = process_images![image1, image2, image3];
///
/// // Process multiple images with custom options
/// let options = ImageProcessorOptions::default()
///     .option_resize_mode(ResizeMode::Custom(CustomResizeOption::new(80, 40)))
///     .get_options();
/// let results = process_images!(image1, image2, image3, @with_options options);
///
/// // Process a vector of images
/// let image_vec = vec![image1, image2, image3];
/// let results = process_images!(@vec image_vec);
///
/// // Process a vector of images with custom options
/// let results = process_images!(@vec image_vec, @with_options options);
///
/// // Process a vector of images with custom options and map operation
/// let results = process_images!(@vec image_vec, @with_options options, @var img, @map {
///     let img = img.unwrap();
///     println!("Processed image size: {}x{}", img.width, img.height);
///     img
/// });
///
/// // Process a vector of images with default options and for_each operation
/// process_images!(@vec image_vec, @var img, @for_each {
///     let img = img.unwrap();
///     println!("Processed image size: {}x{}", img.width, img.height);
/// });
///
/// // Return an empty vector
/// let empty_results: Vec<ImageProcessorResult> = process_images!();
/// ```
///
/// # Returns
///
/// * For single image processing: `ImageProcessorResult`
/// * For multiple image processing: `Vec<ImageProcessorResult>`
/// * For empty invocation: `Vec<ImageProcessorResult>` (empty vector)
/// * For map operations: `Vec<T>` where T is the return type of the map block
macro_rules! process_images {
    () => {
        Vec::<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>::new()
    };
    ($(@with_options $options:expr,)?$([]=>$(@$mut:tt)?$name:ident),*$(,)?) => {
        $(let $($mut )?$name = $crate::process_images!();)*
    };
    (@with_options $options:expr,$([$($image:expr),*]=>$(@$mut:tt)?$name:ident),*$(,)?) => {
        use $crate::processor::ImageProcessorOptionsCreate;
        let options: $crate::processor::ImageProcessorOptions = $options;
        $(let $($mut )?$name = $crate::process_images!(@vec vec![$($image),*], @with_options options);)*
    };
    ($([$($image:expr),*]=>$(@$mut:tt)?$name:ident),*$(,)?) => {
        let display_mode = $crate::protocol::Protocol::Auto.builder().build();
        let options = $crate::processor::ImageProcessorOptions::default()
                .option_display_mode(display_mode)
                .get_options();
        $(let $($mut )?$name = $crate::process_images!(@vec vec![$($image),*], @with_options options);)*
    };
    ($($image:expr=>$(@$mut:tt)?$name:ident$(@$end:tt)?),*$(,)?) => {
        use $crate::processor::ImageProcessorOptionsCreate;
        let display_mode = $crate::protocol::Protocol::Auto.builder().build();
        let options = $crate::processor::ImageProcessorOptions::default()
                .option_display_mode(display_mode)
                .get_options();
        $(let $($mut )?$name = options.create_processor($image).process()$($end)?;)*
    };
    (@with_options $options:expr,$($image:expr=>$(@$mut:tt)?$name:ident$(@$end:tt)?),*$(,)?) => {
        use $crate::processor::ImageProcessorOptionsCreate;
        let options: $crate::processor::ImageProcessorOptions = $options;
        $(let $($mut )?$name = options.create_processor($image).process()$($end)?;)*
    };
    (@vec $images:expr) => {{
        fn _process_images(images: Vec<$crate::image::DynamicImage>) -> Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>> {
            use $crate::processor::ImageProcessorOptionsCreate;
            let display_mode = $crate::protocol::Protocol::Auto.builder().build();
            let option = $crate::processor::ImageProcessorOptions::default()
                    .option_display_mode(display_mode)
                    .get_options();

            if images.len() > 10 {
                use $crate::rayon::prelude::*;
                images
                    .into_par_iter()
                    .map(|image| option.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            } else {
                images
                    .into_iter()
                    .map(|image| option.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            }
        }
        _process_images($images)}
    };
    (@vec $images:expr$(,@with_options $options:expr)?,@var $var:ident,@map $block:block) => {
        $crate::__vec_process_images!($images, map, $var $(,options: $options)?, ty: Vec, collect: collect, block: $block)
    };
    (@vec $images:expr$(,@with_options $options:expr)?,@map $block:block) => {
        $crate::__vec_process_images!($images, map, result $(,options: $options)?, ty: Vec, collect: collect, result: result, block: $block)
    };
    (@vec $images:expr$(,@with_options $options:expr)?,@var $var:ident,@for_each $block:block) => {
        $crate::__vec_process_images!($images, for_each, $var $(,options: $options)?, block: $block, end: ;)
    };
    (@vec $images:expr$(,@with_options $options:expr)?,@for_each $block:block) => {
        $crate::__vec_process_images!($images, for_each, result $(,options: $options)?, block: $block, end: ;)
    };
    (@vec $images:expr,@with_options $options:expr) => {{
        fn _process_images(images: Vec<$crate::image::DynamicImage>, options: $crate::processor::ImageProcessorOptions) -> Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>> {
            use $crate::processor::ImageProcessorOptionsCreate;
            if images.len() > 10 {
                use $crate::rayon::prelude::*;
                images
                    .into_par_iter()
                    .map(|image| options.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            } else {
                images
                    .into_iter()
                    .map(|image| options.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            }
        }
        _process_images($images, $options)}
    };
    ($image:expr) => {{
        let display_mode = $crate::protocol::Protocol::Auto.builder().build();
        let option = $crate::processor::ImageProcessorOptions::default()
                .option_display_mode(display_mode)
                .get_options();
        fn _process_image(image: $crate::image::DynamicImage, option: $crate::processor::ImageProcessorOptions) -> $crate::error::ConvertResult<$crate::processor::ImageProcessorResult> {
            $crate::processor::ImageProcessor::new(image, option).process()
        }
        _process_image($image, option)}
    };
    ($image:expr,@with_options $options:expr) => {{
        fn _process_image(image: $crate::image::DynamicImage, option: $crate::processor::ImageProcessorOptions) -> $crate::error::ConvertResult<$crate::processor::ImageProcessorResult> {
            $crate::processor::ImageProcessor::new(image, option).process()
        }
        _process_image($image, $options)}
    };
    ($($image:expr),+,@with_options $options: expr) => {{
        fn _process_images(images: Vec<$crate::image::DynamicImage>, options: $crate::processor::ImageProcessorOptions) -> Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>> {
            use $crate::processor::ImageProcessorOptionsCreate;
            if images.len() > 10 {
                use $crate::rayon::prelude::*;
                images
                    .into_par_iter()
                    .map(|image| options.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            } else {
                images
                    .into_iter()
                    .map(|image| options.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            }
        }
        let options = $options;
        let images = vec![$($image),+];
        _process_images(images, options)
    }};
    ($($image:expr),+$(,)?) => {{
        fn _process_images(images: Vec<$crate::image::DynamicImage>) -> Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>> {
            use $crate::processor::ImageProcessorOptionsCreate;
            let display_mode = $crate::protocol::Protocol::Auto.builder().build();
            let option = $crate::processor::ImageProcessorOptions::default()
                    .option_display_mode(display_mode)
                    .get_options();

            if images.len() > 10 {
                use $crate::rayon::prelude::*;
                images
                    .into_par_iter()
                    .map(|image| option.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            } else {
                images
                    .into_iter()
                    .map(|image| option.create_processor(image).process())
                    .collect::<Vec<$crate::error::ConvertResult<$crate::processor::ImageProcessorResult>>>()
            }
        }
        let images = vec![$(
            $image
        ),+];
        _process_images(images)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processed_image_creation() {
        let img = DynamicImage::new_rgba8(10, 10);

        // Test color modes
        let processed = ProcessedImage::new(DisplayMode::HalfColor, &img);
        assert!(matches!(processed, ProcessedImage::Color(_)));

        let processed = ProcessedImage::new(DisplayMode::FullColor, &img);
        assert!(matches!(processed, ProcessedImage::Both(_, _)));

        let processed = ProcessedImage::new(DisplayMode::WezTerm, &img);
        assert!(matches!(processed, ProcessedImage::Color(_)));

        let processed = ProcessedImage::new(DisplayMode::Kitty, &img);
        assert!(matches!(processed, ProcessedImage::Color(_)));

        let processed = ProcessedImage::new(DisplayMode::Iterm2, &img);
        assert!(matches!(processed, ProcessedImage::Color(_)));

        // Test luma modes
        let processed = ProcessedImage::new(DisplayMode::Ascii, &img);
        assert!(matches!(processed, ProcessedImage::NoColor(_)));

        let processed = ProcessedImage::new(DisplayMode::FullNoColor, &img);
        assert!(matches!(processed, ProcessedImage::NoColor(_)));

        let processed = ProcessedImage::new(DisplayMode::WezTermNoColor, &img);
        assert!(matches!(processed, ProcessedImage::NoColor(_)));

        let processed = ProcessedImage::new(DisplayMode::KittyNoColor, &img);
        assert!(matches!(processed, ProcessedImage::NoColor(_)));

        let processed = ProcessedImage::new(DisplayMode::Iterm2NoColor, &img);
        assert!(matches!(processed, ProcessedImage::NoColor(_)));
    }

    #[test]
    fn test_processed_image_accessors() {
        let rgba_img = DynamicImage::new_rgba8(10, 10);
        let processed = ProcessedImage::new(DisplayMode::HalfColor, &rgba_img);

        // Test rgba accessor
        assert!(processed.rgba().is_some());

        // Test luma accessor (should be none for color mode)
        assert!(processed.luma().is_none());

        // Test both accessor (should be none for color mode)
        assert!(processed.both().is_none());

        // Test is_color
        assert!(processed.is_color());

        let rgba_img2 = DynamicImage::new_rgba8(10, 10);
        let processed_both = ProcessedImage::new(DisplayMode::FullColor, &rgba_img2);

        // Test both accessor
        assert!(processed_both.both().is_some());

        // Test is_color
        assert_eq!(processed_both.is_color(), false);

        let luma_img = DynamicImage::new_luma8(10, 10);
        let processed_luma = ProcessedImage::new(DisplayMode::Ascii, &luma_img);

        // Test rgba accessor (should be none for luma mode)
        assert!(processed_luma.rgba().is_none());

        // Test luma accessor
        assert!(processed_luma.luma().is_some());

        // Test is_color (should be false for luma mode)
        assert!(!processed_luma.is_color());
    }
}
