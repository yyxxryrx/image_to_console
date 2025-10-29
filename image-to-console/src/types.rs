use clap::ValueEnum;
use clap::builder::PossibleValue;
use image::DynamicImage;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum ImageType {
    Image(DynamicImage),
    Path(String),
    #[cfg(feature = "gif_player")]
    Gif(crossbeam_channel::Receiver<Result<(DynamicImage, usize, u16), String>>),
    #[cfg(feature = "video_player")]
    /// The channel to receive video events
    Video(crossbeam_channel::Receiver<Result<VideoEvent, String>>),
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

/// The event type to of video parser
#[cfg(feature = "video_player")]
#[derive(Debug, Clone)]
pub enum VideoEvent {
    Starting,
    /// The first one is the receiver of the video data
    ///
    /// The last one is the frame rate.
    #[cfg(not(feature = "audio_support"))]
    Initialized(
        (
            crossbeam_channel::Receiver<Result<(DynamicImage, usize), crate::errors::FrameError>>,
            f32,
        ),
    ),
    #[cfg(feature = "audio_support")]
    Initialized(
        (
            crossbeam_channel::Receiver<Result<(DynamicImage, usize), crate::errors::FrameError>>,
            image_to_console_renderer::audio_path::AudioPath,
            f32,
        ),
    ),
    Finished,
}
