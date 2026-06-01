use std::fmt::Formatter;

pub type VideoResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidStream,
    ToImageFailed,
    FFmpegError(ffmpeg_next::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidStream => write!(f, "Invalid video stream"),
            Self::ToImageFailed => write!(f, "To image failed"),
            Self::FFmpegError(err) => write!(f, "FFmpeg Error: {err}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FFmpegError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<ffmpeg_next::Error> for Error {
    fn from(err: ffmpeg_next::Error) -> Self {
        Self::FFmpegError(err)
    }
}
