use crate::types::ClapResizeMode;
use clap::{
    Parser, Subcommand,
    builder::{
        Styles,
        styling::{AnsiColor, Color, Style},
    },
};
use image_to_console_core::protocol::Protocol;

pub const CLAP_STYLING: Styles = Styles::styled()
    .header(Style::new().fg_color(Some(Color::Ansi(AnsiColor::BrightGreen))))
    .usage(Style::new().fg_color(Some(Color::Ansi(AnsiColor::BrightGreen))))
    .literal(Style::new().fg_color(Some(Color::Ansi(AnsiColor::BrightCyan))))
    .placeholder(Style::new().fg_color(Some(Color::Ansi(AnsiColor::BrightBlue))))
    .error(Style::new().fg_color(Some(Color::Ansi(AnsiColor::BrightRed))))
    .valid(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Blue))))
    .invalid(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Magenta))));

#[derive(Clone, Parser)]
#[clap(name = "image_to_console")]
#[command(color = clap::ColorChoice::Auto, styles = CLAP_STYLING)]
pub struct Cli {
    /// Center the image
    #[clap(short = 'c', long, default_value_t = false)]
    pub center: bool,

    /// Clear the screen
    #[clap(long, default_value_t = false)]
    pub clear: bool,

    /// Pause at the edn
    #[clap(long, default_value_t = false)]
    pub pause: bool,

    /// Show the consuming time at the bottom
    #[clap(short, long, default_value_t = false)]
    pub show_time: bool,

    /// Output file path
    #[clap(short, long)]
    pub output: Option<String>,

    /// Operates at half resolution
    #[clap(long, default_value_t = false)]
    pub half_resolution: bool,

    /// Disable the print
    #[clap(long, default_value_t = false)]
    pub disable_print: bool,

    /// Disable display info
    #[clap(long, default_value_t = false)]
    pub disable_info: bool,

    /// Convert the image to grayscale
    #[clap(long, default_value_t = false)]
    pub no_color: bool,

    /// Black background (Only run in no-color mode)
    #[clap(short, long, default_value_t = false)]
    pub black_background: bool,

    /// Disable resize (Only run in auto mode)
    #[clap(short, long, default_value_t = false)]
    pub no_resize: bool,

    /// Protocol to use
    #[clap(short, long, default_value = "auto")]
    pub protocol: Protocol,

    /// Set image resize mode
    #[clap(short, long, default_value = "auto")]
    pub resize_mode: ClapResizeMode,

    /// Set image width (Only run in custom mode)
    #[clap(long)]
    pub width: Option<u32>,

    /// Set image height (Only run in custom mode)
    #[clap(long)]
    pub height: Option<u32>,

    /// Without resize the width (Only run in auto mode)
    #[clap(long, default_value_t = false)]
    pub without_resize_width: bool,

    /// Without resize the height (Only run in auto mode)
    #[clap(short, long, default_value_t = false)]
    pub without_resize_height: bool,

    #[cfg(feature = "sixel_support")]
    /// Max colors (Only run in sixel protocol)
    #[clap(long, default_value = "256", value_parser = clap::value_parser!(u16).range(1..=256))]
    pub max_colors: u16,

    /// Enable compression (Only run in normal protocol)
    #[clap(long, default_value_t = false)]
    pub enable_compression: bool,

    #[cfg(feature = "sixel_support")]
    /// Disable dither (Only run in normal protocol)
    #[clap(long, default_value_t = false)]
    pub disable_dither: bool,

    #[cfg(feature = "sixel_support")]
    /// Set color space
    #[clap(long, default_value = "srgb")]
    pub color_space: crate::types::ColorSpace,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Clone, Subcommand)]
pub enum Commands {
    /// Load an image from a file
    File(FileArgs),

    /// Load an image from input bytes
    Bytes,

    /// Load an image from a base64
    Base64(Base64Args),

    /// Load all the images from a directory
    Directory(DirectoryArgs),

    #[cfg(feature = "url")]
    /// Load an image from a url
    Url(UrlArgs),

    #[cfg(feature = "gif_player")]
    /// Load a gif from a file
    Gif(GifArgs),

    #[cfg(feature = "video_player")]
    /// Load a video from a file
    Video(VideoArgs),

    #[cfg(feature = "dot_file")]
    /// dot-file command
    DotFile(DotFileArgs),
}

#[derive(Clone, Parser)]
pub struct FileArgs {
    /// Hide the filename in the bottom
    #[clap(long, default_value_t = false)]
    pub hide_filename: bool,

    /// Path to the image
    pub path: String,
}

#[derive(Clone, Parser)]
pub struct DirectoryArgs {
    /// Read all images at once
    #[clap(long, default_value_t = false)]
    pub read_all: bool,

    /// Path of directory
    pub path: String,
}

#[cfg(feature = "gif_player")]
#[derive(Clone, Parser)]
pub struct GifArgs {
    /// Set the frames per second for gif playback
    #[clap(long)]
    pub fps: Option<u64>,

    /// Loop the gif playback
    #[clap(long = "loop", default_value_t = false)]
    pub loop_play: bool,

    /// Audio file path
    #[cfg(feature = "audio_support")]
    #[clap(long)]
    pub audio: Option<String>,

    /// Gif file path
    pub path: String,
}

#[derive(Clone, Parser)]
pub struct Base64Args {
    /// Base64 string
    pub base64: String,
}

#[cfg(feature = "url")]
#[derive(Clone, Parser)]
pub struct UrlArgs {
    /// Url to the image
    pub url: String,
}

#[cfg(feature = "video_player")]
#[derive(Clone, Parser)]
pub struct VideoArgs {
    /// Set flush interval
    #[clap(short = 'F', long, default_value = "1s")]
    pub flush_interval: crate::types::FlushInterval,

    /// Audio file path
    #[clap(long)]
    pub audio: Option<String>,

    /// Path to the video
    pub path: String,
}

#[cfg(feature = "dot_file")]
#[derive(Clone, Subcommand)]
pub enum DotFileSubcommands {
    /// taplo schema about
    Schema(DotFileSchemaArgs),

    /// run dot-file
    Run(DotFileRunOrCheckArgs),

    /// check dot-file
    Check(DotFileRunOrCheckArgs),
}

#[cfg(feature = "dot_file")]
#[derive(Clone, Parser)]
pub struct DotFileSchemaArgs {
    #[clap(subcommand)]
    pub command: Option<DotFileSchemaSubcommands>,
}

#[cfg(feature = "dot_file")]
#[derive(Clone, Subcommand)]
pub enum DotFileSchemaSubcommands {
    /// summon the schema file if not exists
    Init,

    /// get the schema path if exists
    Path,

    /// remove the schema file if exists
    Remove,

    /// resummon the schema file
    ReInit,

    /// print the schema to stdout
    Out,
}

#[cfg(feature = "dot_file")]
impl Default for DotFileSchemaSubcommands {
    fn default() -> Self {
        Self::ReInit
    }
}

#[cfg(feature = "dot_file")]
#[derive(Clone, Parser)]
pub struct DotFileRunOrCheckArgs {
    /// Dot file path
    pub path: String,
}

#[cfg(feature = "dot_file")]
#[derive(Clone, Parser)]
pub struct DotFileArgs {
    #[clap(subcommand)]
    pub command: DotFileSubcommands,
}

impl Commands {
    pub fn is_directory(&self) -> bool {
        matches!(self, Commands::Directory(_))
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            output: None,
            clear: false,
            pause: false,
            center: false,
            no_color: false,
            show_time: false,
            no_resize: false,
            disable_info: false,
            disable_print: false,
            half_resolution: false,
            black_background: false,
            enable_compression: false,
            protocol: Protocol::Normal,
            without_resize_width: false,
            without_resize_height: false,
            resize_mode: ClapResizeMode::Auto,
            command: Commands::File(FileArgs {
                hide_filename: false,
                path: "".to_string(),
            }),
            #[cfg(feature = "sixel_support")]
            max_colors: 256,
            #[cfg(feature = "sixel_support")]
            disable_dither: false,
            #[cfg(feature = "sixel_support")]
            color_space: Default::default(),
        }
    }
}
