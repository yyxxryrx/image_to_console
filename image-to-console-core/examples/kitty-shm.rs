#[cfg(target_os = "linux")]
use image::open;
#[cfg(target_os = "linux")]
use image_to_console_core::converter::kitty_shm::KittyImage;

#[cfg(target_os = "linux")]
fn main() {
    let img1 = open(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/images/flower.jpg"
    ))
    .expect("Cannot found image");

    let img = img1.to_rgb8();
    let image = KittyImage::new(String::from("/test-for-kitty-1"), &img).expect("Ok");
    let p = image.to_string();
    println!("{p}")
}

#[cfg(not(target_os = "linux"))]
fn main() {
    unreachable!("Not supported")
}
