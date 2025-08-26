use base64::Engine;
use clap::builder::styling::{AnsiColor, Color, Style};
use clap::builder::Styles;
use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::{Client, Response};
use std::io::Write;
use std::path::Path;
use crate::types::DisplayMode;

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
#[command(version = "0.1.5", color = clap::ColorChoice::Auto, styles = CLAP_STYLING)]
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
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(about = "Load an image from a file")]
    File(FileArgs),
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
    pub mode: DisplayMode,
    pub disable_info: bool,
    pub disable_print: bool,
    pub show_file_name: bool,
    pub full_resolution: bool,
    pub output: Option<String>,
    pub file_name: Option<String>,
    pub image: image::DynamicImage,
    pub without_resize_height: bool,
}

impl Config {
    #[allow(dead_code)]
    pub fn new(
        image: image::DynamicImage,
        pause: bool,
        center: bool,
        no_color: bool,
        show_time: bool,
        disable_info: bool,
        disable_print: bool,
        show_file_name: bool,
        full_resolution: bool,
        output: Option<String>,
        file_name: Option<String>,
        without_resize_height: bool,
    ) -> Self {
        Self {
            image,
            pause,
            center,
            output,
            no_color,
            show_time,
            file_name,
            disable_info,
            disable_print,
            show_file_name,
            full_resolution,
            without_resize_height,
            mode: DisplayMode::from_bool(full_resolution, no_color)
        }
    }

    pub fn parse() -> Result<Self, String> {
        let cli = Cli::parse();
        let builder = |img, file_name, show_file_name| Self {
            file_name,
            image: img,
            show_file_name,
            pause: cli.pause,
            center: cli.center,
            output: cli.output,
            no_color: cli.no_color,
            show_time: cli.show_time,
            disable_info: cli.disable_info,
            disable_print: cli.disable_print,
            without_resize_height: cli.without_resize_height,
            full_resolution: cli.full_resolution || cli.no_color,
            mode: DisplayMode::from_bool(cli.full_resolution || cli.no_color, cli.no_color),
        };
        match cli.command {
            Commands::File(args) => {
                let path = Path::new(&args.path);
                if !path.exists() {
                    return Err("Path is not exist".to_string());
                }
                if !path.is_file() {
                    return Err("Path is not a file".to_string());
                }
                let img = image::open(&args.path).expect("Failed to open image");
                Ok(builder(
                    img,
                    Some(path.file_name().unwrap().to_string_lossy().to_string()),
                    !args.hide_filename,
                ))
            }
            Commands::Base64(args) => {
                let buffer = base64::engine::general_purpose::STANDARD
                    .decode(args.base64)
                    .map_err(|_| "Invalid base64 string")?;
                let img = image::load_from_memory(&buffer)
                    .map_err(|_| "Failed to load image from bytes")?;
                Ok(builder(img, None, false))
            }
            Commands::Url(args) => {
                println!("Downloading the image from: {}", args.url);
                let client = Client::new();
                let resp: Response = client
                    .get(&args.url)
                    .send()
                    .map_err(|_| "Fail to download the image")?;
                if resp.status().is_success() {
                    let type_ = resp
                        .headers()
                        .get("Content-Type")
                        .expect("Cannot get the file type!")
                        .to_str()
                        .unwrap();
                    if !(type_.starts_with("image") || type_.starts_with("binary")) {
                        return Err(format!(
                            "The file is not an image! (\x1b[0;35m{}\x1b[0m)",
                            type_
                        ));
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
                    let img = image::load_from_memory(&buffer)
                        .map_err(|_| "Failed to load image from bytes")?;
                    Ok(builder(img, None, false))
                } else {
                    Err(format!("Bad requests({})", resp.status()))
                }
            }
        }
    }
}
