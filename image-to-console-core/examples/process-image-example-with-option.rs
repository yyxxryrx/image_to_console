use image::open;
use image_to_console_core::{process_images, processor::ImageProcessorOptions};

fn main() {
    let img1 = open(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/images/flower.jpg"
    ))
        .expect("Cannot found image");
    // Create options
    let options = ImageProcessorOptions::default();
    // Process images
    let result = process_images!(img1, @with_options options);
    // Do something with result
    println!("{}", result.display())
}
