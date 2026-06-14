use crate::config::Config;
use crate::types::ImageType;
use crate::util::CreateIPFromConfig;
use crossbeam_channel::bounded;
use image::DynamicImage;
use image_to_console_colored::colors::TerminalColor;
use image_to_console_colored::prelude::ToColoredText;
use image_to_console_core::processor::{ImageProcessor, ImageProcessorResult};
use image_to_console_renderer::renderer::render;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
#[allow(unused_imports)]
use rayon::prelude::*;
use std::time::Duration;

pub fn err<E: std::fmt::Display>(err_msg: E) -> ! {
    eprintln!(
        "{}: {err_msg}",
        "error"
            .to_colored_text()
            .set_foreground_color(TerminalColor::Red),
    );
    std::process::exit(1)
}

pub fn run(config: Result<(ImageType, Config), String>) {
    match config {
        Ok((img, config)) => match ImageProcessor::from_config(img, &config) {
            Ok(mut image_processor) => {
                let Ok(result) = image_processor.process() else {
                    return;
                };
                if let Err(e) = render(
                    result,
                    image_to_console_renderer::config::Config::from(&config),
                ) {
                    eprintln!(
                        "{}: {e}",
                        "error"
                            .to_colored_text()
                            .set_foreground_color(TerminalColor::Red)
                    );
                    std::process::exit(e.raw_os_error().unwrap_or(1))
                }
            }
            Err(e) => err(e),
        },
        Err(e) => err(e),
    }
}

#[cfg(feature = "gif_player")]
fn process(
    img: DynamicImage,
    config: &Config,
) -> image_to_console_core::ConvertResult<ImageProcessorResult> {
    ImageProcessor::from_config(ImageType::Image(img), config).and_then(|mut p| p.process())
}

#[cfg(any(feature = "video_player", feature = "gif_player"))]
pub fn run_video(config: Result<(ImageType, Config), String>) {
    use crate::types::ImageType;
    #[allow(unused_imports)]
    use crossbeam_channel::{bounded, unbounded};
    match config {
        Ok((image, config)) => match image {
            #[cfg(feature = "gif_player")]
            ImageType::Gif(gif_type) => gif(gif_type, &config),
            #[cfg(feature = "video_player")]
            ImageType::Video(video_event) => video(video_event, &config),
            _ => err(String::from("cannot init")),
        },
        Err(e) => err(e),
    }
}

#[cfg(feature = "gif_player")]
fn gif(gif: crate::types::GifType, config: &Config) {
    use image_to_console_renderer::frame::Frame;
    use image_to_console_renderer::renderer::render_gif;
    let (st, rt) = bounded::<Frame>(config.fps.unwrap_or(30) as _);
    // Process the every frame image
    std::thread::scope(|s| {
        s.spawn(|| {
            for frame in gif {
                match frame {
                    Ok((frame, index, delay)) => {
                        let r = process(frame, config).map_err(err).unwrap();
                        st.send(Frame {
                            index,
                            delay: delay as u64,
                            frame: r.display().to_string(),
                        })
                        .unwrap()
                    }
                    Err(e) => {
                        err(e);
                    }
                }
            }
        });

        s.spawn(|| {
            render_gif(
                rt,
                image_to_console_renderer::config::Config::from(config.clone()),
            );
        });
    });
}

#[cfg(feature = "video_player")]
fn video(video_event: crate::types::VideoType, config: &Config) {
    use crate::errors::FrameError::*;
    use crate::types::VideoEvent::*;
    use image_to_console_renderer::renderer::render_video;
    for event in video_event {
        match event {
            Ok(event) => match event {
                Starting => {
                    println!("正在初始化中...");
                }
                Initialized(args) => {
                    #[cfg(not(feature = "audio_support"))]
                    let (vrx, fps) = args;
                    #[cfg(feature = "audio_support")]
                    let (vrx, audio_path, fps, sync_pos) = args;
                    let (st, rt) = bounded(10);
                    let flush_interval = config.flush_interval.get_frames(fps);

                    #[cfg(feature = "audio_support")]
                    let per_frame = Duration::from_secs_f32(1f32 / fps);

                    #[cfg(feature = "audio_support")]
                    let two_frame = per_frame * 2;

                    #[cfg(feature = "audio_support")]
                    let mut spare = true;

                    #[cfg(feature = "audio_support")]
                    let pos = sync_pos.clone();
                    std::thread::scope(|s| {
                        s.spawn(|| {
                            loop {
                                match vrx.recv() {
                                    Err(_) => {
                                        // Channel disconnected
                                        break;
                                    }
                                    Ok(frame) => match frame {
                                        Ok((frame, index, pts)) => {
                                            #[cfg(feature = "audio_support")]
                                            if let Some(pts) = pts {
                                                let p = Duration::from_millis(
                                                    pos.load(std::sync::atomic::Ordering::SeqCst),
                                                )
                                                .saturating_sub(pts);

                                                if p > two_frame && spare {
                                                    continue;
                                                }

                                                if p.as_millis() > 300 {
                                                    continue;
                                                }
                                            }
                                            #[cfg(feature = "audio_support")]
                                            let timer = std::time::Instant::now();
                                            let r = process(frame, config).map_err(err).unwrap();
                                            st.send((r.lines.join("\n"), index, pts)).unwrap();
                                            #[cfg(feature = "audio_support")]
                                            {
                                                spare = timer.elapsed() <= two_frame;
                                            }
                                        }
                                        Err(Eof) => {
                                            drop(st);
                                            return;
                                        }
                                        Err(DecodeError) => {
                                            err("cannot decode this frame".to_string())
                                        }
                                        Err(Other(e)) => err(format!("Other decode error: {e}")),
                                    },
                                }
                            }
                        });

                        s.spawn(|| {
                            #[cfg(all(feature = "sixel_support", feature = "audio_support"))]
                            render_video(
                                rt,
                                audio_path,
                                fps,
                                config.clear,
                                flush_interval,
                                sync_pos,
                            );
                            #[cfg(not(feature = "audio_support"))]
                            render_video(rt, fps, config.clear, flush_interval);
                        });
                    });
                }
                Finished => break,
            },
            Err(e) => err(e),
        }
    }
}

pub fn run_multiple(configs: Vec<Result<(ImageType, Config), String>>) {
    let pd = ProgressBar::new(configs.len() as u64);
    pd.set_style(ProgressStyle::default_bar().template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} {speed} ({eta} remaining)").unwrap());
    pd.enable_steady_tick(Duration::from_millis(100));
    configs.into_par_iter().for_each(|config| {
        run(config);
        pd.inc(1);
    })
}
