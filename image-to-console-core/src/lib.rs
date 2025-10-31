pub mod converter;
#[cfg(feature = "gif")]
pub mod gif_processor;
#[cfg(feature = "sixel")]
pub mod indexed_image;
pub mod processor;
pub mod protocol;

#[cfg(feature = "sixel")]
use image::RgbImage;
use image::{DynamicImage, GrayImage, RgbaImage};

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
}

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
    /// * `true` - If the processed image contains color data (Color or Both variants)
    /// * `false` - If the processed image only contains grayscale data (NoColor variant)
    pub fn is_color(&self) -> bool {
        matches!(self, Self::Color(_) | Self::Both(_, _))
    }
}

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
macro_rules! show_image {
    ($image:expr) => {
        fn _show_image(image: image::DynamicImage) {
            let display_mode = image_to_console_core::protocol::Protocol::Auto
                .builder()
                .build();
            let result = image_to_console_core::processor::ImageProcessorOptions::default()
                .option_display_mode(display_mode)
                .create_processor(image)
                .process();
            println!("{}", result.display());
        }
        _show_image($image);
    };
    ($image: expr, $option: expr) => {
        fn _show_image(
            image: image::DynamicImage,
            option: image_to_console_core::processor::ImageProcessorOptions,
        ) {
            let result = option.create_processor(image).process();
            println!("{}", result.display());
        }
        _show_image($image, $option);
    };
}

#[macro_export]
/// A macro to display multiple images to the terminal.
///
/// This macro allows displaying multiple images either with default options or with
/// shared custom options. It's useful when you want to display a series of images
/// with the same processing settings.
///
/// # Arguments
///
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
/// ```
macro_rules! show_images {
    ($($image:expr),+, @with_options $option: expr) => {
        fn _show_image(image: image::DynamicImage, option: image_to_console_core::processor::ImageProcessorOptions) {
            let result = option
                .create_processor(image)
                .process();
            println!("{}", result.display());
        }
        let option = $option;
        $(
            _show_image($image, option);
        )+
    };
    ($($image:expr),+) => {
        fn _show_image(image: image::DynamicImage) {
            let display_mode = image_to_console_core::protocol::Protocol::Auto.builder().build();
            let result = image_to_console_core::processor::ImageProcessorOptions::default()
                .option_display_mode(display_mode)
                .create_processor(image)
                .process();
            println!("{}", result.display());
        }
        $(
            _show_image($image);
        )+
    };
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
        assert!(processed_both.is_color());

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
