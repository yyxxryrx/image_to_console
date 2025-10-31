use image::error::ImageResult;
use image_to_console_core::show_image;

fn main() -> ImageResult<()> {
    let img = image::open("path/to/image.png")?;
    // show image
    show_image!(img);
    Ok(())
}
