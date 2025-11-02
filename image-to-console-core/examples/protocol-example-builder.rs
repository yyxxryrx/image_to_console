use image::open;
use image_to_console_core::{
    DisplayMode::Ascii,
    ResizeMode,
    error::ConvertResult,
    processor::{ImageProcessor, ImageProcessorOptions},
    protocol::{DisplayModeBuilder, Protocol},
};

fn main() -> ConvertResult<()> {
    let img = open(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/images/flower.jpg"
    ))
    .expect("Cannot found image");
    // Get the Display Mode builder
    // This builder will to be Ascii Mode
    let mut builder = DisplayModeBuilder::new(Protocol::Normal);
    // Set the options
    let builder = builder.option_has_color(false).option_is_full(false);
    // Is Ascii
    let display_mode = builder.build();
    assert_eq!(display_mode, Ascii);
    let options = ImageProcessorOptions::new(display_mode, ResizeMode::default(), false);
    let mut processor = ImageProcessor::new(img, options);
    let result = processor.process()?;
    println!("{}", result.display());
    Ok(())
}
