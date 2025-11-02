use image::open;
use image_to_console_core::{
    CustomResizeOption, DisplayMode, ResizeMode,
    error::ConvertResult,
    processor::{ImageProcessorOptions, ImageProcessorOptionsCreate},
};

fn main() -> ConvertResult<()> {
    let img = open(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/images/flower.jpg"
    ))
    .expect("Cannot found image");
    // Create options with builder and chained calls
    let result = ImageProcessorOptions::new(
        DisplayMode::FullNoColor,
        ResizeMode::Custom(CustomResizeOption::new(100, 100)),
        false,
    )
    .option_black_background(true)
    // Create processor
    .create_processor(img)
    .process()?;
    println!("{}", result.display());
    Ok(())
}
