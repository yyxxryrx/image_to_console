use image::error::ImageResult;
use image_to_console_core::processor::{ImageProcessor, ImageProcessorOptions};

fn main() -> ImageResult<()> {
    let img = image::open("path/to/image.png")?;

    // Use default config
    let option = ImageProcessorOptions::default();

    let mut processor = ImageProcessor::new(img, option);
    let result = processor.process();
    // Exception handling (this is only shown, not handled, please refer to the actual use of the need)
    let result = result.expect("Process image failed");
    // result.lines contains the formatted terminal output
    // you also can use display method to print
    println!("{}", result.display());
    Ok(())
}
