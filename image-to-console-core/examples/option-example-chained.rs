use image::open;
use image_to_console_core::{
    CustomResizeOption, DisplayMode, ResizeMode, processor::ImageProcessorOptions,
};

fn main() {
    let img = open(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/images/flower.jpg"
    ))
    .expect("Cannot found image");
    // Use chained calls to create, configure, and process images
    let result = ImageProcessorOptions::default()
        .option_resize(ResizeMode::Custom(CustomResizeOption::new(100, 100)))
        .option_display_mode(DisplayMode::Ascii)
        .create_processor(img)
        .process();
    println!("{}", result.display());
}
