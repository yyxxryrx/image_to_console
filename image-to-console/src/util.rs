use crate::config::{Cli, Config};
use image_to_console_core::processor::{ImageProcessor, ImageProcessorOptions};
use std::io::Read;
use image_to_console_core::{AutoResizeOption, CustomResizeOption, DisplayMode, ResizeMode};
use crate::types::{ClapResizeMode, Protocol};
use crate::types::ImageType::{Image, Path};

pub fn get_char() -> char {
    let mut buf = vec![0; 1];
    std::io::stdin().lock().read_exact(&mut buf).unwrap();
    buf[0] as char
}

pub trait CreateIPFromConfig {
    fn from_config(config: &Config) -> Result<Self, String> where Self: Sized;
}

impl CreateIPFromConfig for ImageProcessor {
    fn from_config(config: &Config) -> Result<Self, String> {
        let option = ImageProcessorOptions {
            mode: config.mode,
            center: config.center,
            full: config.full_resolution,
            resize_mode: config.resize_mode,
            black_background: config.black_background,
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
