pub(crate) mod cli;

#[cfg(feature = "dot_file")]
mod dot_file;

use crate::{
    config::RunMode::*,
    const_value::IMAGE_EXTS,
    types::ImageType::{self, Image},
};
use base64::Engine;
use build_options::Options;
use clap::Parser;
use cli::*;
#[allow(unused)]
#[cfg(any(feature = "gif_player", feature = "video_player"))]
use crossbeam_channel::{bounded, unbounded};
use image_to_console_core::{DisplayMode, ResizeMode};
#[cfg(feature = "audio_support")]
use image_to_console_renderer::audio_path::AudioPath;
use rayon::{iter::ParallelIterator, prelude::ParallelBridge};
use std::{io::Read, path::Path};

#[allow(unused)]
#[derive(Debug, Clone, Options, Default)]
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
    #[cfg(feature = "sixel_support")]
    pub color_space: crate::types::ColorSpace,
    #[cfg(feature = "video_player")]
    pub flush_interval: crate::types::FlushInterval,
}

impl From<&Cli> for Config {
    fn from(cli: &Cli) -> Self {
        Self {
            clear: cli.clear,
            center: cli.center,
            no_color: cli.no_color,
            show_time: cli.show_time,
            output: cli.output.clone(),
            resize_mode: ResizeMode::from(cli),
            full_resolution: !cli.half_resolution,
            black_background: cli.black_background,
            enable_compression: cli.enable_compression,
            pause: cli.pause && !cli.command.is_directory(),
            disable_info: cli.disable_info || cli.command.is_directory(),
            disable_print: cli.disable_print || cli.command.is_directory(),
            mode: cli
                .protocol
                .builder()
                .option_is_full(!cli.half_resolution)
                .option_has_color(!cli.no_color)
                .build(),
            #[cfg(feature = "sixel_support")]
            max_colors: cli.max_colors,
            #[cfg(feature = "sixel_support")]
            disable_dither: cli.disable_dither,
            #[cfg(feature = "sixel_support")]
            color_space: cli.color_space,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub enum RunMode {
    Once(Result<Config, String>),
    Multiple(Vec<Result<Config, String>>),
    #[cfg(any(feature = "video_player", feature = "gif_player"))]
    Video(Result<Config, String>),
    Error(String),
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

pub fn parse2(cli: Cli) -> RunMode {
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
            let config = Config::from(&cli)
                .image(Image(img))
                .file_name(Some(
                    path.file_name().unwrap().to_string_lossy().to_string(),
                ))
                .show_time(!args.hide_filename)
                .get_options();
            Once(Ok(config))
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
                                    let img = if args.read_all {
                                        Image(image::open(&path).unwrap())
                                    } else {
                                        ImageType::Path(path.to_str().unwrap().to_string())
                                    };
                                    let config = Config::from(&cli)
                                        .image(img)
                                        .output(Some(output.to_str().unwrap().to_string() + ".txt"))
                                        .show_file_name(false)
                                        .get_options();
                                    Some(Ok(config))
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
                        #[allow(unused_mut)]
                        let mut config = Config::from(&cli2)
                            .image(Gif(rx))
                            .fps(args.fps)
                            .loop_play(args.loop_play)
                            .get_options();
                        #[cfg(feature = "audio_support")]
                        {
                            config.audio = args
                                .audio
                                .map(|path| AudioPath::Custom(Path::new(&path).to_path_buf()))
                                .unwrap_or_default();
                        }

                        Video(Ok(config))
                    }
                    Err(err) => Once(Err(err.to_string())),
                }
            }
            Err(err) => Once(Err(err.to_string())),
        },
        Commands::Base64(ref args) => {
            match base64::engine::general_purpose::STANDARD.decode(args.base64.clone()) {
                Ok(buffer) => match image::load_from_memory(&buffer) {
                    Ok(img) => Once(Ok(Config::from(&cli).image(Image(img)).get_options())),
                    Err(_) => Once(Err("Failed to load image from base64".to_string())),
                },
                Err(_) => Once(Err("Invalid base64 string".to_string())),
            }
        }
        Commands::Bytes => {
            let mut buffer = Vec::new();
            match std::io::stdin().lock().read_to_end(&mut buffer) {
                Ok(_) => match image::load_from_memory(&buffer) {
                    Ok(img) => Once(Ok(Config::from(&cli).image(Image(img)).get_options())),
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
                            Ok(img) => Once(Ok(Config::from(&cli).image(Image(img)).get_options())),
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
                        crate::util::pick_audio(path, &audio_path)?;
                        Ok::<std::path::PathBuf, ffmpeg_next::error::Error>(audio_path)
                    };
                    function()
                        .map(|path| AudioPath::Temp(path))
                        .unwrap_or_default()
                } else {
                    if args
                        .audio
                        .as_ref()
                        .map(|s| s.to_lowercase() == "none")
                        .unwrap_or_default()
                    {
                        AudioPath::None
                    } else {
                        AudioPath::Custom(Path::new(&args.audio.unwrap()).to_path_buf())
                    }
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
            Video(Ok(Config::from(&cli2)
                .image(ImageType::Video(erx))
                .flush_interval(args.flush_interval)
                .get_options()))
        }
        #[cfg(feature = "dot_file")]
        Commands::DotFile(args) => {
            use cli::DotFileSubcommands::*;

            let is_run = matches!(&args.command, Run(..));
            match &args.command {
                Schema(..) => {
                    std::process::exit(1);
                }
                Run(args) | Check(args) => {
                    let file_path = Path::new(&args.path);

                    if !file_path.is_file() {
                        return Error(format!("file \"{}\" not exists", file_path.display()));
                    }

                    let mut file = match std::fs::File::open(file_path) {
                        Ok(file) => file,
                        Err(..) => {
                            return Error(format!("cannot open file \"{}\"", file_path.display()));
                        }
                    };

                    let mut file_content = String::new();
                    if let Err(e) = file.read_to_string(&mut file_content) {
                        return Error(format!("file \"{}\" not exists ({e})", file_path.display()));
                    }
                    let content = match toml::from_str::<dot_file::DotFileContent>(&file_content) {
                        Ok(c) => c,
                        Err(e) => {
                            crate::util::show_error(e, &file_content, file_path);
                            std::process::exit(2);
                        }
                    };
                    if !is_run {
                        std::process::exit(0);
                    }
                    parse2((&content).into())
                }
            }
        }
    }
}

pub fn parse() -> RunMode {
    let cli = Cli::parse();
    parse2(cli)
}
