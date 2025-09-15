mod converter;
#[cfg(feature = "gif")]
pub mod gif_processor;
pub mod processor;
#[cfg(feature = "sixel")]
pub mod indexed_image;

#[allow(unused_imports)]
use image::{DynamicImage, GrayImage, RgbImage, RgbaImage};

#[allow(dead_code)]
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
#[allow(dead_code)]
impl Default for DisplayMode {
    fn default() -> Self {
        Self::HalfColor
    }
}
#[allow(dead_code)]
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
            Self::FullColor | Self::HalfColor | Self::WezTerm | Self::Kitty | Self::Iterm2 | Self::SixelHalf | Self::SixelFull
        );
        #[cfg(not(feature = "sixel"))]
        return matches!(
            self,
            Self::FullColor | Self::HalfColor | Self::WezTerm | Self::Kitty | Self::Iterm2
        );
    }

    pub fn is_normal(&self) -> bool {
        matches!(
            self,
            Self::HalfColor | Self::FullColor | Self::Ascii | Self::FullNoColor
        )
    }
    
    #[cfg(feature = "sixel")]
    pub fn is_sixel(&self) -> bool {
        matches!(self, Self::SixelHalf | Self::SixelFull)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessedImage {
    Color(RgbaImage),
    Color2(RgbImage),
    NoColor(GrayImage),
    Both(RgbaImage, GrayImage),
}

#[allow(dead_code)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CustomResizeOption {
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResizeMode {
    // Resize with terminal size
    Auto(AutoResizeOption),
    // Resize with given size
    Custom(CustomResizeOption),
    // No resize
    None,
}

impl Default for ResizeMode {
    fn default() -> Self {
        Self::Auto(AutoResizeOption {
            width: true,
            height: true,
        })
    }
}
