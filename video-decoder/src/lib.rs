#![deny(clippy::unwrap_used)]
mod decoder;
mod error;
mod video;

pub use decoder::{VideoDecoder, VideoFrame};
pub use error::*;
pub use video::Video;

pub fn open<P>(path: P) -> VideoResult<Video>
where
    P: AsRef<std::path::Path>,
{
    let input = ffmpeg_next::format::input(&path)?;
    Ok(Video::new(input))
}
