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
            if let Err(e) = render(result, config) {
                eprintln!(
                    "{}: {}",
                    "error"
                        .to_colored_text()
                        .set_foreground_color(TerminalColor::Red),
                    e.to_string()
                );
                std::process::exit(e.raw_os_error().unwrap_or(1))
            }
        }
        Err(err_msg) => {
            eprintln!(
                "{}: {}",
                "error"
                    .to_colored_text()
                    .set_foreground_color(TerminalColor::Red),
                err_msg
            );
            std::process::exit(1)
        },
    }
}
