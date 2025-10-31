use image::open;
use image_to_console_core::process_images;

fn main() {
    let img1 = open(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/images/flower.jpg"
    ))
        .expect("Cannot found image");
    // Process images
    let result = process_images!(img1);
    // Do something with result
    println!("{}", result.display())
}
