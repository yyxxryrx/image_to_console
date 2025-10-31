use image_to_console_core::process_images;

fn main() {
    // No image will return empty vector
    let results = process_images!();
    assert_eq!(results.is_empty(), true);
}
