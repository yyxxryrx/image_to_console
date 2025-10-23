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
    // resize image to fit the terminal size
    let option = ImageProcessorOptions::new(
        DisplayMode::Kitty,
        ResizeMode::Auto(AutoResizeOption::default()),
        false,
    );
    let result = ImageProcessor::new(img, option).process();
    println!("{}", result.display());
}
