use crate::{
    error::{ConvertError, ConvertErrorContext, ConvertErrorContextSource, ConvertResult},
    indexed_image::IndexedImage,
};
use nohash_hasher::BuildNoHashHasher;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashMap;

const AIR_STYLE: &[u8; 6] = &[0u8; 6];

// tool enum
#[derive(Copy, Clone, Default)]
pub enum ColorIndexState {
    First(u8),
    Same(u8),
    #[default]
    None,
}

impl From<ColorIndexState> for Option<u8> {
    fn from(state: ColorIndexState) -> Self {
        match state {
            ColorIndexState::First(index) => Some(index),
            ColorIndexState::Same(_) => None,
            ColorIndexState::None => None,
        }
    }
}

impl PartialEq<u8> for ColorIndexState {
    fn eq(&self, other: &u8) -> bool {
        match self {
            ColorIndexState::First(index) => index == other,
            ColorIndexState::Same(index) => index == other,
            ColorIndexState::None => false,
        }
    }
}

impl ColorIndexState {
    pub fn update_index(&self, index: u8) -> Self {
        if *self == index {
            Self::Same(index)
        } else {
            Self::First(index)
        }
    }
}

// Some tool functions
fn get_sixel(style: &[u8; 6]) -> String {
    let mut v = 0u8;
    for (i, item) in style.iter().enumerate() {
        v |= item << i;
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
    counter: &mut [usize],
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
                (Some(index), String::from("!") + &times.to_string() + char)
            }
        }
        None => {
            if times == 0 {
                (None, String::new())
            } else if times < 3 {
                (None, char.repeat(times))
            } else {
                (None, String::from("!") + &times.to_string() + char)
            }
        }
    }
}

pub fn convert(
    img: &image::RgbImage,
    is_full: bool,
    option: &super::ImageConverterOption,
) -> ConvertResult<Vec<String>> {
    let img = IndexedImage::from_image(
        img,
        option.max_colors,
        option.dither,
        option.quantize_method,
        option.color_space,
    )
    .map_err(|err| {
        ConvertError::AboveMaxLength(
            err.0,
            ConvertErrorContext::new(ConvertErrorContextSource::SixelConvert, err.to_string())
                .with_inner(Box::new(err)),
        )
    })?;
    let mut result = String::from(if is_full { "\x1bP9;1q" } else { "\x1bPq" });
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
            let mut same_index = ColorIndexState::default();
            while !col.is_empty() {
                line.push((None, "$".to_string()));
                let mut skip_count = 0;
                let mut same_count = 0;
                let mut same_style = [0u8; 6];
                (0..width).for_each(|x| {
                    if !col.contains_key(&x) {
                        if same_count > 0 {
                            line.push(render_same(
                                same_index.into(),
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
                                same_index.into(),
                                same_count,
                                &get_sixel(&same_style),
                                is_full,
                                &mut col_index_counter,
                            ))
                        }
                        // Set the counter to 1
                        same_count = 1;
                        // update other information
                        same_index = same_index.update_index(cur_index);
                        same_style = style;
                    }
                });
                // maybe some data in the same counter is not written
                // so we should check the same_count here
                // write into this line if the counter is not zero
                if same_count > 0 {
                    line.push(render_same(
                        same_index.into(),
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
        .map_err(|err| {
            ConvertError::LockError(ConvertErrorContext::new(
                ConvertErrorContextSource::SixelConvert,
                err.to_string(),
            ))
        })?
        .iter()
        .enumerate()
        .map(|(index, &count)| (index, count))
        .collect::<Vec<(usize, usize)>>();
    index_counter.sort_by_key(|item| std::cmp::Reverse(item.1));
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
            Some(index) => format!("#{}{char}", index_mapping[&(index as usize)]),
            None => char,
        })
        .collect::<String>();

    result.push_str(&palette);
    result.push_str(&pixels);
    result.push_str("\x1b\\");
    let mut lines = vec![String::from(" "); 2];
    lines[0] = result;
    Ok(lines)
}
