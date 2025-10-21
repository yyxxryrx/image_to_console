use image::open;
use image_to_console_core::{
    DisplayMode, ResizeMode,
    processor::{ImageProcessor, ImageProcessorOptions},
};

/// This is an example to show how to display an image in terminal with use kitty protocol
fn main() {
    // Open the image
    let img = open(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/images/flower.jpg"
    ))
    .expect("Cannot found image");
    // Create option
    let option = ImageProcessorOptions::new(DisplayMode::Kitty, ResizeMode::None, false);
    // Process image
    let result = ImageProcessor::new(img, option).process();
    // Display the result
    println!("{}", result.display());
}
