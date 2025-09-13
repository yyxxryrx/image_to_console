#[allow(unused_imports)]
use std::fmt::{Debug, Display};
#[cfg(feature = "video_player")]
#[derive(Clone)]
pub enum FrameError {
    EOF,
    DecodeError,
    Other(String),
}

#[cfg(feature = "video_player")]
impl Debug for FrameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EOF => write!(f, "EOF"),
            Self::DecodeError => write!(f, "DecodeError"),
            Self::Other(arg0) => write!(f, "Other({:?})", arg0),
        }
    }
}

#[cfg(feature = "video_player")]
impl Display for FrameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EOF => write!(f, "end of frames"),
            Self::DecodeError => write!(f, "decode error"),
            Self::Other(arg0) => write!(f, "Other error: {}", arg0),
        }
    }
}
