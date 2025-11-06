use image_to_console_core::process_images;

fn main() {
    // Empty vec
    process_images! {
        [] => results,
    }
    assert!(results.is_empty());
}
