use image::open;
use image_to_console_core::{error::ConvertResult,processor::{ImageProcessor, ImageProcessorOptions}};

/// This is an example to use default option to display image
fn main() -> ConvertResult<()> {
    // Open the image file from the specified path
    let img = open(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/images/flower.jpg"
    ))
    .expect("Cannot found image");

    // Create default options for image processing
    let option = ImageProcessorOptions::default();

    // Process the image with the given options
    let result = ImageProcessor::new(img, option).process()?;

    // Display the processed image in the console
    println!("{}", result.display());
    Ok(())
}
