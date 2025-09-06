use crate::color::{colors::TerminalColor, prelude::ToColoredText};
use crate::config::Config;
use image_to_console_core::processor::ImageProcessorResult;
use crate::util::get_char;
use std::io::Result;
use std::io::Write;
use std::ops::Mul;
use std::time::Duration;
use std::vec::Vec;

pub fn render(result: ImageProcessorResult, config: Config) -> Result<()> {
    let output = result.lines.join("\n");
    if !config.disable_print {
        if config.mode.is_normal() {
            print!("\x1bc");
        }
        for _ in 0..result.air_lines {
            println!();
        }
        println!("{}", output);
    }
    if !config.disable_info {
        println!(
            "{}: \x1b[1m{} x {}\x1b[0m",
            "Image Size"
                .to_colored_text()
                .set_foreground_color(TerminalColor::Green),
            result.width,
            result.height
        );
        if config.show_file_name {
            if let Some(file_name) = config.file_name {
                println!(
                    "{}: {}",
                    "File Name"
                        .to_colored_text()
                        .set_foreground_color(TerminalColor::Green),
                    file_name
                        .to_colored_text()
                        .set_foreground_color(TerminalColor::LightBlue)
                );
            }
        }

        println!(
            "{}: \x1b[1mcenter={} full-resolution={} no-color={}",
            "Args"
                .to_colored_text()
                .set_foreground_color(TerminalColor::Green),
            config
                .center
                .to_string()
                .to_colored_text()
                .set_foreground_color(TerminalColor::Yellow),
            result
                .option
                .full
                .to_string()
                .to_colored_text()
                .set_foreground_color(TerminalColor::Yellow),
            config
                .no_color
                .to_string()
                .to_colored_text()
                .set_foreground_color(TerminalColor::Yellow),
        );
        if config.show_time {
            println!();
            let duration = result.time.elapsed().unwrap();
            let min = duration.as_secs() / 60;
            let sec = duration.as_secs() % 60;
            let ms = duration.as_millis() % 1000;
            println!(
                "{} {}",
                "RENDER FINISHED IN"
                    .to_colored_text()
                    .set_foreground_color(TerminalColor::Green),
                format!("{:02}:{:02}.{:03}", min, sec, ms)
                    .to_colored_text()
                    .set_foreground_color(TerminalColor::LightGreen)
            );
        }
    }
    if let Some(filename) = config.output {
        let mut file = std::fs::File::create(filename)?;
        file.write_all(output.as_ref())?;
    }
    if config.pause {
        print!("Press the 'enter' to continue...");
        std::io::stdout().flush()?;
        get_char();
    }
    Ok(())
}

#[allow(unused)]
pub fn render_video(results: Vec<(ImageProcessorResult, usize, u64)>, config: Config) {
    let frames = results
        .iter()
        .map(|result| (result.0.lines.join("\n"), result.1, result.2));
    // Load the audio if exists
    let stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    let audio = if let Some(audio_file) = config.audio {
        let file = std::io::BufReader::new(std::fs::File::open(audio_file).unwrap());
        Some(rodio::play(&stream_handle.mixer(), file).unwrap())
    } else {
        None
    };
    // Clear the screen
    print!("\x1bc");
    // calculate the delay
    let delay = config.fps.and_then(|fps| Some(100 / fps));
    let start_time = std::time::Instant::now();
    let mut other_timer = std::time::Instant::now();
    let mut play_frame = |frame: String, index: usize, mut frame_delay: u64| {
        if let Some(delay) = delay {
            frame_delay = delay;
        }
        let system_call = other_timer.elapsed().as_micros();
        // Move the cursor to the first row and column
        let time = std::time::Instant::now();
        print!("\x1b[0;0H");
        println!("{}", frame);
        let delay = frame_delay
            .mul(10_000)
            .saturating_sub(time.elapsed().as_micros() as u64);

        // .saturating_sub(
        //     frame_delay
        //         .pow(2)
        //         .mul(3)
        //         .add(900)
        //         .saturating_sub(frame_delay * 112),
        // );
        // .saturating_sub(900u64.saturating_sub(frame_delay * 100));
        println!(
            "\x1b[2K\rframe rate: {:.2} fps",
            1_000_000f64 / delay as f64
        );
        println!("current frame: {index}");
        std::thread::sleep(Duration::from_micros(delay.saturating_sub(
            system_call as u64 * 10u64.saturating_sub(frame_delay).max(1),
        )));
        other_timer = std::time::Instant::now();
    };
    if config.loop_play {
        for (frame, index, delay) in frames.cycle() {
            play_frame(frame, index, delay);
        }
    } else {
        for (frame, index, delay) in frames {
            play_frame(frame, index, delay);
        }
    }
    println!(
        "{} {}",
        "Render in"
            .to_colored_text()
            .set_foreground_color(TerminalColor::Green),
        format!(
            "{:02}:{:02}.{:03}",
            start_time.elapsed().as_secs() / 60,
            start_time.elapsed().as_secs() % 60,
            start_time.elapsed().as_millis() % 1000
        )
        .to_colored_text()
        .set_foreground_color(TerminalColor::LightGreen)
    );
    // quit the audio stream
    std::mem::forget(stream_handle);
}
