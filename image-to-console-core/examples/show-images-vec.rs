use image::open;
use image_to_console_core::show_images;

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
    let images = vec![img1, img2];
    // Show images
    show_images!(@vec images);
}
