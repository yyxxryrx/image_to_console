use crate::util::{CreateMDFromBool, CreateRMFromCli};
use crate::{
    config::RunMode::*,
    const_value::IMAGE_EXTS,
    types::{
        ClapResizeMode,
        ImageType::{self, Gif, Image},
        Protocol,
    },
};
use base64::Engine;
use clap::{
    builder::{
        styling::{AnsiColor, Color, Style},
        Styles,
    },
    Parser, Subcommand,
};
#[allow(unused)]
use crossbeam_channel::{bounded, unbounded};
use image::DynamicImage;
use image_to_console_core::{gif_processor::GifFrameProcessor, DisplayMode, ResizeMode};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::{iter::ParallelIterator, prelude::ParallelBridge};
use reqwest::blocking::Client;
use std::{io::Write, path::Path};

pub const CLAP_STYLING: Styles = Styles::styled()
    .header(Style::new().fg_color(Some(Color::Ansi(AnsiColor::BrightGreen))))
    .usage(Style::new().fg_color(Some(Color::Ansi(AnsiColor::BrightGreen))))
    .literal(Style::new().fg_color(Some(Color::Ansi(AnsiColor::BrightCyan))))
    .placeholder(Style::new().fg_color(Some(Color::Ansi(AnsiColor::BrightBlue))))
    .error(Style::new().fg_color(Some(Color::Ansi(AnsiColor::BrightRed))))
    .valid(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Blue))))
    .invalid(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Magenta))));

#[derive(Parser)]
#[clap(name = "image_to_console")]
#[command(version = "0.1.13", color = clap::ColorChoice::Auto, styles = CLAP_STYLING)]
pub struct Cli {
    #[clap(short = 'c', long, help = "Center the image", default_value_t = false)]
    pub center: bool,
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
    #[clap(
        short,
        long,
        help = "Operates at full resolution",
        default_value_t = false
    )]
    pub full_resolution: bool,
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
    #[clap(short, long, help = "Protocol to use", default_value = "normal")]
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
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(about = "Load an image from a file")]
    File(FileArgs),
    #[clap(about = "Load all the images from a directory")]
    Directory(DirectoryArgs),
    #[clap(about = "Load a gif from a file")]
    Gif(GifArgs),
    #[clap(about = "Load an image from a base64")]
    Base64(Base64Args),
    #[clap(about = "Load an image from a url")]
    Url(UrlArgs),
    #[cfg(feature = "video")]
    #[clap(about = "Load a video from a file")]
    Video(VideoArgs),
}

#[derive(Parser)]
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

#[derive(Parser)]
pub struct DirectoryArgs {
    #[clap(long, help = "Read all images at once", default_value_t = false)]
    pub read_all: bool,
    #[clap(help = "Path of directory")]
    pub path: String,
}

#[derive(Parser)]
pub struct GifArgs {
    #[clap(long, help = "Set the frames per second for gif playback")]
    pub fps: Option<u64>,
    #[clap(long = "loop", help = "Loop the gif playback", default_value_t = false)]
    pub loop_play: bool,
    #[clap(long, help = "Audio file path")]
    pub audio: Option<String>,
    #[clap(help = "Gif file path")]
    pub path: String,
}

#[derive(Parser)]
pub struct Base64Args {
    #[clap(help = "Base64 string")]
    pub base64: String,
}

#[derive(Parser)]
pub struct UrlArgs {
    #[clap(help = "Url to the image")]
    pub url: String,
}

#[cfg(feature = "video")]
#[derive(Parser)]
pub struct VideoArgs {
    #[clap(help = "Path to the video")]
    pub path: String,
}

impl Default for Cli {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            output: None,
            pause: false,
            center: false,
            no_color: false,
            show_time: false,
            no_resize: false,
            disable_info: false,
            disable_print: false,
            full_resolution: false,
            black_background: false,
            protocol: Protocol::Normal,
            without_resize_width: false,
            without_resize_height: false,
            resize_mode: ClapResizeMode::Auto,
            command: Commands::File(FileArgs {
                hide_filename: false,
                path: "".to_string(),
            }),
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Config {
    pub fps: Option<u64>,
    pub pause: bool,
    pub center: bool,
    pub no_color: bool,
    pub loop_play: bool,
    pub show_time: bool,
    pub image: ImageType,
    pub mode: DisplayMode,
    pub disable_info: bool,
    pub disable_print: bool,
    pub show_file_name: bool,
    pub full_resolution: bool,
    pub audio: Option<String>,
    pub black_background: bool,
    pub output: Option<String>,
    pub resize_mode: ResizeMode,
    pub file_name: Option<String>,
}

#[derive(Debug, Clone)]
pub enum RunMode {
    Once(Result<Config, String>),
    Multiple(Vec<Result<Config, String>>),
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
    pub fn video(&self) -> Result<Config, String> {
        match self {
            Video(config) => config.clone(),
            _ => panic!("Cannot get the config in other mode"),
        }
    }
}

pub fn parse() -> RunMode {
    let cli = Cli::parse();
    let resize_mode = ResizeMode::from_cli(&cli);
    let output_base = cli.output.clone();
    let builder = |img, file_name, show_file_name, fps, loop_play, audio| Config {
        fps,
        audio,
        file_name,
        loop_play,
        image: img,
        resize_mode,
        show_file_name,
        pause: cli.pause,
        center: cli.center,
        output: cli.output,
        no_color: cli.no_color,
        show_time: cli.show_time,
        disable_info: cli.disable_info,
        disable_print: cli.disable_print,
        black_background: cli.black_background,
        full_resolution: cli.full_resolution || cli.no_color,
        mode: DisplayMode::from_bool(
            cli.full_resolution || cli.no_color,
            cli.no_color,
            cli.protocol,
        ),
    };
    match cli.command {
        Commands::File(args) => {
            let path = Path::new(&args.path);
            if !path.exists() {
                return Once(Err("Path is not exist".to_string()));
            }
            if !path.is_file() {
                return Once(Err("Path is not a file".to_string()));
            }
            let img = image::open(&args.path).expect("Failed to open image");
            Once(Ok(builder(
                Image(img),
                Some(path.file_name().unwrap().to_string_lossy().to_string()),
                !args.hide_filename,
                None,
                false,
                None,
            )))
        }
        Commands::Directory(args) => {
            let path = Path::new(&args.path);
            if !path.exists() {
                return Multiple(vec![Err("Path is not exist".to_string())]);
            }
            if !path.is_dir() {
                return Multiple(vec![Err("Path is not a directory".to_string())]);
            }

            let configs = std::fs::read_dir(args.path)
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
                                    Some(Ok(Config {
                                        mode: DisplayMode::from_bool(
                                            cli.full_resolution || cli.no_color,
                                            cli.no_color,
                                            cli.protocol,
                                        ),
                                        fps: None,
                                        audio: None,
                                        resize_mode,
                                        pause: false,
                                        file_name: None,
                                        loop_play: false,
                                        show_time: false,
                                        center: cli.center,
                                        disable_info: true,
                                        disable_print: true,
                                        show_file_name: false,
                                        no_color: cli.no_color,
                                        black_background: cli.black_background,
                                        full_resolution: cli.full_resolution || cli.no_color,
                                        output: Some(output.to_str().unwrap().to_string() + ".txt"),
                                        image: if args.read_all {
                                            Image(image::open(&path).unwrap())
                                        } else {
                                            ImageType::Path(path.to_str().unwrap().to_string())
                                        },
                                    }))
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
        Commands::Gif(args) => match std::fs::File::open(&args.path) {
            Ok(file) => {
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
                                decoder.global_palette().and_then(|p| Some(p.to_vec())),
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
                        Video(Ok(builder(
                            Gif(rx),
                            None,
                            false,
                            args.fps,
                            args.loop_play,
                            args.audio,
                        )))
                    }
                    Err(err) => Once(Err(err.to_string())),
                }
            }
            Err(err) => Once(Err(err.to_string())),
        },
        Commands::Base64(args) => {
            match base64::engine::general_purpose::STANDARD.decode(args.base64) {
                Ok(buffer) => match image::load_from_memory(&buffer) {
                    Ok(img) => Once(Ok(builder(Image(img), None, false, None, false, None))),
                    Err(_) => Once(Err("Failed to load image from base64".to_string())),
                },
                Err(_) => Once(Err("Invalid base64 string".to_string())),
            }
        }
        Commands::Url(args) => {
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
                            Ok(img) => {
                                Once(Ok(builder(Image(img), None, false, None, false, None)))
                            }
                            Err(e) => Once(Err(format!("Failed to load image from bytes: {}", e))),
                        }
                    } else {
                        Once(Err(format!("Bad requests({})", resp.status())))
                    }
                }
                Err(e) => Once(Err(format!("Failed to download the image: {}", e))),
            }
        }
        #[cfg(feature = "video")]
        Commands::Video(args) => {
            let (etx, erx) = bounded(1);

            // decode the audio and video in another thread
            std::thread::spawn(move || {
                etx.send(Ok(Starting)).unwrap();
                // First, extract the audio to temp folder
                let path = Path::new(&args.path);
                let audio_path = std::env::temp_dir()
                    .join(path.file_stem().unwrap().to_str().unwrap().to_owned() + "-tmp.aac");
                ez_ffmpeg::FfmpegContext::builder()
                    .input(args.path.as_str())
                    .output(audio_path.to_str().unwrap())
                    .build()
                    .unwrap()
                    .start()
                    .unwrap()
                    .wait()
                    .unwrap();
                // And then, extract the video
                // We will to use ffmpeg-next lib to do this
                use crate::types::VideoEvent::*;
                use ffmpeg_next::{codec, format, media};
                use image::Rgb;

                let mut input = match format::input(&args.path) {
                    Ok(input) => input,
                    Err(err) => {
                        etx.send(Err(format!("Failed to open the video: {}", err)))
                            .unwrap();
                        return;
                    }
                };
                // find the best video stream and record its index
                let video_stream = match input.streams().best(media::Type::Video) {
                    Some(stream) => stream,
                    None => {
                        etx.send(Err("No video stream found".to_string())).unwrap();
                        return;
                    }
                };
                let video_index = video_stream.index();

                // get the video information
                let rate = video_stream.rate();
                let video_fps = rate.0 as f32 / rate.1 as f32;

                // create channels
                let (vtx, vrx) = bounded(100);
                // create the video decoder context
                let video_context =
                    match codec::context::Context::from_parameters(video_stream.parameters()) {
                        Ok(context) => context,
                        Err(err) => {
                            etx.send(Err(err.to_string())).unwrap();
                            return;
                        }
                    };

                let mut video_decoder = video_context.decoder().video().unwrap();

                // create the video scaler (to convert the format)
                let mut video_scaler = ffmpeg_next::software::scaling::Context::get(
                    video_decoder.format(),
                    video_decoder.width(),
                    video_decoder.height(),
                    format::Pixel::RGB24,
                    video_decoder.width(),
                    video_decoder.height(),
                    ffmpeg_next::software::scaling::Flags::BILINEAR,
                )
                .unwrap();
                // create the video frame
                let mut decoded_frame = ffmpeg_next::frame::Video::empty();
                let mut rgb_frame = ffmpeg_next::frame::Video::empty();
                let mut frame_counter = 0;
                // tell the runner function that it's ready now
                etx.send(Ok(Initialized((vrx, audio_path, video_fps)))).unwrap();
                // loop through all the packets
                for (stream, packet) in input.packets() {
                    // if the stream is the video stream
                    if stream.index() == video_index {
                        // decode the frame
                        if let Err(e) = video_decoder.send_packet(&packet) {
                            vtx.send(Err(format!(
                                "Failed to send packet to video decoder: {}",
                                e
                            )))
                            .unwrap();
                            continue;
                        }
                        // convert frame data to rgb image
                        while video_decoder.receive_frame(&mut decoded_frame).is_ok() {
                            if let Err(e) = video_scaler.run(&decoded_frame, &mut rgb_frame) {
                                vtx.send(Err(format!("Failed to scale video frame: {}", e)))
                                    .unwrap();
                                continue;
                            }
                            let buffer = rgb_frame.data(0);
                            let width = rgb_frame.width();
                            let height = rgb_frame.height();
                            let img = match image::ImageBuffer::<Rgb<u8>, Vec<u8>>::from_raw(
                                width,
                                height,
                                buffer.to_vec(),
                            ) {
                                Some(img) => img,
                                None => {
                                    vtx.send(Err("Failed to convert image".to_string()))
                                        .unwrap();
                                    continue;
                                }
                            };
                            vtx.send(Ok((DynamicImage::from(img), frame_counter)))
                                .unwrap();
                            frame_counter += 1;
                        }
                    }
                }
                etx.send(Ok(Finished)).unwrap();
            });
            Video(Ok(builder(
                ImageType::Video(erx),
                None,
                false,
                None,
                false,
                None,
            )))
        }
    }
}
