use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ConvertErrorContextSource {
    Pixel(u32, u32),
    SixelConvert,
    Function(String),
}

#[derive(Debug)]
pub struct ConvertErrorContext {
    pub source: ConvertErrorContextSource,
    pub message: String,
}

impl ConvertErrorContext {
    pub fn new(source: ConvertErrorContextSource, message: String) -> Self {
        Self { source, message }
    }
}

#[derive(Debug)]
pub enum ConvertError {
    EmptyData,
    UnsupportedImageType,
    TerminalSizeError,
    AboveMaxLength(u32),
    LockError(ConvertErrorContext),
    ImageWriteError(ConvertErrorContext),
}

impl Display for ConvertError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self { 
            ConvertError::EmptyData => write!(f, "Empty data"),
            ConvertError::UnsupportedImageType => write!(f, "Unsupported image type"),
            ConvertError::TerminalSizeError => write!(f, "Terminal size error"),
            ConvertError::AboveMaxLength(len) => write!(f, "Length of the string is above max length: {}", len),
            ConvertError::LockError(context) => write!(f, "{}", context.message),
            ConvertError::ImageWriteError(context) => write!(f, "{}", context.message),
        }
    }
}

pub type ConvertResult<T> = Result<T, ConvertError>;
