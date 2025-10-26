use image::open;
use image_to_console_core::{
    DisplayMode, ResizeMode,
    processor::{ImageProcessor, ImageProcessorOptions},
};

fn main() {
    let img = open(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/images/flower.jpg"
    ))
    .expect("Cannot found image");
    // No resize
    let options = ImageProcessorOptions::new(DisplayMode::Kitty, ResizeMode::None, false);
    let result = ImageProcessor::new(img, options).process();
    print!("{}", result.display());
}
