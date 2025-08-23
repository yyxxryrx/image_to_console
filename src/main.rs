mod util;
mod image;
mod config;
mod display;
use image::processor::ImageProcessor;
use display::renderer::render;


fn main() {
    let config = config::Config::parse();
    match config {
        Ok(config) => {
            let mut image_processor = ImageProcessor::from_config(config.clone());
            let result = image_processor.process();
            render(result, config);
        },
        Err(err_msg) => eprintln!("\x1b[31mERR\x1b[0m: {}", err_msg)
    }
}
