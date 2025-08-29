use crate::types::DisplayMode::{self, *};
use base64::Engine;
use rayon::iter::*;
use std::io::Cursor;
use base64::engine::general_purpose::STANDARD;

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

// experimental
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
            Kitty | KittyNoColor => self.kitty_convert(),
            Iterm2 | Iterm2NoColor => self.iterm2_convert(),
            WezTerm | WezTermNoColor => self.wezterm_convert(),
            _ => {
                let chunk_size = std::cmp::max(1, self.option.height / num_cpus::get() as u32);

                let convert_pixel = |x, y| match self.option.mode {
                    FullColor => self.full_convert(x, y),
                    HalfColor => self.unfull_convert(x, y),
                    FullNoColor => self.no_color_convert(x, y),
                    _ => String::new(),
                };
                let mut lines = (if self.full {
                    0..self.option.height / 2
                } else {
                    0..self.option.height
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
                            let c = (0..self.option.width)
                                .into_par_iter()
                                .map(move |x| convert_pixel(x, y))
                                .collect::<String>();
                            line.push_str(&c);
                            if self.option.mode.is_color() {
                                line.push_str("\x1b[0m");
                            }
                            line
                        })
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<String>>();
                // Maybe the last line is not converted
                if self.full && self.option.height % 2 == 1 {
                    let mut line = self.option.line_init.clone();
                    let c = (0..self.option.width)
                        .into_par_iter()
                        .map(|x| self.unfull_convert(x, self.option.height - 1))
                        .collect::<String>();
                    line.push_str(&c);
                    if self.option.mode.is_color() {
                        line.push_str("\x1b[0m");
                    }
                    lines.push(line);
                }
                lines
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
        // Choose a pixel one by one to see if it matches the current pixel
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

    // Older conversion algorithms may be used in the future
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

    fn get_image_data(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        let mut writer = Cursor::new(&mut buffer);
        if self.option.mode.is_color() {
            self.rgba_img
                .write_to(&mut writer, image::ImageFormat::Png)
                .unwrap();
        } else {
            self.luma_img
                .write_to(&mut writer, image::ImageFormat::Png)
                .unwrap();
        }
        buffer
    }

    fn wezterm_convert(&self) -> Vec<String> {
        let image_data = self.get_image_data();
        // Add space to prevent misalignment
        let mut lines: Vec<String> = vec![String::from(" "); 2];
        lines[0] = format!("\x1b]1337;File=size={};inline=1:{}\x1b\\", image_data.len(), STANDARD.encode(image_data));
        lines
    }

    fn kitty_convert(&self) -> Vec<String> {
        /// Base64 encodes 3 raw bytes → 4 ASCII bytes.
        ///
        /// 3072 raw bytes / 3 * 4 = 4096 encoded bytes.
        ///
        /// Thus CHUNK_SIZE = 3072 keeps every Kitty payload ≤ 4096 bytes after encoding.
        ///
        /// This matches the Python example, which splits *after* encoding.
        const CHUNK_SIZE: usize = 3072;
        let image_data = self.get_image_data();
        let mut line = format!(
            "\x1b_Gm=1,a=T,f=100,s={},v={},S={};",
            self.option.width,
            self.option.height,
            image_data.len()
        );
        let mut chunks = image_data.chunks(CHUNK_SIZE);
        line.push_str(&STANDARD.encode(chunks.nth(0).unwrap()));
        line.push_str("\x1b\\");
        for chunk in chunks.clone().take(chunks.len() - 1) {
            line.push_str(&format!(
                "\x1b_Gm=1;{}\x1b\\",
                STANDARD.encode(chunk)
            ));
        }
        line.push_str(&format!(
            "\x1b_Gm=0;{}\x1b\\",
            STANDARD.encode(chunks.last().unwrap())
        ));
        vec![line]
    }

    fn iterm2_convert(&self) -> Vec<String> {
        let image_data = self.get_image_data();
        vec![format!("\x1b]1337;File=size={};inline=1:{}\x07", image_data.len(), STANDARD.encode(image_data))]
    }
}
