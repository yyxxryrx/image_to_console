use image_to_console_core::{
    DisplayMode, ProcessedImage,
    converter::{ImageConverter, ImageConverterOption},
    error::ConvertError,
};

#[test]
fn test_image_converter_creation() {
    let img = ProcessedImage::NoColor(image::GrayImage::new(10, 10));
    let options = ImageConverterOption {
        center: false,
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
        ..Default::default()
    };

    let converter = ImageConverter::new(img, options);
    assert_eq!(converter.option.mode, DisplayMode::Ascii);
    assert_eq!(converter.option.width, 10);
    assert_eq!(converter.option.height, 10);
}

#[test]
fn test_image_converter_convert() {
    let img = image::DynamicImage::default();
    let options = ImageConverterOption::default()
        .mode(DisplayMode::Kitty)
        .get_options();
    let converter = ImageConverter::new(ProcessedImage::NoColor(img.to_luma8()), options.clone());
    let result = converter.convert();
    assert!(matches!(result, Err(ConvertError::UnsupportedImageType)));
    let converter = ImageConverter::new(ProcessedImage::Color(img.to_rgba8()), options.clone());
    let result = converter.convert();
    assert!(matches!(result, Err(ConvertError::ImageWriteError(_))));
    let converter = ImageConverter::new(ProcessedImage::Color(image::RgbaImage::new(10, 10)), options);
    let result = converter.convert();
    assert!(result.is_ok());
}
