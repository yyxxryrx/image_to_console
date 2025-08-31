use crate::config::Cli;
use clap::builder::PossibleValue;
use clap::{Parser, ValueEnum};
use image::DynamicImage;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Normal,
    WezTerm,
    Kitty,
    ITerm2,
}
#[allow(dead_code)]
impl Default for Protocol {
    fn default() -> Self {
        Self::Normal
    }
}
impl ValueEnum for Protocol {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Normal, Self::WezTerm, Self::Kitty, Self::ITerm2]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Normal => PossibleValue::new("normal"),
            Self::WezTerm => PossibleValue::new("wezterm"),
            Self::Kitty => PossibleValue::new("kitty"),
            Self::ITerm2 => PossibleValue::new("iterm2"),
        })
    }
}

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
        !matches!(self, Self::HalfColor | Self::Ascii)
    }
    pub fn is_color(&self) -> bool {
        matches!(self, Self::FullColor | Self::HalfColor | Self::WezTerm | Self::Kitty | Self::Iterm2)
    }

    pub fn is_normal(&self) -> bool {
        matches!(self, Self::HalfColor | Self::FullColor | Self::Ascii | Self::FullNoColor)
    }

    pub fn is_wezterm(&self) -> bool {
        matches!(self, Self::WezTerm | Self::WezTermNoColor)
    }

    pub fn from_bool(full: bool, no_color: bool, protocol: Protocol) -> Self {
        match protocol {
            Protocol::Normal => match (full, no_color) {
                (true, true) => Self::FullNoColor,
                (true, false) => Self::FullColor,
                (false, false) => Self::HalfColor,
                (false, true) => Self::Ascii,
            },
            Protocol::WezTerm => match no_color {
                true => Self::WezTermNoColor,
                false => Self::WezTerm,
            },
            Protocol::Kitty => match no_color {
                true => Self::KittyNoColor,
                false => Self::Kitty,
            },
            Protocol::ITerm2 => match no_color {
                true => Self::Iterm2NoColor,
                false => Self::Iterm2,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum ImageType {
    Image(DynamicImage),
    Path(String),
}

#[derive(Debug, Clone, Copy)]
pub enum ClapResizeMode {
    Auto,
    Custom,
    None,
}

impl Default for ClapResizeMode {
    fn default() -> Self {
        Self::Auto
    }
}
impl ValueEnum for ClapResizeMode {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Auto, Self::Custom, Self::None]
    }
    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Auto => PossibleValue::new("auto"),
            Self::Custom => PossibleValue::new("custom"),
            Self::None => PossibleValue::new("none"),
        })
    }
}

#[derive(Debug, Clone, Copy, Parser)]
pub struct AutoResizeOption {
    // Resize with terminal width
    #[clap(short, long)]
    pub width: bool,
    // Resize with terminal height
    #[clap(short, long)]
    pub height: bool,
}

#[derive(Debug, Clone, Copy, Parser)]
pub struct CustomResizeOption {
    #[clap(short, long)]
    pub width: Option<u32>,
    #[clap(short, long)]
    pub height: Option<u32>,
}

#[derive(Debug, Clone, Copy)]
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

impl ResizeMode {
    pub fn from_cli(cli: &Cli) -> Self {
        match cli.resize_mode {
            ClapResizeMode::Auto => Self::Auto(AutoResizeOption {
                width: !(cli.without_resize_width || cli.no_resize),
                height: !(cli.without_resize_height || cli.no_resize),
            }),
            ClapResizeMode::Custom => Self::Custom(CustomResizeOption {
                width: cli.width,
                height: cli.height,
            }),
            ClapResizeMode::None => Self::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_crate_display_mode() {
        assert_eq!(
            DisplayMode::from_bool(false, false, Protocol::Normal),
            DisplayMode::HalfColor
        );
        assert_eq!(
            DisplayMode::from_bool(true, false, Protocol::Normal),
            DisplayMode::FullColor
        );
        assert_eq!(
            DisplayMode::from_bool(true, true, Protocol::Normal),
            DisplayMode::FullNoColor
        );
        assert_eq!(
            DisplayMode::from_bool(false, true, Protocol::Normal),
            DisplayMode::Ascii
        );
        assert_eq!(
            DisplayMode::from_bool(true, false, Protocol::WezTerm),
            DisplayMode::WezTerm
        );
        assert_eq!(
            DisplayMode::from_bool(true, true, Protocol::WezTerm),
            DisplayMode::WezTermNoColor
        );
        assert_eq!(
            DisplayMode::from_bool(false, false, Protocol::Kitty),
            DisplayMode::Kitty
        );
        assert_eq!(
            DisplayMode::from_bool(false, true, Protocol::Kitty),
            DisplayMode::KittyNoColor
        );
        assert_eq!(
            DisplayMode::from_bool(false, false, Protocol::ITerm2),
            DisplayMode::Iterm2
        );
        assert_eq!(
            DisplayMode::from_bool(false, true, Protocol::ITerm2),
            DisplayMode::Iterm2NoColor
        );
    }

    #[test]
    fn test_get_display_mode_info() {
        assert_eq!(DisplayMode::HalfColor.is_full(), false);
        assert_eq!(DisplayMode::HalfColor.is_color(), true);
        assert_eq!(DisplayMode::HalfColor.is_normal(), true);
        assert_eq!(DisplayMode::HalfColor.is_wezterm(), false);
        assert_eq!(DisplayMode::FullColor.is_full(), true);
        assert_eq!(DisplayMode::FullColor.is_color(), true);
        assert_eq!(DisplayMode::FullColor.is_normal(), true);
        assert_eq!(DisplayMode::FullColor.is_wezterm(), false);
        assert_eq!(DisplayMode::FullNoColor.is_full(), true);
        assert_eq!(DisplayMode::FullNoColor.is_color(), false);
        assert_eq!(DisplayMode::FullNoColor.is_normal(), true);
        assert_eq!(DisplayMode::FullNoColor.is_wezterm(), false);
        assert_eq!(DisplayMode::Ascii.is_full(), false);
        assert_eq!(DisplayMode::Ascii.is_color(), false);
        assert_eq!(DisplayMode::Ascii.is_normal(), true);
        assert_eq!(DisplayMode::Ascii.is_wezterm(), false);
        assert_eq!(DisplayMode::Kitty.is_full(), true);
        assert_eq!(DisplayMode::Kitty.is_color(), true);
        assert_eq!(DisplayMode::Kitty.is_normal(), false);
        assert_eq!(DisplayMode::Kitty.is_wezterm(), false);
        assert_eq!(DisplayMode::KittyNoColor.is_full(), true);
        assert_eq!(DisplayMode::KittyNoColor.is_color(), false);
        assert_eq!(DisplayMode::KittyNoColor.is_normal(), false);
        assert_eq!(DisplayMode::KittyNoColor.is_wezterm(), false);
        assert_eq!(DisplayMode::Iterm2.is_full(), true);
        assert_eq!(DisplayMode::Iterm2.is_color(), true);
        assert_eq!(DisplayMode::Iterm2.is_normal(), false);
        assert_eq!(DisplayMode::Iterm2.is_wezterm(), false);
        assert_eq!(DisplayMode::Iterm2NoColor.is_full(), true);
        assert_eq!(DisplayMode::Iterm2NoColor.is_color(), false);
        assert_eq!(DisplayMode::Iterm2NoColor.is_normal(), false);
        assert_eq!(DisplayMode::Iterm2NoColor.is_wezterm(), false);
    }
}
