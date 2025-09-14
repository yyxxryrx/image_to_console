use quantette::palette::encoding::Srgb;
use quantette::palette::rgb::Rgb;
use quantette::AboveMaxLen;

#[derive(Clone)]
pub struct IndexedImage {
    pub palette: Vec<Rgb<Srgb, u8>>,
    pub index_data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl IndexedImage {
    pub fn from_image(img: &image::RgbImage, max_colors: u16) -> Result<Self, AboveMaxLen<u32>> {
        let (width, height) = img.dimensions();
        let (palette, index_data) = quantette::ImagePipeline::try_from(img)?
            .palette_size(quantette::PaletteSize::from_clamped(max_colors))
            .dither(true)
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

    pub fn get_pixel(&self, x: u32, y: u32) -> u8 {
        self.index_data[y as usize * self.width as usize + x as usize]
    }
}
