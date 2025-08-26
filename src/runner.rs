use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};
use crate::color::colors::TerminalColor;
use crate::color::prelude::ToColoredText;
use crate::config::Config;
use crate::display::renderer::render;
use crate::image::processor::ImageProcessor;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

pub fn run(config: Result<Config, String>) {
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
        }
    }
}

pub fn run_multiple(configs: Vec<Result<Config, String>>) {
    let pd = ProgressBar::new(configs.len() as u64);
    pd.set_style(ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} {speed} ({eta} remaining)").unwrap());
    pd.enable_steady_tick(Duration::from_millis(100));
    configs.into_par_iter().for_each(|config| {
        run(config);
        pd.inc(1);
    })
}
