use crate::{
    config::Config,
    image::{
        converter::{ImageConverter, ImageConverterOption},
        ProcessedImage,
    },
    types::{
        DisplayMode,
        ImageType::*,
        ResizeMode::{self, *},
    },
};
use image::{imageops::FilterType, GenericImageView};

#[derive(Copy, Clone)]
pub struct ImageProcessorOptions {
    pub full: bool,
    pub center: bool,
    pub mode: DisplayMode,
    pub black_background: bool,
    pub resize_mode: ResizeMode,
}

pub struct ImageProcessorResult {
    pub width: u32,
    pub height: u32,
    pub air_lines: usize,
    pub lines: Vec<String>,
    pub time: std::time::SystemTime,
    pub option: ImageProcessorOptions,
}

pub struct ImageProcessor {
    pub image: image::DynamicImage,
    pub option: ImageProcessorOptions,
}

impl ImageProcessor {
    pub fn new(image: image::DynamicImage, option: ImageProcessorOptions) -> Self {
        Self { image, option }
    }

    pub fn from_config(config: Config) -> Result<Self, String> {
        let option = ImageProcessorOptions {
            mode: config.mode,
            center: config.center,
            full: config.full_resolution,
            resize_mode: config.resize_mode,
            black_background: config.black_background,
        };
        match config.image {
            Image(image) => Ok(Self::new(image, option)),
            Path(path) => {
                let image = image::open(path).map_err(|e| e.to_string())?;
                Ok(Self::new(image, option))
            },
            _ => Err(String::from("cannot init"))
        }
    }

    pub fn process(&mut self) -> ImageProcessorResult {
        let time = std::time::SystemTime::now();
        let mut arr: Vec<String> = Vec::new();
        let mut air_line: usize = 0;
        let (mut w, mut h) = self.image.dimensions();
        let (width, height) = terminal_size::terminal_size().unwrap();
        match self.option.resize_mode {
            Auto(option) => {
                if option.width {
                    if w > (width.0 / if self.option.full { 1 } else { 2 }) as u32 {
                        let new_img = self.image.resize(
                            (width.0 as f32 / if self.option.full { 1f32 } else { 2f32 }).round()
                                as u32,
                            h,
                            FilterType::Lanczos3,
                        );
                        (w, h) = new_img.dimensions();
                        self.image = new_img;
                    }
                }
                if option.height {
                    if h > (height.0 * if self.option.full { 2 } else { 1 }) as u32 {
                        let new_img = self.image.resize(
                            w,
                            (height.0 * if self.option.full { 2 } else { 1 }) as u32,
                            FilterType::Lanczos3,
                        );
                        (w, h) = new_img.dimensions();
                        self.image = new_img;
                    }
                }
            }
            Custom(option) => {
                if option.width.is_some() || option.height.is_some() {
                    let width = option.width.unwrap_or(w);
                    let height = option.height.unwrap_or(h);
                    let new_img = self.image.resize_exact(width, height, FilterType::Lanczos3);
                    (w, h) = new_img.dimensions();
                    self.image = new_img;
                }
            }
            None => {}
        }
        let mut line_init = String::new();
        if self.option.center {
            if !self.option.full && h < height.0 as u32
                || self.option.full && h < height.0 as u32 / 2
            {
                for _ in 0..(height.0 / 2) as u32 - h / if self.option.full { 4 } else { 2 } {
                    arr.push(String::new());
                    air_line += 1;
                }
            }

            if !self.option.full && w < (width.0 / 2) as u32
                || self.option.full && w < width.0 as u32
            {
                let len = (width.0 as f32 / 2f32
                    - w as f32 / if self.option.full { 2f32 } else { 1f32 })
                .round() as usize;
                let mut lst = Vec::new();
                lst.resize(len, " ");
                line_init.push_str(&lst.join(""));
            }
        }
        let converter = ImageConverter::new(
            ProcessedImage::new(self.option.mode, &self.image),
            ImageConverterOption {
                width: w,
                height: h,
                line_init,
                mode: self.option.mode,
                black_background: self.option.black_background,
            },
        );
        ImageProcessorResult {
            time,
            width: w,
            height: h,
            air_lines: air_line,
            lines: converter.convert(),
            option: self.option,
        }
    }
}
