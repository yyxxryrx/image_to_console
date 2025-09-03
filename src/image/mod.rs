use crate::types::DisplayMode::{self, *};
use image::{DynamicImage, GrayImage, RgbaImage};

mod converter;
pub mod processor;
pub mod gif_processor;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessedImage {
    Color(RgbaImage),
    NoColor(GrayImage),
    Both(RgbaImage, GrayImage),
}

#[allow(dead_code)]
impl ProcessedImage {
    pub fn new(mode: DisplayMode, img: &DynamicImage) -> Self {
        match mode {
            Ascii => Self::NoColor(img.to_luma8()),
            Kitty => Self::Color(img.to_rgba8()),
            Iterm2 => Self::Color(img.to_rgba8()),
            WezTerm => Self::Color(img.to_rgba8()),
            HalfColor => Self::Color(img.to_rgba8()),
            FullNoColor => Self::NoColor(img.to_luma8()),
            KittyNoColor => Self::NoColor(img.to_luma8()),
            Iterm2NoColor => Self::NoColor(img.to_luma8()),
            WezTermNoColor => Self::NoColor(img.to_luma8()),
            FullColor => Self::Both(img.to_rgba8(), img.to_luma8()),
        }
    }
    pub fn rgba(&self) -> Option<&RgbaImage> {
        match self {
            Self::Color(img) => Some(img),
            Self::Both(img, _) => Some(img),
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
