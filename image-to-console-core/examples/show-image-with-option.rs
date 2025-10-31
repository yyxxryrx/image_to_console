use image::open;
use image_to_console_core::{processor::ImageProcessorOptions, show_image};

fn main() {
    let img = open(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/images/flower.jpg"
    ))
    .expect("Cannot found image");
    // create option
    let option = ImageProcessorOptions::default();
    // show image with option
    show_image!(img, option);
}
