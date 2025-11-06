use image::open;
use image_to_console_core::{error::ConvertResult, process_images};

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
    process_images! {
        // `results` is a vector and is mutable
        [img1] => @mut results,
    }
    process_images! {
        img2 => result
    }
    // push the result to the vector
    results.push(result);
    for result in results {
        // print the result
        println!("{}", result?.display());
    }
    Ok(())
}
