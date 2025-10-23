use image_to_console_core::processor::{ImageProcessor, ImageProcessorOptions};
use image::error::ImageResult;

fn main() -> ImageResult<()> {
    let img = image::open("path/to/image.png")?;

    // Use default config
    let option = ImageProcessorOptions::default();

    let mut processor = ImageProcessor::new(img, option);
    let result = processor.process();
    // result.lines contains the formatted terminal output
    // you also can use display method to print
    println!("{}", result.display());
    Ok(())
}