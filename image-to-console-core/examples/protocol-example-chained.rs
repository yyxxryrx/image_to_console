use image::open;
use image_to_console_core::{
    processor::{ImageProcessorOptions, ImageProcessorOptionsCreate},
    protocol::Protocol,
};

fn main() {
    let img = open(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/images/flower.jpg"
    ))
    .expect("Cannot found image");

    // Use chained calls to create, configure, and process images
    let result = ImageProcessorOptions::default()
        .option_display_mode(Protocol::Normal.builder().no_colored().no_full().build())
        .create_processor(img)
        .process();
    println!("{}", result.display());
}
