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
    // Your images vector
    let arr = vec![img1, img2];
    // Create options
    let options = ImageProcessorOptions::default();
    // Process images
    let results = process_images!(@vec arr, @with_options options);
    // Do something with results
    for result in results {
        println!("{}", result?.display());
    }
    Ok(())
}
