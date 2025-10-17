use image_to_console_core::{
    processor::{ImageProcessor, ImageProcessorOptions},
    DisplayMode, ResizeMode
};

#[test]
fn test_image_processor_creation() {
    let img = image::DynamicImage::new_rgba8(10, 10);
    let options = ImageProcessorOptions {
        full: false,
        center: false,
        #[cfg(feature = "sixel")]
        dither: false,
        mode: DisplayMode::HalfColor,
        black_background: false,
        resize_mode: ResizeMode::default(),
        enable_compression: false,
        #[cfg(feature = "sixel")]
        max_colors: 256,
    };
    
    let processor = ImageProcessor::new(img, options);
    assert_eq!(processor.option.mode, DisplayMode::HalfColor);
}