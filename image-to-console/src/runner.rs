use crate::color::colors::TerminalColor;
use crate::color::prelude::ToColoredText;
use crate::config::Config;
use crate::display::renderer::{render};
use crate::util::CreateIPFromConfig;
use image_to_console_core::processor::ImageProcessor;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
#[allow(unused_imports)]
use rayon::prelude::*;
use std::time::Duration;

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

#[cfg(any(feature = "video_player", feature = "gif_player"))]
pub fn run_video(config: Result<Config, String>) {
    use crate::types::ImageType;
    #[allow(unused_imports)]
    use crossbeam_channel::{bounded, unbounded};
    match config {
        Ok(config) => {
            let config_clone = config.clone();
            #[allow(unused)]
            let config_clone2 = config.clone();
            match config.image {
                #[cfg(feature = "gif_player")]
                ImageType::Gif(gif) => {
                    use crate::types::Frame;
                    use crate::display::renderer::render_gif;
                    let (st, rt) = bounded::<Frame>(config_clone.fps.unwrap_or(30) as _);
                    // Process the evey frame image
                    let task = std::thread::spawn(move || {
                        for frame in gif {
                            match frame {
                                Ok((frame, index, delay)) => {
                                    let mut frame_config = config_clone.clone();
                                    frame_config.image = ImageType::Image(frame);
                                    match ImageProcessor::from_config(&frame_config) {
                                        Ok(mut image_processor) => st
                                            .send(Frame {
                                                index,
                                                delay: delay as u64,
                                                frame: image_processor.process().lines.join("\n"),
                                            })
                                            .map_err(|e| err(e.to_string()))
                                            .unwrap(),
                                        Err(e) => {
                                            err(e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    err(e);
                                }
                            }
                        }
                    });
                    render_gif(rt, config_clone2);
                    task.join().unwrap();
                }
                #[cfg(feature = "video_player")]
                ImageType::Video(video_event) => {
                    use crate::display::renderer::render_video;
                    use crate::types::VideoEvent::*;
                    use crate::errors::FrameError::*;
                    for event in video_event {
                        match event {
                            Ok(event) => match event {
                                Starting => {
                                    println!("正在初始化中...");
                                }
                                Initialized((vrx, audio_path, fps)) => {
                                    let (st, rt) = bounded(10);
                                    let config_clone = config_clone.clone();
                                    let task = std::thread::spawn(move || loop {
                                        // 使用阻塞接收确保不丢失帧
                                        match vrx.recv() {
                                            Err(_) => {
                                                // Channel disconnected
                                                break;
                                            }
                                            Ok(frame) => match frame {
                                                Ok((frame, index)) => {
                                                    let mut frame_config = config_clone.clone();
                                                    frame_config.image = ImageType::Image(frame);
                                                    match ImageProcessor::from_config(&frame_config)
                                                    {
                                                        Ok(mut image_processor) => st
                                                            .send((
                                                                image_processor
                                                                    .process()
                                                                    .lines
                                                                    .join("\n"),
                                                                index,
                                                            ))
                                                            .map_err(|e| err(e.to_string()))
                                                            .unwrap(),
                                                        Err(e) => {
                                                            err(e);
                                                        }
                                                    }
                                                }
                                                Err(EOF) => break,
                                                Err(DecodeError) => {
                                                    err("cannot decode this frame".to_string())
                                                }
                                                Err(Other(e)) => err(format!("Other decode error: {e}")),
                                            },
                                        }
                                    });
                                    let render_task = std::thread::spawn(move || {
                                        #[cfg(feature = "sixel_support")]
                                        render_video(rt, audio_path, fps, config.mode.is_sixel());
                                        #[cfg(not(feature = "sixel_support"))]
                                        render_video(rt, audio_path, fps, false);
                                    });
                                    task.join().unwrap();
                                    render_task.join().unwrap();
                                }
                                Finished => {}
                            },
                            Err(e) => err(e),
                        }
                    }
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