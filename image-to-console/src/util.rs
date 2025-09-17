use crate::config::{Cli, Config};
use crate::types::ImageType::{Image, Path};
use crate::types::{ClapResizeMode, Protocol};
use image_to_console_core::processor::{ImageProcessor, ImageProcessorOptions};
use image_to_console_core::{AutoResizeOption, CustomResizeOption, DisplayMode, ResizeMode};
use std::io::{Read, Write};

pub fn get_char() -> char {
    let mut buf = vec![0; 1];
    std::io::stdin().lock().read_exact(&mut buf).unwrap();
    buf[0] as char
}

pub fn get_terminal_protocol() -> Protocol {
    let term_program = std::env::var("TERM_PROGRAM")
        .unwrap_or_default()
        .to_lowercase();
    let term = std::env::var("TERM").unwrap_or_default().to_lowercase();
    if term_program.contains("wezterm") || term.contains("wezterm") {
        std::io::stdout().flush().unwrap();
        Protocol::WezTerm
    } else if term_program.contains("kitty") || term.contains("kitty") {
        Protocol::Kitty
    } else if term_program.contains("iterm")
        || term.contains("iterm")
        || std::env::var("ITERM_SESSION").is_ok()
    {
        Protocol::ITerm2
    } else {
        #[cfg(feature = "crossterm")]
        {
            use std::io::BufRead;
            crossterm::terminal::enable_raw_mode().unwrap();
            std::io::stdout().write_all(b"\x1b[>c").unwrap();
            std::io::stdout().flush().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(100));
            let (st, rt) = std::sync::mpsc::channel::<String>();
            std::thread::spawn(move || {
                let mut buffer = Vec::new();
                std::io::stdin()
                    .lock()
                    .read_until(b'c', &mut buffer)
                    .unwrap();
                st.send(String::from_utf8(buffer).unwrap()).unwrap();
            });
            let p = match rt.recv_timeout(std::time::Duration::from_millis(100)) {
                Ok(s) => {
                    let s = s.chars().skip(3).take_while(|&c| c != 'c').collect::<String>();
                    let args = s.split(";").collect::<Vec<&str>>();
                    if args.len() <= 2 {
                        Protocol::Normal
                    } else if args.last().unwrap().parse::<u8>().unwrap_or(0) & 1 == 1 {
                        Protocol::Sixel
                    } else {
                        Protocol::Normal
                    }
                }
                Err(_) => {
                    Protocol::Normal
                }
            };
            crossterm::terminal::disable_raw_mode().unwrap();
            p
        }
        #[cfg(not(feature = "crossterm"))]
        Protocol::Normal
    }
}

pub trait CreateIPFromConfig {
    fn from_config(config: &Config) -> Result<Self, String>
    where
        Self: Sized;
}

impl CreateIPFromConfig for ImageProcessor {
    fn from_config(config: &Config) -> Result<Self, String> {
        let option = ImageProcessorOptions {
            mode: config.mode,
            center: config.center,
            full: config.full_resolution,
            resize_mode: config.resize_mode,
            black_background: config.black_background,
            enable_compression: config.enable_compression,
            #[cfg(feature = "sixel_support")]
            max_colors: config.max_colors,
        };
        match config.image.clone() {
            Image(image) => Ok(Self::new(image, option)),
            Path(path) => {
                let image = image::open(path).map_err(|e| e.to_string())?;
                Ok(Self::new(image, option))
            }
            #[cfg(any(feature = "gif_player", feature = "video_player"))]
            _ => Err(String::from("cannot init")),
        }
    }
}

pub trait CreateRMFromCli {
    fn from_cli(cli: &Cli) -> Self;
}

impl CreateRMFromCli for ResizeMode {
    fn from_cli(cli: &Cli) -> Self {
        match cli.resize_mode {
            ClapResizeMode::Auto => Self::Auto(AutoResizeOption {
                width: !(cli.without_resize_width || cli.no_resize),
                height: !(cli.without_resize_height || cli.no_resize),
            }),
            ClapResizeMode::Custom => Self::Custom(CustomResizeOption {
                width: cli.width,
                height: cli.height,
            }),
            ClapResizeMode::None => Self::None,
        }
    }
}

pub trait CreateMDFromBool {
    fn from_bool(full: bool, no_color: bool, protocol: Protocol) -> Self;
}

impl CreateMDFromBool for DisplayMode {
    fn from_bool(full: bool, no_color: bool, protocol: Protocol) -> Self {
        match protocol {
            Protocol::Auto => Self::from_bool(full, no_color, get_terminal_protocol()),
            Protocol::Normal => match (full, no_color) {
                (true, true) => Self::FullNoColor,
                (true, false) => Self::FullColor,
                (false, false) => Self::HalfColor,
                (false, true) => Self::Ascii,
            },
            Protocol::WezTerm => match no_color {
                true => Self::WezTermNoColor,
                false => Self::WezTerm,
            },
            Protocol::Kitty => match no_color {
                true => Self::KittyNoColor,
                false => Self::Kitty,
            },
            Protocol::ITerm2 => match no_color {
                true => Self::Iterm2NoColor,
                false => Self::Iterm2,
            },
            #[cfg(feature = "sixel_support")]
            Protocol::Sixel => match full {
                true => Self::SixelFull,
                false => Self::SixelHalf,
            },
        }
    }
}
