use crate::{
    config::{Cli, Config},
    types::{
        ClapResizeMode,
        ImageType::{Image, Path},
    },
};
use image_to_console_core::{
    AutoResizeOption, CustomResizeOption, ResizeMode,
    processor::{ImageProcessor, ImageProcessorOptions},
};

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
            #[cfg(feature = "sixel_support")]
            dither: !config.disable_dither,
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

impl From<&Cli> for ResizeMode {
    fn from(cli: &Cli) -> Self {
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

impl From<Config> for image_to_console_renderer::config::Config {
    fn from(config: Config) -> Self {
        Self {
            fps: config.fps,
            clear: config.clear,
            pause: config.pause,
            center: config.center,
            output: config.output,
            file_name: config.file_name,
            show_time: config.show_time,
            disable_info: config.disable_info,
            disable_print: config.disable_print,
            show_file_name: config.show_file_name,
            #[cfg(feature = "audio_support")]
            audio: config.audio,
            #[cfg(feature = "sixel_support")]
            mode: config.mode,
        }
    }
}

impl From<&Config> for image_to_console_renderer::config::Config {
    fn from(config: &Config) -> Self {
        Self {
            fps: config.fps,
            clear: config.clear,
            pause: config.pause,
            center: config.center,
            show_time: config.show_time,
            output: config.output.clone(),
            disable_info: config.disable_info,
            file_name: config.file_name.clone(),
            disable_print: config.disable_print,
            show_file_name: config.show_file_name,
            #[cfg(feature = "audio_support")]
            audio: config.audio.clone(),
            #[cfg(feature = "sixel_support")]
            mode: config.mode,
        }
    }
}
