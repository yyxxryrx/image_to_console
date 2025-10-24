use crate::{
    config::RunMode::*,
    const_value::IMAGE_EXTS,
    types::{
        ClapResizeMode,
        ImageType::{self, Image},
        Protocol,
    },
    util::CreateMDFromBool,
};
use base64::Engine;
use clap::{
    Parser, Subcommand,
    builder::{
        Styles,
        styling::{AnsiColor, Color, Style},
    },
};
#[allow(unused)]
#[cfg(any(feature = "gif_player", feature = "video_player"))]
use crossbeam_channel::{bounded, unbounded};
use image_to_console_core::{DisplayMode, ResizeMode};
#[cfg(feature = "audio_support")]
use image_to_console_renderer::audio_path::AudioPath;
use rayon::{iter::ParallelIterator, prelude::ParallelBridge};
use std::io::Read;
use std::path::Path;

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
#[command(version = "0.1.16", color = clap::ColorChoice::Auto, styles = CLAP_STYLING)]
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
    #[clap(long, help = "Audio file path")]
    pub audio: Option<String>,
    #[clap(help = "Path to the video")]
    pub path: String,
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
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Config {
    pub clear: bool,
    pub pause: bool,
    pub center: bool,
    pub no_color: bool,
    pub loop_play: bool,
    pub show_time: bool,
    pub image: ImageType,
    pub fps: Option<u64>,
    pub mode: DisplayMode,
    pub disable_info: bool,
    pub disable_print: bool,
    #[cfg(feature = "sixel_support")]
    pub disable_dither: bool,
    pub show_file_name: bool,
    pub full_resolution: bool,
    #[cfg(feature = "audio_support")]
    pub audio: AudioPath,
    pub black_background: bool,
    pub output: Option<String>,
    pub resize_mode: ResizeMode,
    pub enable_compression: bool,
    pub file_name: Option<String>,
    #[cfg(feature = "sixel_support")]
    pub max_colors: u16,
}

impl Config {
    fn from_cli(
        cli: &Cli,
        image: ImageType,
        output: Option<String>,
        file_name: Option<String>,
        show_file_name: bool,
        fps: Option<u64>,
        loop_play: bool,
        #[cfg(feature = "audio_support")] audio: AudioPath,
    ) -> Self {
        Self {
            fps,
            #[cfg(feature = "audio_support")]
            audio,
            file_name,
            loop_play,
            image,
            show_file_name,
            clear: cli.clear,
            center: cli.center,
            no_color: cli.no_color,
            show_time: cli.show_time,
            resize_mode: ResizeMode::from(cli),
            output: output.or(cli.output.clone()),
            black_background: cli.black_background,
            enable_compression: cli.enable_compression,
            pause: cli.pause && !cli.command.is_directory(),
            full_resolution: !cli.half_resolution,
            disable_info: cli.disable_info || cli.command.is_directory(),
            disable_print: cli.disable_print || cli.command.is_directory(),
            mode: DisplayMode::from_bool(!cli.half_resolution, cli.no_color, cli.protocol),
            #[cfg(feature = "sixel_support")]
            max_colors: cli.max_colors,
            #[cfg(feature = "sixel_support")]
            disable_dither: cli.disable_dither,
        }
    }
}

#[derive(Debug, Clone)]
pub enum RunMode {
    Once(Result<Config, String>),
    Multiple(Vec<Result<Config, String>>),
    #[cfg(any(feature = "video_player", feature = "gif_player"))]
    Video(Result<Config, String>),
}

#[allow(dead_code)]
impl RunMode {
    pub fn once(&self) -> Result<Config, String> {
        match self {
            Once(config) => config.clone(),
            _ => panic!("Cannot get the config in other mode"),
        }
    }
    pub fn multiple(&self) -> Vec<Result<Config, String>> {
        match self {
            Multiple(configs) => configs.clone(),
            _ => panic!("Cannot get the config in other mode"),
        }
    }
    #[cfg(any(feature = "video_player", feature = "gif_player"))]
    pub fn video(&self) -> Result<Config, String> {
        match self {
            Video(config) => config.clone(),
            _ => panic!("Cannot get the config in other mode"),
        }
    }
}

pub fn parse() -> RunMode {
    let cli = Cli::parse();
    #[cfg(any(feature = "video_player", feature = "gif_player"))]
    let cli2 = cli.clone();
    let output_base = cli.output.clone();
    match cli.command {
        Commands::File(ref args) => {
            let path = Path::new(&args.path);
            if !path.exists() {
                return Once(Err("Path is not exist".to_string()));
            }
            if !path.is_file() {
                return Once(Err("Path is not a file".to_string()));
            }
            let img = image::open(&args.path).expect("Failed to open image");
            Once(Ok(Config::from_cli(
                &cli,
                Image(img),
                None,
                Some(path.file_name().unwrap().to_string_lossy().to_string()),
                !args.hide_filename,
                None,
                false,
                #[cfg(feature = "audio_support")]
                AudioPath::None,
            )))
        }
        Commands::Directory(ref args) => {
            let path = Path::new(&args.path);
            if !path.exists() {
                return Multiple(vec![Err("Path is not exist".to_string())]);
            }
            if !path.is_dir() {
                return Multiple(vec![Err("Path is not a directory".to_string())]);
            }

            let configs = std::fs::read_dir(&args.path)
                .expect("Failed to read directory")
                .par_bridge()
                .filter_map(|entry: std::io::Result<std::fs::DirEntry>| {
                    let entry = match entry {
                        Ok(entry) => entry,
                        Err(err) => {
                            eprintln!("{}", err);
                            return None;
                        }
                    };
                    match entry.file_type() {
                        Ok(file_type) => {
                            if !file_type.is_file() {
                                return None;
                            }
                            let path = entry.path();
                            match path.extension() {
                                Some(ext) => {
                                    if !IMAGE_EXTS.contains(&ext.to_str().unwrap()) {
                                        return None;
                                    }
                                    let output = match output_base.clone() {
                                        Some(o) => {
                                            let p = Path::new(&o);
                                            if let Err(e) = std::fs::create_dir_all(p) {
                                                return Some(Err(e.to_string()));
                                            }
                                            p.join(&path.file_stem().unwrap())
                                        }
                                        None => {
                                            path.parent().unwrap().join(&path.file_stem().unwrap())
                                        }
                                    };
                                    Some(Ok(Config::from_cli(
                                        &cli,
                                        if args.read_all {
                                            Image(image::open(&path).unwrap())
                                        } else {
                                            ImageType::Path(path.to_str().unwrap().to_string())
                                        },
                                        Some(output.to_str().unwrap().to_string() + ".txt"),
                                        None,
                                        false,
                                        None,
                                        false,
                                        #[cfg(feature = "audio_support")]
                                        AudioPath::None,
                                    )))
                                }
                                None => None,
                            }
                        }
                        Err(err) => {
                            eprintln!("{}", err);
                            None
                        }
                    }
                })
                .collect();
            Multiple(configs)
        }
        #[cfg(feature = "gif_player")]
        Commands::Gif(args) => match std::fs::File::open(&args.path) {
            Ok(file) => {
                use crate::types::ImageType::Gif;
                use image::DynamicImage;
                use image_to_console_core::gif_processor::GifFrameProcessor;
                let mut decoder = gif::DecodeOptions::new();
                decoder.set_color_output(gif::ColorOutput::Indexed);
                match decoder.read_info(file) {
                    Ok(mut decoder) => {
                        let (tx, rx) = bounded::<Result<(DynamicImage, usize, u16), String>>(6);
                        std::thread::spawn(move || {
                            let mut index: usize = 0;
                            let mut gif_processor = GifFrameProcessor::new(
                                decoder.width() as u32,
                                decoder.height() as u32,
                                decoder.global_palette().map(|p| p.to_vec()),
                            );
                            loop {
                                match decoder.read_next_frame() {
                                    Ok(frame) => match frame {
                                        Some(frame) => {
                                            let img = gif_processor.process_frame(frame);
                                            tx.send(Ok((img, index, frame.delay))).unwrap();
                                            index += 1;
                                        }
                                        None => {
                                            break;
                                        }
                                    },
                                    Err(err) => tx.send(Err(err.to_string())).unwrap(),
                                }
                            }
                        });
                        Video(Ok(Config::from_cli(
                            &cli2,
                            Gif(rx),
                            None,
                            None,
                            false,
                            args.fps,
                            args.loop_play,
                            #[cfg(feature = "audio_support")]
                            args.audio
                                .map(|path| AudioPath::Custom(Path::new(&path).to_path_buf()))
                                .unwrap_or_default(),
                        )))
                    }
                    Err(err) => Once(Err(err.to_string())),
                }
            }
            Err(err) => Once(Err(err.to_string())),
        },
        Commands::Base64(ref args) => {
            match base64::engine::general_purpose::STANDARD.decode(args.base64.clone()) {
                Ok(buffer) => match image::load_from_memory(&buffer) {
                    Ok(img) => Once(Ok(Config::from_cli(
                        &cli,
                        Image(img),
                        None,
                        None,
                        false,
                        None,
                        false,
                        #[cfg(feature = "audio_support")]
                        AudioPath::None,
                    ))),
                    Err(_) => Once(Err("Failed to load image from base64".to_string())),
                },
                Err(_) => Once(Err("Invalid base64 string".to_string())),
            }
        }
        Commands::Bytes => {
            let mut buffer = Vec::new();
            match std::io::stdin().lock().read_to_end(&mut buffer) {
                Ok(_) => match image::load_from_memory(&buffer) {
                    Ok(img) => Once(Ok(Config::from_cli(
                        &cli,
                        Image(img),
                        None,
                        None,
                        false,
                        None,
                        false,
                        #[cfg(feature = "audio_support")]
                        AudioPath::None,
                    ))),
                    Err(e) => Once(Err(e.to_string())),
                },
                Err(e) => Once(Err(e.to_string())),
            }
        }
        #[cfg(feature = "reqwest")]
        Commands::Url(ref args) => {
            use indicatif::{ProgressBar, ProgressStyle};
            use reqwest::blocking::Client;
            use std::io::Write;
            println!("Downloading the image from: {}", args.url);
            let client = Client::new();
            match client.get(&args.url).send() {
                Ok(resp) => {
                    if resp.status().is_success() {
                        let type_ = resp
                            .headers()
                            .get("Content-Type")
                            .expect("Cannot get the file type!")
                            .to_str()
                            .unwrap();
                        if !(type_.starts_with("image") || type_.starts_with("binary")) {
                            return Once(Err(format!(
                                "The file is not an image! (\x1b[0;35m{}\x1b[0m)",
                                type_
                            )));
                        }
                        let total_size = resp.content_length().expect("Cannot get the file length");
                        let pd = ProgressBar::new(total_size);
                        pd.set_style(ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta} remaining)").unwrap());
                        let mut buffer = Vec::new();
                        let mut cursor = std::io::Cursor::new(&mut buffer);
                        let content = resp.bytes().unwrap();
                        pd.enable_steady_tick(std::time::Duration::from_millis(100));
                        for i in content.chunks(1024) {
                            cursor.write(i).unwrap();
                            pd.inc(i.len() as u64);
                        }
                        pd.finish_with_message("Download complete");
                        match image::load_from_memory(&buffer) {
                            Ok(img) => Once(Ok(Config::from_cli(
                                &cli,
                                Image(img),
                                None,
                                None,
                                false,
                                None,
                                false,
                                #[cfg(feature = "audio_support")]
                                AudioPath::None,
                            ))),
                            Err(e) => Once(Err(format!("Failed to load image from bytes: {}", e))),
                        }
                    } else {
                        Once(Err(format!("Bad requests({})", resp.status())))
                    }
                }
                Err(e) => Once(Err(format!("Failed to download the image: {}", e))),
            }
        }
        #[cfg(feature = "video_player")]
        Commands::Video(args) => {
            let (etx, erx) = bounded(1);

            // decode the audio and video in another thread
            std::thread::spawn(move || {
                etx.send(Ok(Starting)).unwrap();
                #[cfg(feature = "audio_support")]
                let audio_path = if args.audio.is_none() {
                    let function = || {
                        // First, extract the audio to temp folder
                        let path = Path::new(&args.path);
                        let audio_path = std::env::temp_dir().join(
                            path.file_stem().unwrap().to_str().unwrap().to_owned() + "-tmp.aac",
                        );
                        ez_ffmpeg::FfmpegContext::builder()
                            .input(args.path.as_str())
                            .output(audio_path.to_str().unwrap())
                            .build()?
                            .start()?
                            .wait()?;
                        Ok::<std::path::PathBuf, ez_ffmpeg::error::Error>(audio_path)
                    };
                    function()
                        .map(|path| AudioPath::Temp(path))
                        .unwrap_or_default()
                } else {
                    AudioPath::Custom(Path::new(&args.audio.unwrap()).to_path_buf())
                };
                // And then, extract the video
                // We will to use ffmpeg-next lib to do this
                use crate::{errors::FrameError, types::VideoEvent::*};

                use image::{DynamicImage, Rgb};

                // create frame channel

                // Open the video file
                let mut video = match video_rs::Decoder::new(Path::new(&args.path)) {
                    Ok(video) => video,
                    Err(err) => {
                        etx.send(Err(err.to_string())).unwrap();
                        return;
                    }
                };
                let (width, height) = video.size();
                let (vtx, vrx) = bounded(video.frame_rate().ceil() as usize);

                // tell the channel
                #[cfg(not(feature = "audio_support"))]
                etx.send(Ok(Initialized((vrx, video.frame_rate()))))
                    .unwrap();
                #[cfg(feature = "audio_support")]
                etx.send(Ok(Initialized((vrx, audio_path, video.frame_rate()))))
                    .unwrap();
                std::thread::spawn(move || {
                    let mut frame_counter = 0usize;
                    for frame in video.decode_iter() {
                        match frame {
                            Ok((_, frame)) => {
                                let data = frame
                                    .slice(ndarray::s![.., .., ..])
                                    .to_slice()
                                    .map(|data| data.to_vec());
                                match data {
                                    Some(data) => {
                                        let img = image::ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(
                                            width, height, data,
                                        )
                                        .map(|img| DynamicImage::from(img));
                                        match img {
                                            Some(img) => {
                                                vtx.send(Ok((img, frame_counter))).unwrap()
                                            }
                                            None => vtx
                                                .send(Err(FrameError::Other(
                                                    "Failed to create the image".to_string(),
                                                )))
                                                .unwrap(),
                                        }
                                        frame_counter += 1;
                                    }
                                    // None => {},
                                    None => vtx.send(Err(FrameError::DecodeError)).unwrap(),
                                }
                            }
                            // EOF
                            Err(
                                video_rs::error::Error::ReadExhausted
                                | video_rs::error::Error::DecodeExhausted,
                            ) => break,
                            // Other errors
                            Err(err) => vtx.send(Err(FrameError::Other(err.to_string()))).unwrap(),
                        }
                    }
                    vtx.send(Err(FrameError::EOF)).unwrap();
                })
                .join()
                .unwrap();
                etx.send(Ok(Finished)).unwrap();
            });
            Video(Ok(Config::from_cli(
                &cli2,
                ImageType::Video(erx),
                None,
                None,
                false,
                None,
                false,
                #[cfg(feature = "audio_support")]
                AudioPath::None,
            )))
        }
    }
}
