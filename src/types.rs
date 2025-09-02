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
        matches!(
            self,
            Self::FullColor | Self::HalfColor | Self::WezTerm | Self::Kitty | Self::Iterm2
        )
    }

    pub fn is_normal(&self) -> bool {
        matches!(
            self,
            Self::HalfColor | Self::FullColor | Self::Ascii | Self::FullNoColor
        )
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
    Gif(Vec<DynamicImage>)
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

#[derive(Debug, Clone, Copy, Parser, PartialEq)]
pub struct AutoResizeOption {
    // Resize with terminal width
    #[clap(short, long)]
    pub width: bool,
    // Resize with terminal height
    #[clap(short, long)]
    pub height: bool,
}

#[derive(Debug, Clone, Copy, Parser, PartialEq)]
pub struct CustomResizeOption {
    #[clap(short, long)]
    pub width: Option<u32>,
    #[clap(short, long)]
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
        let test_cases = vec![
            ((false, false, Protocol::Normal), DisplayMode::HalfColor),
            ((true, false, Protocol::Normal), DisplayMode::FullColor),
            ((true, true, Protocol::Normal), DisplayMode::FullNoColor),
            ((false, true, Protocol::Normal), DisplayMode::Ascii),
            ((true, false, Protocol::WezTerm), DisplayMode::WezTerm),
            ((true, true, Protocol::WezTerm), DisplayMode::WezTermNoColor),
            ((false, false, Protocol::Kitty), DisplayMode::Kitty),
            ((false, true, Protocol::Kitty), DisplayMode::KittyNoColor),
            ((false, false, Protocol::ITerm2), DisplayMode::Iterm2),
            ((false, true, Protocol::ITerm2), DisplayMode::Iterm2NoColor),
        ];

        for ((full, no_color, protocol), expected) in test_cases {
            assert_eq!(
                DisplayMode::from_bool(full, no_color, protocol),
                expected,
                "Failed for input: full={}, no_color={}, protocol={:?}",
                full,
                no_color,
                protocol
            );
        }
        assert_eq!(
            DisplayMode::default(),
            DisplayMode::HalfColor,
            "Default display mode should be HalfColor"
        );
    }

    #[test]
    fn test_get_display_mode_info() {
        let mode_properties = vec![
            (DisplayMode::HalfColor, false, true, true),
            (DisplayMode::FullColor, true, true, true),
            (DisplayMode::FullNoColor, true, false, true),
            (DisplayMode::Ascii, false, false, true),
            (DisplayMode::Kitty, true, true, false),
            (DisplayMode::KittyNoColor, true, false, false),
            (DisplayMode::Iterm2, true, true, false),
            (DisplayMode::Iterm2NoColor, true, false, false),
        ];

        for (mode, is_full, is_color, is_normal) in mode_properties {
            assert_eq!(
                mode.is_full(),
                is_full,
                "is_full check failed for {:?}",
                mode
            );
            assert_eq!(
                mode.is_color(),
                is_color,
                "is_color check failed for {:?}",
                mode
            );
            assert_eq!(
                mode.is_normal(),
                is_normal,
                "is_normal check failed for {:?}",
                mode
            );
        }
    }

    #[test]
    fn test_crate_resize_mode() {
        let mut cli = Cli::default();
        let test_cases = vec![(true, true), (true, false), (false, true), (false, false)];
        for (width, height) in test_cases {
            cli.without_resize_width = !width;
            cli.without_resize_height = !height;
            assert_eq!(
                ResizeMode::from_cli(&cli),
                ResizeMode::Auto(AutoResizeOption { width, height }),
                "Failed for input: width={}, height={}",
                width,
                height
            );
        }
        cli.resize_mode = ClapResizeMode::None;
        assert_eq!(
            ResizeMode::from_cli(&cli),
            ResizeMode::None,
            "Default resize mode should be None"
        );
        let test_cases = vec![
            (Some(10), Some(20)),
            (Some(10), None),
            (None, Some(20)),
            (None, None),
        ];
        cli.resize_mode = ClapResizeMode::Custom;
        for (width, height) in test_cases {
            cli.width = width;
            cli.height = height;
            assert_eq!(
                ResizeMode::from_cli(&cli),
                ResizeMode::Custom(CustomResizeOption { width, height }),
                "Failed for input: width={:?}, height={:?}",
                width,
                height
            );
        }
    }
}
