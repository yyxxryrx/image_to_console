use crate::types::DisplayMode;
use base64::Engine;
use rayon::iter::*;
use std::io::Cursor;

struct PixelColor {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl PixelColor {
    fn from_channels(channels: [u8; 4]) -> Self {
        Self {
            r: channels[0],
            g: channels[1],
            b: channels[2],
            a: channels[3],
        }
    }

    fn bg(&self) -> String {
        format!("\x1b[48;2;{};{};{}m", self.r, self.g, self.b)
    }

    fn fg(&self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.r, self.g, self.b)
    }
}

struct NoColorPixel {
    top: &'static str,
    full: &'static str,
    bottom: &'static str,
    sep: bool,
    from: usize,
    to: usize,
}

// 实验得出的
const NO_COLOR_PIXELS: [NoColorPixel; 5] = [
    NoColorPixel {
        top: "▘",
        full: "▮",
        bottom: "▖",
        sep: true,
        from: 153,
        to: 204,
    },
    NoColorPixel {
        top: "",
        full: "▪",
        bottom: "",
        sep: false,
        from: 122,
        to: 204,
    },
    NoColorPixel {
        top: "",
        bottom: "",
        full: "▫",
        sep: false,
        from: 100,
        to: 204,
    },
    NoColorPixel {
        top: "",
        bottom: "",
        full: ",",
        sep: false,
        from: 75,
        to: 204,
    },
    NoColorPixel {
        top: "",
        bottom: "",
        full: ".",
        sep: false,
        from: 51,
        to: 204,
    },
];

pub struct ImageConverterOption {
    pub width: u32,
    pub height: u32,
    pub line_init: String,
    pub mode: DisplayMode,
    pub black_background: bool,
}

pub struct ImageConverter {
    full: bool,
    rgba_img: image::RgbaImage,
    luma_img: image::GrayImage,
    pub option: ImageConverterOption,
}

impl ImageConverter {
    pub fn new(
        rgba_img: image::RgbaImage,
        luma_img: image::GrayImage,
        option: ImageConverterOption,
    ) -> Self {
        Self {
            rgba_img,
            luma_img,
            full: option.mode.is_full(),
            option,
        }
    }

    pub fn convert(&self) -> Vec<String> {
        match self.option.mode {
            DisplayMode::WezTerm | DisplayMode::WezTermNoColor => self.wezterm_convert(),
            _ => {
                let chunk_size = std::cmp::max(1, self.option.height / num_cpus::get() as u32);
                (if self.full {
                    0..(self.option.height - 1) / 2
                } else {
                    0..(self.option.height - 1)
                })
                .into_par_iter()
                .chunks(chunk_size as usize)
                .flat_map(|chunk| {
                    chunk
                        .iter()
                        .map(move |&y| {
                            let mut line = self.option.line_init.clone();
                            if self.option.black_background {
                                line.push_str("\x1b[40m");
                            }
                            let c = (0..(self.option.width - 1))
                                .into_par_iter()
                                .map(move |x| match self.option.mode {
                                    DisplayMode::FullColor => self.full_convert(x, y),
                                    DisplayMode::HalfColor => self.unfull_convert(x, y),
                                    DisplayMode::FullNoColor => self.no_color_convert(x, y),
                                    _ => String::new(),
                                })
                                .collect::<String>();
                            line.push_str(&c);
                            line.push_str("\x1b[0m");
                            line
                        })
                        .collect::<Vec<String>>()
                })
                .collect()
            }
        }
    }

    fn unfull_convert(&self, x: u32, y: u32) -> String {
        let pixel = self.rgba_img.get_pixel(x, y);
        let color = PixelColor::from_channels(pixel.0);
        let mut c = if color.a >= 128 {
            color.bg()
        } else {
            "\x1b[0m".to_string()
        };
        c.push_str("  ");
        c
    }

    fn full_convert(&self, x: u32, y: u32) -> String {
        let pixel1 = self.rgba_img.get_pixel(x, y * 2);
        let pixel2 = self.rgba_img.get_pixel(x, y * 2 + 1);
        let p1 = self.luma_img.get_pixel(x, y * 2).0[0];
        let p2 = self.luma_img.get_pixel(x, y * 2 + 1).0[0];
        let pixel1_color = PixelColor::from_channels(pixel1.0);
        let pixel2_color = PixelColor::from_channels(pixel2.0);
        if pixel1_color.a < 128 && pixel2_color.a < 128 {
            return "\x1b[0m".to_string();
        }
        if pixel1_color.a < 128 {
            return format!("\x1b[0m{}▄", pixel2_color.fg());
        }
        if pixel2_color.a < 128 {
            return format!("\x1b[0m{}▀", pixel1_color.fg());
        }
        if p1 > p2 {
            format!("{}{}▀", pixel1_color.fg(), pixel2_color.bg())
        } else if p2 > p1 {
            format!("{}{}▄", pixel1_color.bg(), pixel2_color.fg())
        } else {
            format!("{}█", pixel1_color.fg())
        }
    }

    fn no_color_convert(&self, x: u32, y: u32) -> String {
        let pixel1 = self.luma_img.get_pixel(x, y * 2);
        let pixel2 = self.luma_img.get_pixel(x, y * 2 + 1);
        let p1 = pixel1.0[0] as usize;
        let p2 = pixel2.0[0] as usize;
        // 逐个选取pixel看是否可以和当前的像素匹配
        for pixel in NO_COLOR_PIXELS.iter() {
            if pixel.sep {
                if pixel.from < p1 && p1 < pixel.to && pixel.from < p2 && p2 < pixel.to {
                    return pixel.full.to_string();
                } else if pixel.from < p1 && p1 < pixel.to {
                    return pixel.top.to_string();
                } else if pixel.from < p2 && p2 < pixel.to {
                    return pixel.bottom.to_string();
                }
            } else {
                if (pixel.from < p1 || pixel.from < p2) && (p1 < pixel.to && p2 < pixel.to) {
                    return pixel.full.to_string();
                }
            }
        }
        if p1 > 128 && p2 > 128 {
            "█".to_string()
        } else if p1 > 128 {
            "▀".to_string()
        } else if p2 > 128 {
            "▄".to_string()
        } else {
            " ".to_string()
        }
    }

    // 旧的转换算法，以后可能会用到
    #[allow(dead_code)]
    fn no_color_convert_old(&self, x: u32, y: u32) -> String {
        let pixel1 = self.luma_img.get_pixel(x, y * 2);
        let pixel2 = self.luma_img.get_pixel(x, y * 2 + 1);
        let p1 = pixel1.0[0] as usize;
        let p2 = pixel2.0[0] as usize;
        if (153 < p1) && (p1 < 204) && (153 < p2) && (p2 < 204) {
            return "▮".to_string();
        } else if (153 < p1) && (p1 < 204) {
            return "▘".to_string();
        } else if (153 < p2) && (p2 < 204) {
            return "▖".to_string();
        }
        if (102 < p1 || 102 < p2) && (p1 < 204 && p2 < 204) {
            return "▪".to_string();
        }
        if (51 < p1 || 51 < p2) && (p1 < 204 && p2 < 204) {
            return ".".to_string();
        }
        if p1 > 128 && p2 > 128 {
            "█".to_string()
        } else if p1 > 128 {
            "▀".to_string()
        } else if p2 > 128 {
            "▄".to_string()
        } else {
            " ".to_string()
        }
    }

    fn wezterm_convert(&self) -> Vec<String> {
        let (image_base64, image_size) = if self.option.mode.is_color() {
            let mut buffer = Vec::new();
            let mut writer = Cursor::new(&mut buffer);
            self.rgba_img.write_to(&mut writer, image::ImageFormat::Png).unwrap();
            (
                base64::engine::general_purpose::STANDARD.encode(&buffer),
                buffer.len(),
            )
        } else {
            let mut buffer = Vec::new();
            let mut writer = Cursor::new(&mut buffer);
            self.luma_img
                .write_to(&mut writer, image::ImageFormat::Jpeg)
                .unwrap();
            (
                base64::engine::general_purpose::STANDARD.encode(&buffer),
                buffer.len(),
            )
        };
        // Add space to prevent misalignment
        let mut lines: Vec<String> = vec![String::from(" "); 2];
        lines[0] = format!("\x1b]1337;File=size={image_size};inline=1:{image_base64}\x1b\\");
        lines
    }
}
