use image::open;
use image_to_console_core::{processor::ImageProcessorOptions, show_images};

fn main() {
    let img1 = open(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/images/flower.jpg"
    ))
        .expect("Cannot found image");
    let img2 = open(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/images/flower2.jpg"
    ))
        .expect("Cannot found image");
    // Create options
    let option = ImageProcessorOptions::default();
    // Show images with options
    show_images!(img1, img2, @with_options option);
}
