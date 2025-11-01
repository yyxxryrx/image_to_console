use image::open;
use image_to_console_core::process_images;
use image_to_console_core::processor::ImageProcessorOptions;

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
    process_images!(@vec arr, @with_options options, @var img, @for_each {
        println!("{}", img.display());
    });
    // Do something with results
    // for result in results {
    //     println!("{}", result.display());
    // }
}
