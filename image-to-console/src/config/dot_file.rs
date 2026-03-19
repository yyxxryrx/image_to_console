use serde::{Deserialize, Serialize};
#[cfg(feature = "video_player")]
use serde_json::{Map, Value};
use summon_schema::Schema;

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Schema)]
#[serde(rename_all = "kebab-case")]
pub enum RunType {
    File,
    Bytes,
    Base64,
    Directory,
    #[cfg(feature = "url")]
    Url,
    #[cfg(feature = "gif_player")]
    Gif,
    #[cfg(feature = "video_player")]
    Video,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Schema)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Auto,
    Normal,
    Wezterm,
    Kitty,
    ITerm2,
    Sixel,
}

impl From<Protocol> for image_to_console_core::protocol::Protocol {
    fn from(value: Protocol) -> Self {
        match value {
            Protocol::Auto => Self::Auto,
            Protocol::Sixel => Self::Sixel,
            Protocol::Kitty => Self::Kitty,
            Protocol::ITerm2 => Self::ITerm2,
            Protocol::Normal => Self::Normal,
            Protocol::Wezterm => Self::WezTerm,
        }
    }
}

impl Default for Protocol {
    fn default() -> Self {
        Self::Auto
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Schema)]
#[serde(rename_all = "lowercase")]
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

impl From<ClapResizeMode> for crate::types::ClapResizeMode {
    fn from(value: ClapResizeMode) -> Self {
        match value {
            ClapResizeMode::Auto => Self::Auto,
            ClapResizeMode::Custom => Self::Custom,
            ClapResizeMode::None => Self::None,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, Schema)]
#[serde(rename_all = "lowercase")]
pub enum ColorSpace {
    Srgb,
    Oklab,
    Lab,
}

impl From<ColorSpace> for crate::types::ColorSpace {
    fn from(value: ColorSpace) -> Self {
        match value {
            ColorSpace::Srgb => Self::Srgb,
            ColorSpace::Lab => Self::Lab,
            ColorSpace::Oklab => Self::Oklab,
        }
    }
}

impl Default for ColorSpace {
    fn default() -> Self {
        Self::Srgb
    }
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, Schema)]
#[serde(rename_all = "kebab-case")]
pub struct FileArgs {
    #[serde(default)]
    pub hide_filename: bool,
}

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize, Schema)]
#[serde(rename_all = "kebab-case")]
pub struct DirectoryArgs {
    #[serde(default)]
    pub read_all: bool,
}

#[cfg(feature = "gif_player")]
#[derive(Debug, Clone, Default, Deserialize, Serialize, Schema)]
#[serde(rename_all = "kebab-case")]
pub struct GifArgs {
    #[serde(default)]
    pub fps: Option<u64>,
    #[serde(default)]
    pub loop_play: bool,
    #[serde(default)]
    pub audio: Option<String>,
}

#[cfg(feature = "video_player")]
fn from_str<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(serde::de::Error::custom)
}

#[cfg(feature = "video_player")]
impl summon_schema::ToSchema for crate::types::FlushInterval {
    fn schema_type() -> Value {
        serde_json::json!("string")
    }

    fn schema() -> Map<String, Value> {
        summon_schema::map! {
            "default": "1s"
        }
    }
}

#[cfg(feature = "video_player")]
fn to_str<S>(flush_interval: &crate::types::FlushInterval, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&flush_interval.to_string())
}

#[cfg(feature = "video_player")]
#[derive(Debug, Clone, Default, Deserialize, Serialize, Schema)]
#[serde(rename_all = "kebab-case")]
pub struct VideoArgs {
    #[serde(default, deserialize_with = "from_str", serialize_with = "to_str")]
    pub flush_interval: crate::types::FlushInterval,
    #[serde(default)]
    pub audio: Option<String>,
}

fn default_max_colors() -> u16 {
    256
}

fn deserialize_max_colors<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = u16::deserialize(deserializer)?;
    if (1..=256).contains(&value) {
        Ok(value)
    } else {
        // 返回自定义错误信息
        Err(serde::de::Error::custom(format!(
            "max-colors must be between 1 and 256, got {}",
            value
        )))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Schema)]
#[serde(rename_all = "kebab-case")]
pub struct DotFileContent {
    pub r#type: RunType,
    pub path: String,
    #[serde(default)]
    pub center: bool,
    #[serde(default)]
    pub clear: bool,
    #[serde(default)]
    pub pause: bool,
    #[serde(default)]
    pub show_time: bool,
    #[serde(default)]
    pub output: Option<String>,
    #[serde(default)]
    pub half_resolution: bool,
    #[serde(default)]
    pub disable_print: bool,
    #[serde(default)]
    pub disable_info: bool,
    #[serde(default)]
    pub no_color: bool,
    #[serde(default)]
    pub black_background: bool,
    #[serde(default)]
    pub no_resize: bool,
    #[serde(default)]
    pub protocol: Protocol,
    #[serde(default)]
    pub resize_mode: ClapResizeMode,
    #[serde(default)]
    pub width: Option<u32>,
    #[serde(default)]
    pub height: Option<u32>,
    #[serde(default)]
    pub without_resize_width: bool,
    #[serde(default)]
    pub without_resize_height: bool,
    #[cfg(feature = "sixel_support")]
    #[serde(
        default = "default_max_colors",
        deserialize_with = "deserialize_max_colors"
    )]
    pub max_colors: u16,
    #[serde(default)]
    pub enable_compression: bool,
    #[serde(default)]
    #[cfg(feature = "sixel_support")]
    pub disable_dither: bool,
    #[serde(default)]
    #[cfg(feature = "sixel_support")]
    pub color_space: ColorSpace,
    #[serde(default)]
    pub file: Option<FileArgs>,
    #[serde(default)]
    pub directory: Option<DirectoryArgs>,
    #[serde(default)]
    #[cfg(feature = "gif_player")]
    pub gif: Option<GifArgs>,
    #[serde(default)]
    #[cfg(feature = "video_player")]
    pub video: Option<VideoArgs>,
}

macro_rules! make_cli {
    (var: $var:ident, command: $command:expr) => {{
        let var: &DotFileContent = $var;
        let command: super::cli::Commands = $command;
        super::cli::Cli {
            command,
            center: var.center,
            clear: var.clear,
            pause: var.pause,
            show_time: var.show_time,
            output: var.output.clone(),
            half_resolution: var.half_resolution,
            disable_print: var.disable_print,
            disable_info: var.disable_info,
            no_color: var.no_color,
            black_background: var.black_background,
            no_resize: var.no_resize,
            protocol: var.protocol.into(),
            resize_mode: var.resize_mode.into(),
            width: var.width,
            height: var.height,
            without_resize_width: var.without_resize_width,
            without_resize_height: var.without_resize_height,
            max_colors: var.max_colors,
            enable_compression: var.enable_compression,
            disable_dither: var.disable_dither,
            color_space: var.color_space.into(),
        }
    }};
    ($var:ident: $cmd:ident) => {
        make_cli!(var: $var, command: super::cli::Commands::$cmd)
    };
    ($var:ident: $cmd:ident.$args:ident Default) => {
        make_cli!(var: $var, command: super::cli::Commands::$cmd(Default::default()))
    };
    ($var:ident: $cmd:ident.$args:ident {$($tt:tt)*}) => {
        make_cli!(var: $var, command: super::cli::Commands::$cmd(super::cli::$args {
            $($tt)*
        }))
    };
}

impl From<&DotFileContent> for super::cli::Cli {
    fn from(value: &DotFileContent) -> Self {
        match value.r#type {
            RunType::File => {
                make_cli!(value: File.FileArgs {
                    hide_filename: value.file.unwrap_or_default().hide_filename,
                    path: value.path.clone(),
                })
            }
            RunType::Base64 => make_cli!(value: Base64.Base64Args {
                base64: value.path.clone(),
            }),
            RunType::Bytes => make_cli!(value: Bytes),
            RunType::Directory => make_cli!(value: Directory.DirectoryArgs {
                read_all: value.directory.unwrap_or_default().read_all,
                path: value.path.clone(),
            }),
            #[cfg(feature = "url")]
            RunType::Url => make_cli!(value: Url.UrlArgs {
                url: value.path.clone(),
            }),
            #[cfg(feature = "gif_player")]
            RunType::Gif => {
                let config = value.clone().gif.unwrap_or_default();
                make_cli!(value: Gif.GifArgs {
                    path: value.path.clone(),
                    fps: config.fps,
                    #[cfg(feature = "audio_support")]
                    audio: config.audio,
                    loop_play: config.loop_play,
                })
            }
            #[cfg(feature = "video_player")]
            RunType::Video => {
                let config = value.clone().video.unwrap_or_default();
                make_cli!(value: Video.VideoArgs {
                    path: value.path.clone(),
                    audio: config.audio,
                    flush_interval: config.flush_interval,
                })
            }
        }
    }
}
