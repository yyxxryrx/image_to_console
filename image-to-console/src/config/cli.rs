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
#[command(version = "0.1.18", color = clap::ColorChoice::Auto, styles = CLAP_STYLING)]
pub struct Cli {
    #[clap(short = 'c', long, help = "Center the image", default_value_t = false)]
    pub center: bool,
    #[clap(long, help = "Clear the screen", default_value_t = false)]
    pub clear: bool,
    #[clap(long, help = "Pause at the end", default_value_t = false)]
    pub pause: bool,
    #[clap(
        short,
        long,
        help = "Show the consuming time at the bottom",
        default_value_t = false
    )]
    pub show_time: bool,
    #[clap(short, long, help = "Output file path")]
    pub output: Option<String>,
    #[clap(long, help = "Operates at half resolution", default_value_t = false)]
    pub half_resolution: bool,
    #[clap(long, help = "Disable the print", default_value_t = false)]
    pub disable_print: bool,
    #[clap(long, help = "Disable display info", default_value_t = false)]
    pub disable_info: bool,
    #[clap(long, help = "Convert the image to grayscale", default_value_t = false)]
    pub no_color: bool,
    #[clap(
        short,
        long,
        help = "Black background (Only run in no-color mode)",
        default_value_t = false
    )]
    pub black_background: bool,
    #[clap(
        short,
        long,
        help = "Disable resize (Only run in auto mode)",
        default_value_t = false
    )]
    pub no_resize: bool,
    #[clap(short, long, help = "Protocol to use", default_value = "auto")]
    pub protocol: Protocol,
    #[clap(short, long, help = "Set image resize mode", default_value = "auto")]
    pub resize_mode: ClapResizeMode,
    #[clap(long, help = "Set image width (Only run in custom mode)")]
    pub width: Option<u32>,
    #[clap(long, help = "Set image height (Only run in custom mode)")]
    pub height: Option<u32>,
    #[clap(
        long,
        help = "Without resize the width (Only run in auto mode)",
        default_value_t = false
    )]
    pub without_resize_width: bool,
    #[clap(
        short,
        long,
        help = "Without resize the height (Only run in auto mode)",
        default_value_t = false
    )]
    pub without_resize_height: bool,
    #[cfg(feature = "sixel_support")]
    #[clap(long, help = "Max colors (Only run in sixel protocol)", default_value = "256", value_parser = clap::value_parser!(u16).range(1..=256)
    )]
    pub max_colors: u16,
    #[clap(
        long,
        help = "Enable compression (Only run in normal protocol)",
        default_value_t = false
    )]
    pub enable_compression: bool,
    #[cfg(feature = "sixel_support")]
    #[clap(
        long,
        help = "Disable dither (Only run in normal protocol)",
        default_value_t = false
    )]
    pub disable_dither: bool,
    #[cfg(feature = "sixel_support")]
    #[clap(long, help = "", default_value = "srgb")]
    pub color_space: crate::types::ColorSpace,
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Clone, Subcommand)]
pub enum Commands {
    #[clap(about = "Load an image from a file")]
    File(FileArgs),
    #[clap(about = "Load an image from input bytes")]
    Bytes,
    #[clap(about = "Load an image from a base64")]
    Base64(Base64Args),
    #[clap(about = "Load all the images from a directory")]
    Directory(DirectoryArgs),
    #[cfg(feature = "reqwest")]
    #[clap(about = "Load an image from a url")]
    Url(UrlArgs),
    #[cfg(feature = "gif_player")]
    #[clap(about = "Load a gif from a file")]
    Gif(GifArgs),
    #[cfg(feature = "video_player")]
    #[clap(about = "Load a video from a file")]
    Video(VideoArgs),
    #[cfg(feature = "dot_file")]
    #[clap(about = "dot-file command")]
    DotFile(DotFileArgs),
}

#[derive(Clone, Parser)]
pub struct FileArgs {
    #[clap(
        long,
        help = "Hide the filename in the bottom",
        default_value_t = false
    )]
    pub hide_filename: bool,
    #[clap(help = "Path to the image")]
    pub path: String,
}

#[derive(Clone, Parser)]
pub struct DirectoryArgs {
    #[clap(long, help = "Read all images at once", default_value_t = false)]
    pub read_all: bool,
    #[clap(help = "Path of directory")]
    pub path: String,
}

#[cfg(feature = "gif_player")]
#[derive(Clone, Parser)]
pub struct GifArgs {
    #[clap(long, help = "Set the frames per second for gif playback")]
    pub fps: Option<u64>,
    #[clap(long = "loop", help = "Loop the gif playback", default_value_t = false)]
    pub loop_play: bool,
    #[cfg(feature = "audio_support")]
    #[clap(long, help = "Audio file path")]
    pub audio: Option<String>,
    #[clap(help = "Gif file path")]
    pub path: String,
}

#[derive(Clone, Parser)]
pub struct Base64Args {
    #[clap(help = "Base64 string")]
    pub base64: String,
}

#[cfg(feature = "reqwest")]
#[derive(Clone, Parser)]
pub struct UrlArgs {
    #[clap(help = "Url to the image")]
    pub url: String,
}

#[cfg(feature = "video_player")]
#[derive(Clone, Parser)]
pub struct VideoArgs {
    #[clap(short = 'F', long, help = "Set flush interval", default_value = "1s")]
    pub flush_interval: crate::types::FlushInterval,
    #[clap(long, help = "Audio file path")]
    pub audio: Option<String>,
    #[clap(help = "Path to the video")]
    pub path: String,
}

#[cfg(feature = "dot_file")]
#[derive(Clone, Subcommand)]
pub enum DotFileSubcommands {
    #[clap(about = "taplo schema about")]
    Schema(DotFileSchemaArgs),
    #[clap(about = "run dot-file")]
    Run(DotFileRunOrCheckArgs),
    #[clap(about = "check dot-file")]
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
    #[clap(about = "summon the schema file if not exists")]
    Init,
    #[clap(about = "get the schema path if exists")]
    Path,
    #[clap(about = "remove the schema file if exists")]
    Remove,
    #[clap(about = "resummon the schema file")]
    ReInit,
}

#[cfg(feature = "dot_file")]
#[derive(Clone, Parser)]
pub struct DotFileRunOrCheckArgs {
    #[arg(help = "Dot file path")]
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
