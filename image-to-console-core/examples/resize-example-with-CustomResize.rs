use image::open;
use image_to_console_core::{
    CustomResizeOption, DisplayMode, ResizeMode,
    processor::{ImageProcessor, ImageProcessorOptions},
};

fn main() {
    let img = open(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/images/flower.jpg"
    ))
    .expect("Cannot found image");
    // resize width and height
    let option = ImageProcessorOptions::new(
        DisplayMode::Kitty,
        ResizeMode::Custom(CustomResizeOption::new(100, 100)),
        false,
    );
    let result = ImageProcessor::new(img, option).process();
    println!("{}", result.display());
}
