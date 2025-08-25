#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisplayMode {
    HalfColor,
    FullColor,
    FullNoColor,
    Ascii,
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
        matches!(self, Self::FullColor | Self::HalfColor)
    }
    
    pub fn from_bool(full: bool, no_color: bool) -> Self {
        match (full, no_color) {
            (true, true) => Self::FullNoColor,
            (true, false) => Self::FullColor,
            (false, true) => Self::HalfColor,
            (false, false) => Self::Ascii,
        }
    }
}
