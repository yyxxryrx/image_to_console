use clap::builder::PossibleValue;
use clap::ValueEnum;
use image::DynamicImage;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Normal,
    WezTerm,
    Kitty,
    ITerm,
}
#[allow(dead_code)]
impl Default for Protocol {
    fn default() -> Self {
        Self::Normal
    }
}
impl ValueEnum for Protocol {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Normal, Self::WezTerm, Self::Kitty, Self::ITerm]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Normal => PossibleValue::new("normal"),
            Self::WezTerm => PossibleValue::new("wezterm"),
            Self::Kitty => PossibleValue::new("kitty"),
            Self::ITerm => PossibleValue::new("iterm"),
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
        matches!(self, Self::FullColor | Self::FullNoColor)
    }
    pub fn is_color(&self) -> bool {
        matches!(self, Self::FullColor | Self::HalfColor | Self::WezTerm)
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
            }
            Protocol::WezTerm => match no_color {
                true => Self::WezTermNoColor,
                false => Self::WezTerm,
            },
            _ => Self::default()
        }

    }
}

#[derive(Debug, Clone)]
pub enum ImageType {
    Image(DynamicImage),
    Path(String),
}
