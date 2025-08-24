mod color;
mod config;
mod display;
mod image;
mod util;

use color::{colors::TerminalColor, prelude::ToColoredText};
use display::renderer::render;
use image::processor::ImageProcessor;

fn main() {
    let config = config::Config::parse();
    match config {
        Ok(config) => {
            let mut image_processor = ImageProcessor::from_config(config.clone());
            let result = image_processor.process();
            render(result, config);
        }
        Err(err_msg) => eprintln!(
            "{}: {}",
            "err"
                .to_colored_text()
                .set_foreground_color(TerminalColor::Red),
            err_msg
        ),
    }
}
