use clap::{ValueEnum, builder::PossibleValue};
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

impl Default for ImageType {
    fn default() -> Self {
        Self::Image(DynamicImage::new_rgb8(1, 1))
    }
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

#[cfg(feature = "video_player")]
#[derive(Debug, Clone, Copy)]
/// Represents how often the display should be refreshed when playing videos.
///
/// This enum controls the frequency at which the video player updates the displayed frame.
/// It can be specified in seconds, frames, or set to always/never refresh.
pub enum FlushInterval {
    /// Refresh every N seconds. Takes a `f32` representing the number of seconds.
    Seconds(f32),
    /// Refresh every N frames. Takes a `u64` representing the number of frames.
    Frames(u64),
    /// Refresh on every frame (equivalent to Frames(1)).
    Always,
    /// Never refresh the display after initial render.
    Never,
}

#[cfg(feature = "video_player")]
impl Default for FlushInterval {
    /// Returns the default flush interval, which is 1 second.
    fn default() -> Self {
        Self::Seconds(1f32)
    }
}

#[cfg(feature = "video_player")]
impl std::str::FromStr for FlushInterval {
    type Err = String;

    /// Parses a string into a `FlushInterval`.
    ///
    /// Supports the following formats:
    /// - "Ns" where N is a number (e.g., "2.5s") - interpreted as seconds
    /// - "Nf" where N is a number (e.g., "30f") - interpreted as frames
    /// - "always" - refresh on every frame
    /// - "never" - never refresh after initial render
    ///
    /// # Examples
    /// ```
    /// use std::str::FromStr;
    /// # use your_crate::types::FlushInterval;
    ///
    /// let interval = FlushInterval::from_str("2.5s").unwrap();
    /// assert!(matches!(interval, FlushInterval::Seconds(2.5)));
    ///
    /// let interval = FlushInterval::from_str("30f").unwrap();
    /// assert!(matches!(interval, FlushInterval::Frames(30)));
    ///
    /// let interval = FlushInterval::from_str("always").unwrap();
    /// assert!(matches!(interval, FlushInterval::Always));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        if s == "always" {
            return Ok(Self::Always);
        }
        if s == "never" {
            return Ok(Self::Never);
        }
        // negative numbers are not allowed
        if s.starts_with("-") {
            return Err("Invalid flush interval".to_string());
        }
        // seconds
        if s.ends_with("s") {
            let num = s
                .chars()
                .take_while(|c| c.is_ascii_digit() || *c == '.')
                .collect::<String>()
                .parse::<f32>()
                .map_err(|err| err.to_string())?;
            return if num > 0f32 {
                Ok(Self::Seconds(num))
            } else {
                Err("Invalid flush interval".to_string())
            };
        }
        // frames
        if s.ends_with("f") {
            let num = s
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u64>()
                .map_err(|err| err.to_string())?;
            return if num > 0 {
                Ok(Self::Frames(num))
            } else {
                Err("Invalid flush interval".to_string())
            };
        }
        Err("Invalid flush interval".to_string())
    }
}

#[cfg(feature = "video_player")]
impl FlushInterval {
    /// Converts the flush interval to a number of frames based on the given FPS.
    ///
    /// # Arguments
    /// * `fps` - The frames per second of the video
    ///
    /// # Returns
    /// The number of frames between each flush as `usize`.
    ///
    /// # Behavior
    /// - `Always` returns 1 (flush every frame)
    /// - `Never` returns `usize::MAX` (essentially never flush)
    /// - `Frames(n)` returns n
    /// - `Seconds(s)` returns s * fps rounded to nearest usize
    pub fn to_frames(&self, fps: f32) -> usize {
        match self {
            Self::Always => 1,
            Self::Never => usize::MAX,
            Self::Frames(frames) => *frames as usize,
            Self::Seconds(seconds) => (seconds * fps) as usize,
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    #[cfg(feature = "video_player")]
    fn test_flush_interval_parsing() {
        use std::str::FromStr;
        assert!(matches!(
            FlushInterval::from_str("1.5s").unwrap(),
            FlushInterval::Seconds(1.5)
        ));
        assert!(matches!(
            FlushInterval::from_str("30f").unwrap(),
            FlushInterval::Frames(30)
        ));
        assert!(matches!(
            FlushInterval::from_str("always").unwrap(),
            FlushInterval::Always
        ));
        assert!(matches!(
            FlushInterval::from_str("never").unwrap(),
            FlushInterval::Never
        ));
        assert!(matches!(FlushInterval::from_str("-1.5s"), Err(_)));
        assert!(matches!(FlushInterval::from_str("0f"), Err(_)));
    }

    #[test]
    #[cfg(feature = "video_player")]
    fn test_to_frames_conversion() {
        let fps = 30.0;
        assert_eq!(FlushInterval::Always.to_frames(fps), 1);
        assert_eq!(FlushInterval::Never.to_frames(fps), usize::MAX);
        assert_eq!(FlushInterval::Frames(10).to_frames(fps), 10);
        assert_eq!(FlushInterval::Seconds(2.0).to_frames(fps), 60);
    }
}
