use rayon::iter::*;

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

pub struct ImageConverterOption {
    pub width: u32,
    pub height: u32,
    pub full: bool,
    pub line_init: String,
}

pub struct ImageConverter {
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
            option,
        }
    }

    pub fn convert(&self) -> Vec<String> {
        let chunk_size = std::cmp::max(1, self.option.height / num_cpus::get() as u32);
        (if self.option.full {
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
                    let c = (0..(self.option.width - 1))
                        .into_par_iter()
                        .map(move |x| {
                            if self.option.full {
                                self.full_convert(x, y)
                            } else {
                                self.unfull_convert(x, y)
                            }
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
}
