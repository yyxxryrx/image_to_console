use image::open;
use image_to_console_core::{DisplayMode, ProcessedImage, converter::*};

fn main() {
    let img = open(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/images/flower.jpg"
    ))
    .expect("Cannot found image");
    // process the image
    let img = img.resize(100, 100, image::imageops::FilterType::Nearest);
    // get options
    let options = ImageConverterOption::default()
        .mode(DisplayMode::Kitty)
        .width(img.width())
        .height(img.height())
        .get_options();
    // create converter
    let converter = ImageConverter::new(ProcessedImage::new(DisplayMode::Kitty, &img), options);
    // convert the image
    let result = converter.convert().expect("Convert image failed");
    print!("{}", result.join("\n"));
}
