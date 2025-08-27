use image::DynamicImage;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    HalfColor,
    FullColor,
    FullNoColor,
    Ascii,
    WezTerm,
    WezTermNoColor
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

    pub fn from_bool(full: bool, no_color: bool, wez: bool) -> Self {
        if wez {
            return if no_color {
                Self::WezTermNoColor
            } else {
                Self::WezTerm
            };
        }
        match (full, no_color) {
            (true, true) => Self::FullNoColor,
            (true, false) => Self::FullColor,
            (false, false) => Self::HalfColor,
            (false, true) => Self::Ascii,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ImageType {
    Image(DynamicImage),
    Path(String),
}
