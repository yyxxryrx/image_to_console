use image::open;
use image_to_console_core::{
    error::ConvertResult, process_images, processor::ImageProcessorOptions,
};

fn main() -> ConvertResult<()> {
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
    let options = ImageProcessorOptions::default();
    process_images! {
        @with_options options,
        [img1, img2] => results,
    }
    for result in results {
        println!("{}", result?.display());
    }
    Ok(())
}
