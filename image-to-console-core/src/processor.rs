use crate::converter::{ImageConverter, ImageConverterOption};
use crate::ResizeMode::{Auto, Custom, None};
use crate::{DisplayMode, ProcessedImage, ResizeMode};
use image::{imageops::FilterType, GenericImageView};

#[derive(Copy, Clone)]
pub struct ImageProcessorOptions {
    pub full: bool,
    pub center: bool,
    pub mode: DisplayMode,
    pub black_background: bool,
    pub resize_mode: ResizeMode,
    pub enable_compression: bool,
    #[cfg(feature = "sixel")]
    pub max_colors: u16,
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

    pub fn process(&mut self) -> ImageProcessorResult {
        let time = std::time::SystemTime::now();
        let mut air_line: usize = 0;
        let (mut w, mut h) = self.image.dimensions();
        let (width, height) = terminal_size::terminal_size().unwrap();
        match self.option.resize_mode {
            Auto(option) => {
                if self.option.mode.is_normal() {
                    if option.width {
                        if w > (width.0 / if self.option.full { 1 } else { 2 }) as u32 {
                            let new_img = self.image.resize(
                                (width.0 as f32 / if self.option.full { 1f32 } else { 2f32 })
                                    .round() as u32,
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
                #[cfg(feature = "sixel")]
                if self.option.mode.is_sixel() {
                    if option.width {
                        if w > width.0 as u32 * if self.option.full { 12 } else { 6 } {
                            let new_img = self.image.resize(
                                width.0 as u32 * if self.option.full { 12 } else { 6 },
                                h,
                                FilterType::Lanczos3,
                            );
                            (w, h) = new_img.dimensions();
                            self.image = new_img;
                        }
                    }
                    if option.height {
                        if h > height.0 as u32 * if self.option.full { 21 } else { 10 } {
                            let new_img = self.image.resize(
                                w,
                                height.0 as u32 * if self.option.full { 21 } else { 10 },
                                FilterType::Lanczos3,
                            );
                            (w, h) = new_img.dimensions();
                            self.image = new_img;
                        }
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
            if self.option.mode.is_normal() {
                if !self.option.full && h < height.0 as u32
                    || self.option.full && h < height.0 as u32 / 2
                {
                    for _ in 0..(height.0 / 2) as u32 - h / if self.option.full { 4 } else { 2 } {
                        air_line += 1;
                    }
                }

                if w < width.0 as u32 / if self.option.full { 1 } else { 2 } {
                    let len = (width.0 as f32 / 2f32
                        - w as f32 / if self.option.full { 2f32 } else { 1f32 })
                    .round() as usize;
                    line_init.push_str(&" ".repeat(len));
                }
            }

            if self.option.mode.is_wezterm() {
                air_line = height.0 as usize;
                let terminal_rate = width.0 as f64 / height.0 as f64 / 2f64;
                let rate = w as f64 / h as f64;
                println!("{} {}", terminal_rate, rate);
                if rate < terminal_rate {
                    let w = (height.0 as f64 * rate).floor() as u16;
                    let offset = width.0 / 2 - w;
                    line_init.push_str(&format!(
                        "\x1b[1;{}H",
                        offset - if rate > 1.0 { offset / 2 } else { 0 }
                    ));
                } else if rate == terminal_rate {
                    if width.0 > height.0 {
                        let w = height.0;
                        let offset = (width.0 - w) / 2;
                        line_init.push_str(&format!("\x1b[1;{}H", offset - offset / 2));
                    } else if width.0 < height.0 {
                        let h = width.0 / 2;
                        let offset = (height.0 * 2 - h) / 2;
                        line_init.push_str(&format!("\x1b[{};1H", offset - offset / 2));
                    }
                } else {
                    let h = (width.0 as f64 / 2f64 / rate).floor() as u16;
                    let offset = height.0 * 2 - h;
                    line_init.push_str(&format!("\x1b[{};1H", offset - offset / 2));
                }
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
                enable_compression: self.option.enable_compression,
                #[cfg(feature = "sixel")]
                max_colors: self.option.max_colors,
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
