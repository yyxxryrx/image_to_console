use crate::config::RunMode::{Multiple, Once};
use crate::const_value::IMAGE_EXTS;
use crate::types::{DisplayMode, ImageType};
use base64::Engine;
use clap::builder::styling::{AnsiColor, Color, Style};
use clap::builder::Styles;
use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::ParallelIterator;
use rayon::prelude::ParallelBridge;
use reqwest::blocking::Client;
use std::io::Write;
use std::path::Path;
use crate::types::ImageType::*;

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
#[command(version = "0.1.6", color = clap::ColorChoice::Auto, styles = CLAP_STYLING)]
pub struct Cli {
    #[clap(short = 'c', long, help = "Center the image", default_value_t = false)]
    pub center: bool,
    #[clap(short, long, help = "Pause at the end", default_value_t = false)]
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
    #[clap(long, help = "Without resize the width", default_value_t = false)]
    pub without_resize_width: bool,
    #[clap(
        short,
        long,
        help = "Without resize the height",
        default_value_t = false
    )]
    pub without_resize_height: bool,
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
    #[clap(long, help = "Read all images at once (Only run in directory mode)", default_value_t = false)]
    pub read_all: bool,
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(about = "Load an image from a file")]
    File(FileArgs),
    #[clap(about = "Load all the images from a directory")]
    Directory(DirectoryArgs),
    #[clap(about = "Load an image from a base64")]
    Base64(Base64Args),
    #[clap(about = "Load an image from a url")]
    Url(UrlArgs),
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
    #[clap(help = "Path of directory")]
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

#[derive(Debug, Clone)]
pub struct Config {
    pub pause: bool,
    pub center: bool,
    pub no_color: bool,
    pub show_time: bool,
    pub image: ImageType,
    pub mode: DisplayMode,
    pub disable_info: bool,
    pub disable_print: bool,
    pub show_file_name: bool,
    pub full_resolution: bool,
    pub black_background: bool,
    pub output: Option<String>,
    pub file_name: Option<String>,
    pub without_resize_width: bool,
    pub without_resize_height: bool,
}

#[derive(Debug, Clone)]
pub enum RunMode {
    Once(Result<Config, String>),
    Multiple(Vec<Result<Config, String>>),
}

#[allow(dead_code)]
impl RunMode {
    pub fn is_once(&self) -> Result<Config, String> {
        match self {
            Once(config) => config.clone(),
            Multiple(_) => panic!("Cannot get the config in multiple mode"),
        }
    }
    pub fn is_multiple(&self) -> Vec<Result<Config, String>> {
        match self {
            Once(_) => panic!("Cannot get the config in once mode"),
            Multiple(configs) => configs.clone(),
        }
    }
}

pub fn parse() -> RunMode {
    let cli = Cli::parse();
    let output_base = cli.output.clone();
    let builder = |img, file_name, show_file_name| Config {
        file_name,
        image: Image(img),
        show_file_name,
        pause: cli.pause,
        center: cli.center,
        output: cli.output,
        no_color: cli.no_color,
        show_time: cli.show_time,
        disable_info: cli.disable_info,
        disable_print: cli.disable_print,
        black_background: cli.black_background,
        without_resize_width: cli.without_resize_width,
        without_resize_height: cli.without_resize_height,
        full_resolution: cli.full_resolution || cli.no_color,
        mode: DisplayMode::from_bool(cli.full_resolution || cli.no_color, cli.no_color),
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
                img,
                Some(path.file_name().unwrap().to_string_lossy().to_string()),
                !args.hide_filename,
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
                                        ),
                                        pause: false,
                                        file_name: None,
                                        show_time: false,
                                        center: cli.center,
                                        disable_info: true,
                                        disable_print: true,
                                        show_file_name: false,
                                        no_color: cli.no_color,
                                        black_background: cli.black_background,
                                        without_resize_width: cli.without_resize_width,
                                        without_resize_height: cli.without_resize_height,
                                        full_resolution: cli.full_resolution || cli.no_color,
                                        output: Some(output.to_str().unwrap().to_string() + ".txt"),
                                        image: if cli.read_all {
                                            Image(image::open(&path).unwrap())
                                        } else {
                                            Path(path.to_str().unwrap().to_string())
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
        Commands::Base64(args) => {
            match base64::engine::general_purpose::STANDARD.decode(args.base64) {
                Ok(buffer) => match image::load_from_memory(&buffer) {
                    Ok(img) => Once(Ok(builder(img, None, false))),
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
                            Ok(img) => Once(Ok(builder(img, None, false))),
                            Err(e) => Once(Err(format!("Failed to load image from bytes: {}", e))),
                        }
                    } else {
                        Once(Err(format!("Bad requests({})", resp.status())))
                    }
                }
                Err(e) => Once(Err(format!("Failed to download the image: {}", e))),
            }
        }
    }
}
