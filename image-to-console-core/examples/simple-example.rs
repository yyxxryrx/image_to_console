use image::error::ImageResult;
use image_to_console_core::processor::{ImageProcessorOptions, ImageProcessorOptionsCreate};

fn main() -> ImageResult<()> {
    let img = image::open("path/to/image.png")?;

    // Use default config and process
    let result = ImageProcessorOptions::default()
        .create_processor(img)
        .process();

    // result.lines contains the formatted terminal output
    // you also can use display method to print
    println!("{}", result.display());
    Ok(())
}
