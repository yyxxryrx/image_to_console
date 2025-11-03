use std::{
    error::Error,
    fmt::{Display, Formatter},
};

/// The source of an error context, indicating where the error occurred
#[derive(Debug, PartialEq, Eq)]
pub enum ConvertErrorContextSource {
    /// Error occurred at a specific pixel coordinate (x, y)
    Pixel(u32, u32),
    /// Error occurred during sixel conversion process
    SixelConvert,
    /// Error occurred in a specific function
    Function(String),
}

/// Context information for an error, including the source and a descriptive message
#[derive(Debug)]
pub struct ConvertErrorContext {
    /// The source where the error occurred
    pub source: ConvertErrorContextSource,
    /// A descriptive message explaining the error
    pub message: String,
    /// The optional source error
    pub inner: Option<Box<dyn Error + Send>>,
}

impl PartialEq for ConvertErrorContext {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source && self.message == other.message
    }
}

impl ConvertErrorContext {
    /// Creates a new error context with the given source and message
    ///
    /// # Arguments
    ///
    /// * `source` - The source where the error occurred
    /// * `message` - A descriptive message explaining the error
    ///
    /// # Returns
    ///
    /// A new `ConvertErrorContext` instance
    pub fn new(source: ConvertErrorContextSource, message: String) -> Self {
        Self {
            source,
            message,
            inner: None,
        }
    }

    pub fn with_inner(self, inner: Box<dyn Error + Send>) -> Self {
        Self {
            inner: Some(inner),
            ..self
        }
    }
}

/// Represents all possible errors that can occur during image conversion
#[derive(Debug, PartialEq)]
pub enum ConvertError {
    /// The input data is empty
    EmptyData,
    /// The image type does not match the expectation
    WrongImageType {
        /// The expected image type
        expect_type: String,
        /// The actual image type
        actual_type: String,
    },
    /// An error occurred while trying to get terminal size
    GetTerminalSizeError,
    /// An error type for when the length of an input (e.g., `Vec` or slice)
    /// is above the maximum supported value.
    ///
    /// The inner value is the maximum supported value.
    ///
    /// > tips: This error is from the `quantette`
    AboveMaxLength(u32, ConvertErrorContext),
    /// An error occurred while trying to acquire a lock
    LockError(ConvertErrorContext),
    /// An error related with image
    ImageError(ConvertErrorContext),
}

impl Display for ConvertError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConvertError::EmptyData => write!(f, "Empty data"),
            ConvertError::WrongImageType {
                actual_type,
                expect_type,
            } => write!(
                f,
                "Unsupported Image Type, expect {}, but actual {}",
                expect_type, actual_type
            ),
            ConvertError::GetTerminalSizeError => write!(f, "Terminal size error"),
            ConvertError::AboveMaxLength(len, _) => {
                write!(f, "above the maximum length of {}", len)
            }
            ConvertError::LockError(context) => write!(f, "{}", context.message),
            ConvertError::ImageError(context) => write!(f, "{}", context.message),
        }
    }
}

impl Error for ConvertError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::EmptyData
            | Self::GetTerminalSizeError
            | Self::WrongImageType {
                actual_type: _,
                expect_type: _,
            } => None,
            Self::AboveMaxLength(_, context)
            | Self::LockError(context)
            | Self::ImageError(context) => context
                .inner
                .as_ref()
                .map(|e| e.as_ref() as &(dyn Error + 'static)),
        }
    }
}

/// A type alias for `Result<T, ConvertError>`
pub type ConvertResult<T> = Result<T, ConvertError>;
