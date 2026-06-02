/// Represents a no-color pixel with different display options
struct NoColorPixel {
    /// Top half character
    top: &'static str,
    /// Full character
    full: &'static str,
    /// Bottom half character
    bottom: &'static str,
    /// Whether to separate top and bottom
    sep: bool,
    /// Lower bound of intensity range
    from: usize,
    /// Upper bound of intensity range
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

pub fn luma_convert(luma_img: &image::GrayImage, x: u32, y: u32) -> String {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_color_pixel_struct() {
        let pixel = &NO_COLOR_PIXELS[0];
        assert_eq!(pixel.top, "▘");
        assert_eq!(pixel.full, "▮");
        assert_eq!(pixel.bottom, "▖");
        assert!(pixel.sep);
        assert_eq!(pixel.from, 153);
        assert_eq!(pixel.to, 204);
    }
}
