use crate::color::colors::TerminalColor;
use crate::color::prelude::ToColoredText;
use crate::config::Config;
use crate::display::renderer::{render, render_video};
use image_to_console_core::processor::{ImageProcessor, ImageProcessorResult};
use crate::types::ImageType;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use std::time::Duration;
use crate::util::CreateIPFromConfig;

pub fn err(err_msg: String) {
    eprintln!(
        "{}: {}",
        "error"
            .to_colored_text()
            .set_foreground_color(TerminalColor::Red),
        err_msg
    );
    std::process::exit(1)
}

pub fn run(config: Result<Config, String>) {
    match config {
        Ok(config) => match ImageProcessor::from_config(&config) {
            Ok(mut image_processor) => {
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
            Err(e) => err(e),
        },
        Err(e) => err(e),
    }
}

pub fn run_video(config: Result<Config, String>) {
    match config {
        Ok(config) => {
            let config_clone = config.clone();
            match config.image {
                ImageType::Gif(video) => {
                    // Process the evey frame image
                    let mut frames = video
                        .iter()
                        .par_bridge()
                        .flat_map(|frame| match frame {
                            Ok((frame, index, delay)) => {
                                let mut frame_config = config_clone.clone();
                                frame_config.image = ImageType::Image(frame);
                                match ImageProcessor::from_config(&frame_config) {
                                    Ok(mut image_processor) => {
                                        print!("\rRendered {} frames", index + 1);
                                        Some((image_processor.process(), index, delay as u64))
                                    }
                                    Err(e) => {
                                        err(e);
                                        None
                                    }
                                }
                            }
                            Err(e) => {
                                err(e);
                                None
                            }
                        })
                        .collect::<Vec<(ImageProcessorResult, usize, u64)>>();
                    frames.sort_by(|a, b| a.1.cmp(&b.1));
                    render_video(&frames, config_clone);
                }
                _ => err(String::from("cannot init")),
            }
        }
        Err(e) => err(e),
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
