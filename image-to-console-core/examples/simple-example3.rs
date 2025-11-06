use image::error::ImageResult;
use image_to_console_core::{print, processor::ImageProcessorOptions};

fn main() -> ImageResult<()> {
    let img = image::open("path/to/image.png")?;
    // create config
    let config = ImageProcessorOptions::default();
    // show image
    print(&img, &config).expect("Show image error");
    Ok(())
}
