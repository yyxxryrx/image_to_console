use quantette::{
    AboveMaxLen,
    palette::{encoding::Srgb, rgb::Rgb},
};

/// An image represented with a limited color palette (indexed image)
/// 
/// This is primarily used for Sixel output which requires images to be quantized
/// to a limited palette.
#[derive(Clone)]
pub struct IndexedImage {
    /// The color palette for the image
    pub palette: Vec<Rgb<Srgb, u8>>,
    /// The index data mapping each pixel to a palette entry
    pub index_data: Vec<u8>,
    /// Image width in pixels
    pub width: u32,
    /// Image height in pixels
    pub height: u32,
}

impl IndexedImage {
    /// Create an indexed image from an RGB image
    /// 
    /// This function quantizes the input image to a limited color palette.
    /// 
    /// # Arguments
    /// 
    /// * `img` - The source RGB image
    /// * `max_colors` - Maximum number of colors in the resulting palette
    /// * `dither` - Whether to apply dithering during quantization
    /// 
    /// # Returns
    /// 
    /// Returns the indexed image or an error if the image is too large
    pub fn from_image(
        img: &image::RgbImage,
        max_colors: u16,
        dither: bool,
    ) -> Result<Self, AboveMaxLen<u32>> {
        let (width, height) = img.dimensions();
        let (palette, index_data) = quantette::ImagePipeline::try_from(img)?
            .palette_size(quantette::PaletteSize::from_clamped(max_colors))
            .dither(dither)
            .colorspace(quantette::ColorSpace::Srgb)
            .quantize_method(quantette::QuantizeMethod::wu())
            .indexed_palette_par();
        Ok(Self {
            palette,
            index_data,
            width,
            height,
        })
    }

    /// Get the palette index of a pixel at the specified coordinates
    /// 
    /// # Arguments
    /// 
    /// * `x` - X coordinate
    /// * `y` - Y coordinate
    /// 
    /// # Returns
    /// 
    /// Returns the palette index of the pixel at (x, y)
    pub fn get_pixel(&self, x: u32, y: u32) -> u8 {
        self.index_data[y as usize * self.width as usize + x as usize]
    }
}
