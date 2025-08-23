use crate::config::Config;
use crate::image::converter::{ImageConverter, ImageConverterOption};
use image::imageops::FilterType;

#[derive(Copy, Clone)]
pub struct ImageProcessorOptions {
    pub full: bool,
    pub center: bool,
    pub resize_height: bool,
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

    pub fn from_config(config: Config) -> Self {
        Self::new(
            config.image,
            ImageProcessorOptions {
                center: config.center,
                full: config.full_resolution,
                resize_height: !config.without_resize_height,
            },
        )
    }

    pub fn process(&mut self) -> ImageProcessorResult {
        let time = std::time::SystemTime::now();
        let mut arr: Vec<String> = Vec::new();
        let mut air_line: usize = 0;
        let mut rgba_img = self.image.to_rgba8();
        let mut luma_img = self.image.to_luma8();
        let (mut w, mut h) = rgba_img.dimensions();
        let (width, height) = terminal_size::terminal_size().unwrap();
        if w > (width.0 / if self.option.full { 1 } else { 2 }) as u32 {
            let new_img = self.image.resize(
                (width.0 as f32 / if self.option.full { 1f32 } else { 2f32 }).round() as u32,
                h,
                FilterType::Lanczos3,
            );
            rgba_img = new_img.to_rgba8();
            luma_img = new_img.to_luma8();
            (w, h) = rgba_img.dimensions();
        }
        if self.option.resize_height {
            if h > height.0 as u32 {
                let new_img = self.image.resize(w, height.0 as u32, FilterType::Lanczos3);
                rgba_img = new_img.to_rgba8();
                luma_img = new_img.to_luma8();
                (w, h) = rgba_img.dimensions();
            }
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
            rgba_img,
            luma_img,
            ImageConverterOption {
                width: w,
                height: h,
                line_init,
                full: self.option.full,
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
