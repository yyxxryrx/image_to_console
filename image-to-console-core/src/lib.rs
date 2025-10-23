pub mod converter;
#[cfg(feature = "gif")]
pub mod gif_processor;
#[cfg(feature = "sixel")]
pub mod indexed_image;
pub mod processor;

#[cfg(feature = "sixel")]
use image::RgbImage;
use image::{DynamicImage, GrayImage, RgbaImage};

/// The protocol of dispaly
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
    pub fn is_full(&self) -> bool {
        #[cfg(feature = "sixel")]
        return !matches!(self, Self::HalfColor | Self::Ascii | Self::SixelHalf);
        #[cfg(not(feature = "sixel"))]
        return !matches!(self, Self::HalfColor | Self::Ascii);
    }
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

    pub fn is_luma(&self) -> bool {
        !self.is_color()
    }

    pub fn is_normal(&self) -> bool {
        matches!(
            self,
            Self::HalfColor | Self::FullColor | Self::Ascii | Self::FullNoColor
        )
    }

    pub fn is_wezterm(&self) -> bool {
        matches!(self, Self::WezTerm | Self::WezTermNoColor)
    }

    pub fn is_iterm2(&self) -> bool {
        matches!(self, Self::Iterm2 | Self::Iterm2NoColor)
    }

    #[cfg(feature = "sixel")]
    pub fn is_sixel(&self) -> bool {
        matches!(self, Self::SixelHalf | Self::SixelFull)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProcessedImage {
    Color(RgbaImage),
    #[cfg(feature = "sixel")]
    Color2(RgbImage),
    NoColor(GrayImage),
    Both(RgbaImage, GrayImage),
}

impl ProcessedImage {
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
    pub fn rgba(&self) -> Option<&RgbaImage> {
        match self {
            Self::Color(img) => Some(img),
            Self::Both(img, _) => Some(img),
            _ => None,
        }
    }

    #[cfg(feature = "sixel")]
    pub fn rgb(&self) -> Option<&RgbImage> {
        match self {
            Self::Color2(img) => Some(img),
            _ => None,
        }
    }

    pub fn luma(&self) -> Option<&GrayImage> {
        match self {
            Self::NoColor(img) => Some(img),
            Self::Both(_, img) => Some(img),
            _ => None,
        }
    }

    pub fn both(&self) -> Option<(&RgbaImage, &GrayImage)> {
        match self {
            Self::Both(rgba, luma) => Some((rgba, luma)),
            _ => None,
        }
    }

    pub fn is_color(&self) -> bool {
        matches!(self, Self::Color(_) | Self::Both(_, _))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutoResizeOption {
    // Resize with terminal width
    pub width: bool,
    // Resize with terminal height
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

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CustomResizeOption {
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl CustomResizeOption {
    pub fn new(width: Option<u32>, height: Option<u32>) -> Self {
        Self { width, height }
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
    /// Returns to the default resize mode [Auto](file:///D:/Desktop/Work/Rust/image_to_console/image-to-console-core/src/lib.rs#L168-L168) and the width and height are automatically adjusted
    fn default() -> Self {
        Self::Auto(AutoResizeOption::default())
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
