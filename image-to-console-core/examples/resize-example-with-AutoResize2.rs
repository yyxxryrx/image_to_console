use image::open;
use image_to_console_core::{
    AutoResizeOption, DisplayMode, ResizeMode,
    processor::{ImageProcessor, ImageProcessorOptions},
};

fn main() {
    let img = open(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/images/flower.jpg"
    ))
    .expect("Cannot found image");
    // Only resize with width
    let option = ImageProcessorOptions::new(
        DisplayMode::Kitty,
        ResizeMode::Auto(AutoResizeOption::only_width()),
        false,
    );
    let result = ImageProcessor::new(img, option).process();
    println!("{}", result.display());
}
