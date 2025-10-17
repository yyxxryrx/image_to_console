use image_to_console_core::{
    converter::{ImageConverter, ImageConverterOption},
    DisplayMode, ProcessedImage
};

#[test]
fn test_image_converter_creation() {
    let img = ProcessedImage::NoColor(image::GrayImage::new(10, 10));
    let options = ImageConverterOption {
        width: 10,
        height: 10,
        line_init: String::new(),
        mode: DisplayMode::Ascii,
        black_background: false,
        enable_compression: false,
        #[cfg(feature = "sixel")]
        dither: false,
        #[cfg(feature = "sixel")]
        max_colors: 256,
    };
    
    let converter = ImageConverter::new(img, options);
    assert_eq!(converter.option.mode, DisplayMode::Ascii);
    assert_eq!(converter.option.width, 10);
    assert_eq!(converter.option.height, 10);
}