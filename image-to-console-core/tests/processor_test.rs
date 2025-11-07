#![cfg(feature = "processor")]
use image_to_console_core::{
    DisplayMode, ResizeMode,
    processor::{ImageProcessor, ImageProcessorOptions, ImageProcessorOptionsCreate},
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

#[test]
fn test_all_protocol_convert() {
    let img = image::DynamicImage::new(10, 10, image::ColorType::Rgba8);
    let modes = vec![
        DisplayMode::HalfColor,
        DisplayMode::FullColor,
        DisplayMode::Ascii,
        DisplayMode::FullNoColor,
        DisplayMode::Kitty,
        DisplayMode::KittyNoColor,
        DisplayMode::Iterm2,
        DisplayMode::Iterm2NoColor,
        DisplayMode::WezTerm,
        DisplayMode::WezTermNoColor,
        #[cfg(feature = "sixel")]
        DisplayMode::SixelFull,
        #[cfg(feature = "sixel")]
        DisplayMode::SixelHalf,
    ];
    for mode in modes {
        let result = ImageProcessorOptions::default()
            .option_display_mode(mode)
            .create_processor(img.clone())
            .process();
        assert!(result.is_ok())
    }
}
