use crate::{
    DisplayMode::{self, *},
    ProcessedImage,
};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use rayon::iter::*;
use std::io::Cursor;

#[derive(Copy, Clone)]
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
    pub enable_compression: bool,
    #[cfg(feature = "sixel")]
    pub max_colors: u16,
}

pub struct ImageConverter {
    full: bool,
    img: ProcessedImage,
    pub option: ImageConverterOption,
}

impl ImageConverter {
    pub fn new(img: ProcessedImage, option: ImageConverterOption) -> Self {
        Self {
            img,
            full: option.mode.is_full(),
            option,
        }
    }

    pub fn convert(&self) -> Vec<String> {
        match self.option.mode {
            Kitty | KittyNoColor => self.kitty_convert(),
            Iterm2 | Iterm2NoColor => self.iterm2_convert(),
            WezTerm | WezTermNoColor => self.wezterm_convert(),
            #[cfg(feature = "sixel")]
            SixelHalf | SixelFull => self.sixel_convert(),
            _ => {
                let chunk_size = std::cmp::max(1, self.option.height / num_cpus::get() as u32);

                let convert_pixel = |x, y| match self.option.mode {
                    FullColor => self.full_convert(x, y, false),
                    HalfColor => self.unfull_convert(x, y, false),
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
                        .map(|x| self.full_convert_pixel(x, self.option.height - 1))
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

    fn unfull_convert(&self, x: u32, y: u32, only_color: bool) -> String {
        if let ProcessedImage::Color(rgba_img) = &self.img {
            let pixel = rgba_img.get_pixel(x, y);
            let color = PixelColor::from_channels(pixel.0);
            let cur_color = if color.a >= 128 {
                color.bg()
            } else {
                "\x1b[0m".to_string()
            };
            if only_color {
                return cur_color;
            }
            let last_color = if x > 0 && self.option.enable_compression {
                self.unfull_convert(x - 1, y, true)
            } else {
                String::new()
            };
            if last_color == cur_color {
                "  ".to_string()
            } else {
                format!("{}  ", cur_color)
            }
        } else {
            panic!("Invalid image type")
        }
    }

    fn full_convert(&self, x: u32, y: u32, only_color: bool) -> String {
        if let ProcessedImage::Both(rgba_img, luma_img) = &self.img {
            let pixel1 = rgba_img.get_pixel(x, y * 2);
            let pixel2 = rgba_img.get_pixel(x, y * 2 + 1);
            let p1 = luma_img.get_pixel(x, y * 2).0[0];
            let p2 = luma_img.get_pixel(x, y * 2 + 1).0[0];
            let pixel1_color = PixelColor::from_channels(pixel1.0);
            let pixel2_color = PixelColor::from_channels(pixel2.0);
            let cur_color = if pixel1_color.a < 128 && pixel2_color.a < 128 {
                "\x1b[0m".to_string()
            } else if pixel1_color.a < 128 {
                format!("\x1b[0m{}", pixel2_color.fg())
            } else if pixel2_color.a < 128 {
                format!("\x1b[0m{}", pixel1_color.fg())
            } else if p1 > p2 {
                format!("{}{}", pixel1_color.fg(), pixel2_color.bg())
            } else if p2 > p1 {
                format!("{}{}", pixel1_color.bg(), pixel2_color.fg())
            } else {
                format!("{}", pixel1_color.bg())
            };
            if only_color {
                return cur_color;
            }
            let last_color = if x > 0 && self.option.enable_compression {
                self.full_convert(x - 1, y, true)
            } else {
                String::new()
            };
            let cur_char = if pixel1_color.a < 128 && pixel2_color.a < 128 {
                " "
            } else if pixel1_color.a < 128 {
                "▄"
            } else if pixel2_color.a < 128 {
                "▀"
            } else if p1 > p2 {
                "▀"
            } else if p2 > p1 {
                "▄"
            } else if self.option.enable_compression {
                " "
            } else {
                "█"
            };
            if cur_color == last_color {
                cur_char.to_string()
            } else if cur_char == " " && last_color.contains(&cur_color) {
                cur_char.to_string()
            } else {
                format!("{}{}", cur_color, cur_char)
            }
        } else {
            panic!("Invalid image type")
        }
    }

    fn full_convert_pixel(&self, x: u32, y: u32) -> String {
        if let ProcessedImage::Both(rgba_img, _) = &self.img {
            let pixel = rgba_img.get_pixel(x, y);
            let color = PixelColor::from_channels(pixel.0);
            format!("{}▀", color.fg())
        } else if let ProcessedImage::NoColor(luma_img) = &self.img {
            let pixel = luma_img.get_pixel(x, y);
            if pixel.0[0] > 128 {
                "▀".to_string()
            } else {
                " ".to_string()
            }
        } else {
            panic!("Invalid image type")
        }
    }

    fn no_color_convert(&self, x: u32, y: u32) -> String {
        if let ProcessedImage::NoColor(luma_img) = &self.img {
            let pixel1 = luma_img.get_pixel(x, y * 2);
            let pixel2 = luma_img.get_pixel(x, y * 2 + 1);
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
        } else {
            panic!("Invalid image type")
        }
    }

    // Older conversion algorithms may be used in the future
    #[allow(dead_code)]
    fn no_color_convert_old(&self, x: u32, y: u32) -> String {
        if let ProcessedImage::NoColor(luma_img) = &self.img {
            let pixel1 = luma_img.get_pixel(x, y * 2);
            let pixel2 = luma_img.get_pixel(x, y * 2 + 1);
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
        } else {
            panic!("Invalid image type")
        }
    }

    fn get_image_data(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        let mut writer = Cursor::new(&mut buffer);
        if self.img.is_color() {
            self.img
                .rgba()
                .unwrap()
                .write_to(&mut writer, image::ImageFormat::Png)
                .unwrap();
        } else {
            self.img
                .luma()
                .unwrap()
                .write_to(&mut writer, image::ImageFormat::Png)
                .unwrap();
        }
        buffer
    }

    fn wezterm_convert(&self) -> Vec<String> {
        let image_data = self.get_image_data();
        // Add space to prevent misalignment
        let mut lines: Vec<String> = vec![String::from(" "); 2];
        lines[0] = format!(
            "\x1b]1337;File=size={};inline=1:{}\x1b\\",
            image_data.len(),
            STANDARD.encode(image_data)
        );
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
            line.push_str(&format!("\x1b_Gm=1;{}\x1b\\", STANDARD.encode(chunk)));
        }
        line.push_str(&format!(
            "\x1b_Gm=0;{}\x1b\\",
            STANDARD.encode(chunks.last().unwrap())
        ));
        vec![line]
    }

    fn iterm2_convert(&self) -> Vec<String> {
        let image_data = self.get_image_data();
        vec![format!(
            "\x1b]1337;File=size={};inline=1:{}\x07",
            image_data.len(),
            STANDARD.encode(image_data)
        )]
    }

    #[cfg(feature = "sixel")]
    fn sixel_convert(&self) -> Vec<String> {
        use crate::indexed_image::IndexedImage;
        use nohash_hasher::BuildNoHashHasher;
        use std::collections::HashMap;

        // Some tool functions
        fn get_sixel(style: &[u8; 6]) -> String {
            let mut v = 0u8;
            for i in 0..6 {
                v |= style[i] << i;
            }
            ((v + 63) as char).to_string()
        }

        fn get_color(r: u8, g: u8, b: u8) -> String {
            format!(
                "{:.0};{:.0};{:.0}",
                r as f32 / 255f32 * 100f32,
                g as f32 / 255f32 * 100f32,
                b as f32 / 255f32 * 100f32
            )
        }

        fn render_same(
            index: Option<u8>,
            mut times: usize,
            char: &str,
            is_full: bool,
            counter: &mut Vec<usize>,
        ) -> (Option<u8>, String) {
            if !is_full {
                times *= 2;
            }
            match index {
                Some(index) => {
                    counter[index as usize] += times;
                    if times == 0 {
                        (Some(index), String::new())
                    } else if times < 3 {
                        (Some(index), char.repeat(times))
                    } else {
                        (Some(index), format!("!{}{}", times, char))
                    }
                }
                None => {
                    if times == 0 {
                        (None, String::new())
                    } else if times < 3 {
                        (None, char.repeat(times))
                    } else {
                        (None, format!("!{}{}", times, char))
                    }
                }
            }
        }

        const AIR_STYLE: &[u8; 6] = &[0u8; 6];
        let is_full = self.full;

        let img = self.img.rgb().unwrap();
        let img = IndexedImage::from_image(img, self.option.max_colors).unwrap();
        let mut result = String::from(if self.full { "\x1bP9;1q" } else { "\x1bPq" });
        let palette_count = img.palette.len();
        let (width, height) = (img.width, img.height);
        let index_counter = vec![0usize; palette_count];
        let ptr = std::sync::Arc::new(std::sync::Mutex::new(index_counter));
        let pixels: Vec<(Option<u8>, String)> = (0..=height / 6)
            .into_par_iter()
            .map(|y| {
                if y * 6 >= height {
                    return vec![];
                }
                let mut line: Vec<(Option<u8>, String)> = vec![];
                let mut col: HashMap<u32, (usize, usize), BuildNoHashHasher<u32>> =
                    (0..width).map(|i| (i, (0, 0))).collect();
                let mut col_indexs: Vec<[i16; 6]> = vec![[-1; 6]; width as usize];
                let mut col_index_counter = vec![0usize; palette_count];
                while col.len() > 0 {
                    line.push((None, "$".to_string()));
                    let mut skip_count = 0;
                    let mut same_count = 0;
                    let mut same_index = 0;
                    let mut same_style = [0u8; 6];
                    (0..width).for_each(|x| {
                        if !col.contains_key(&x) {
                            if same_count > 0 {
                                line.push(render_same(
                                    Some(same_index),
                                    same_count,
                                    &get_sixel(&same_style),
                                    is_full,
                                    &mut col_index_counter,
                                ));
                                same_count = 0;
                            }
                            skip_count += 1;
                            return;
                        }
                        if skip_count > 0 {
                            line.push(render_same(
                                None,
                                skip_count,
                                &get_sixel(AIR_STYLE),
                                is_full,
                                &mut col_index_counter,
                            ));
                            skip_count = 0;
                        }
                        let y = y * 6;
                        // Get the information for this colum
                        let (mut cur_sum, mut cur_head) = col[&x];
                        let mut cur_indexs = col_indexs[x as usize];
                        // Get current color index
                        let cur_index = img.get_pixel(x, y + cur_head as u32);
                        // Update the indexs for this colum
                        cur_indexs[cur_head] = cur_index as i16;
                        // Init some variable
                        let mut style = [0u8; 6];
                        let mut is_head = true;
                        // Get the style and next head
                        for dy in cur_head as u32..6 {
                            if y + dy >= height {
                                break;
                            }
                            let index = img.get_pixel(x, y + dy);
                            if index == cur_index {
                                cur_sum += 1;
                                style[dy as usize] = 1;
                            } else if is_head && !cur_indexs.contains(&(index as i16)) {
                                // update the cur_head
                                is_head = false;
                                cur_head = dy as usize;
                            }
                        }
                        // remove it if cur_sum >= 6, else update it
                        if cur_sum >= 6 {
                            col.remove(&x);
                        } else {
                            col.insert(x, (cur_sum, cur_head));
                            col_indexs[x as usize] = cur_indexs;
                        }
                        // counter add 1 if is the same style and color index when the counter is not zero
                        if same_count > 0 && same_index == cur_index && same_style == style {
                            same_count += 1;
                        } else {
                            // This is not a simple style or color, we need write the last style and color into this line
                            // And update this color and style to the same style and color
                            if same_count > 0 {
                                line.push(render_same(
                                    Some(same_index),
                                    same_count,
                                    &get_sixel(&same_style),
                                    is_full,
                                    &mut col_index_counter,
                                ))
                            }
                            // Set the counter to 1
                            same_count = 1;
                            // update other information
                            same_index = cur_index;
                            same_style = style;
                        }
                    });
                    // maybe some data in the same counter is not written
                    // so we should check the same_count here
                    // write into this line if the counter is not zero
                    if same_count > 0 {
                        line.push(render_same(
                            Some(same_index),
                            same_count,
                            &get_sixel(&same_style),
                            is_full,
                            &mut col_index_counter,
                        ));
                    }
                    // And maybe some data in the skip_count is not written
                    // so we also should check the skip_count here
                    if skip_count > 0 {
                        line.push(render_same(
                            None,
                            skip_count,
                            &get_sixel(AIR_STYLE),
                            is_full,
                            &mut col_index_counter,
                        ));
                    }
                }
                // This line is finished
                // Goto the next line
                line.push((None, "-".to_string()));
                let mut r = ptr.lock().unwrap();
                for (index, count) in col_index_counter.iter().enumerate() {
                    r[index] += *count;
                }
                // Return this line to collect
                line
            })
            .flatten_iter()
            .into_par_iter()
            .collect::<Vec<(Option<u8>, String)>>();
        let mut index_counter = ptr
            .lock()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(index, &count)| (index, count))
            .collect::<Vec<(usize, usize)>>();
        index_counter.sort_by(|a, b| b.1.cmp(&a.1));
        let index_mapping: HashMap<usize, usize, BuildNoHashHasher<usize>> = index_counter
            .iter()
            .enumerate()
            .map(|(index, &(i, _))| (i, index))
            .collect();
        let palette = index_counter
            .iter()
            .enumerate()
            .map(|(index, &(i, _))| {
                let rgb = img.palette[i];
                format!("#{index};2;{}", get_color(rgb.red, rgb.green, rgb.blue))
            })
            .collect::<String>();
        let pixels = pixels
            .into_par_iter()
            .map(|(index, char)| match index {
                Some(index) => format!("#{}{}", index_mapping[&(index as usize)], char),
                None => char,
            })
            .collect::<String>();

        result.push_str(&palette);
        result.push_str(&pixels);
        result.push_str("\x1b\\");
        let mut lines = vec![String::from(" "); 2];
        lines[0] = result;
        lines
    }
}
