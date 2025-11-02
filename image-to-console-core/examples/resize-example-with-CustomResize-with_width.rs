use image::open;
use image_to_console_core::{
    CustomResizeOption, DisplayMode, ResizeMode,
    processor::{ImageProcessor, ImageProcessorOptions},
    error::ConvertResult,
};

fn main() -> ConvertResult<()> {
    let img = open(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/images/flower.jpg"
    ))
    .expect("Cannot found image");
    // Only resize width
    let option = ImageProcessorOptions::new(
        DisplayMode::Kitty,
        ResizeMode::Custom(CustomResizeOption::with_width(100)),
        false,
    );
    let result = ImageProcessor::new(img, option).process()?;
    println!("{}", result.display());
    Ok(())
}
