use clap::builder::PossibleValue;
use clap::ValueEnum;
use image::DynamicImage;
use std::fmt::Debug;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Auto,
    Normal,
    WezTerm,
    Kitty,
    ITerm2,
    #[cfg(feature = "sixel_support")]
    Sixel,
}
#[allow(dead_code)]
impl Default for Protocol {
    fn default() -> Self {
        Self::Normal
    }
}
impl ValueEnum for Protocol {
    fn value_variants<'a>() -> &'a [Self] {
        #[cfg(not(feature = "sixel_support"))]
        return &[Self::Normal, Self::WezTerm, Self::Kitty, Self::ITerm2];
        #[cfg(feature = "sixel_support")]
        &[Self::Auto, Self::Normal, Self::WezTerm, Self::Kitty, Self::ITerm2, Self::Sixel]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Auto => PossibleValue::new("auto"),
            Self::Normal => PossibleValue::new("normal"),
            Self::WezTerm => PossibleValue::new("wezterm"),
            Self::Kitty => PossibleValue::new("kitty"),
            Self::ITerm2 => PossibleValue::new("iterm2"),
            #[cfg(feature = "sixel_support")]
            Self::Sixel => PossibleValue::new("sixel"),
        })
    }
}

#[derive(Debug, Clone)]
pub enum ImageType {
    Image(DynamicImage),
    Path(String),
    #[cfg(feature = "gif_player")]
    Gif(crossbeam_channel::Receiver<Result<(DynamicImage, usize, u16), String>>),
    #[cfg(feature = "video_player")]
    /// The channel to receive video events
    Video(crossbeam_channel::Receiver<Result<VideoEvent, String>>)
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

#[cfg(feature = "gif_player")]
#[derive(Clone)]
pub struct Frame {
    pub index: usize,
    pub frame: String,
    pub delay: u64,
}
#[cfg(feature = "gif_player")]
impl Frame {
    pub fn unpacking(&self) -> (&str, usize, u64) {
        (&self.frame, self.index, self.delay)
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
    Initialized((crossbeam_channel::Receiver<Result<(DynamicImage, usize), crate::errors::FrameError>>, crate::config::AudioPath, f32)),
    Finished,
}

