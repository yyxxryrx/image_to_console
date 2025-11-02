use image::open;
use image_to_console_core::{process_images, processor::ImageProcessorOptions};

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
    // Your images vector
    let arr = vec![img1, img2];
    let options = ImageProcessorOptions::default();
    // Process images and do something with them
    // `@with_options` can was omitted
    process_images!(@vec arr, @with_options options, @var img, @for_each {
        // Do something
        println!("Processed an image");
        println!("{}", img.display());
    });
}
