use crate::color::{colors::TerminalColor, prelude::ToColoredText};
use crate::config::Config;
use crate::types::Frame;
use crate::util::get_char;
use crossbeam_channel::Receiver;
use image_to_console_core::processor::ImageProcessorResult;
use std::io::Result;
use std::io::Write;
use std::thread::JoinHandle;
use std::time::Duration;

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
pub fn render_video(results: Receiver<Frame>, config: Config) {
    // let mut frames = results
    //     .iter()
    //     .map(|result| {
    //         (Frame {
    //             index: result.1,
    //             frame: result.0.lines.join("\n"),
    //             delay: result.2,
    //         })
    //     })
    //     .collect();
    // Load the audio if exists
    let stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    let audio = if let Some(audio_file) = config.audio {
        let file = std::io::BufReader::new(std::fs::File::open(audio_file).unwrap());
        Some(rodio::play(&stream_handle.mixer(), file).unwrap())
    } else {
        None
    };
    // Save current cursor position
    print!("\x1b[s");
    // calculate the delay
    let delay = config.fps.and_then(|fps| Some(100 / fps));
    let start_time = std::time::Instant::now();
    let (st, rt) = crossbeam_channel::unbounded::<JoinHandle<()>>();
    fn play_frame(
        frames: Receiver<Frame>,
        delay: Option<u64>,
        frame_index: usize,
        st: crossbeam_channel::Sender<JoinHandle<()>>,
    ) {
        let frame = frames.recv();
        if frame.is_err() {
            return;
        }
        let frame = unsafe { frame.unwrap_unchecked() };
        let (frame, index, mut frame_delay) = frame.unpacking();
        if let Some(delay) = delay {
            frame_delay = delay;
        }
        // Create new thread and other works it takes about 800 µs time, so we need to subtract it.
        let d = Duration::from_micros(frame_delay * 10_000 - 800);
        let st2 = st.clone();
        let task = std::thread::spawn(move || {
            std::thread::sleep(d);
            play_frame(frames, delay, index + 1, st2);
        });
        st.send(task).unwrap();

        // Move the cursor to the first row and column
        let time = std::time::Instant::now();
        // Back to the saved position
        print!("\x1b[u");
        println!("{}", frame);
        // let delay = frame_delay
        //     .mul(10_000)
        //     .saturating_sub(time.elapsed().as_micros() as u64);

        // .saturating_sub(
        //     frame_delay
        //         .pow(2)
        //         .mul(3)
        //         .add(900)
        //         .saturating_sub(frame_delay * 112),
        // );
        // .saturating_sub(900u64.saturating_sub(frame_delay * 100));
        // println!(
        //     "\x1b[2K\rframe rate: {:.2} fps",
        //     1_000_000f64 / delay as f64
        // );
        println!("current frame: {index}");
        // std::thread::sleep(Duration::from_micros(delay.saturating_sub(
        //     system_call as u64 * 10u64.saturating_sub(frame_delay).max(1),
        // )));
    }

    play_frame(results, delay, 0, st);

    for task in rt.iter() {
        task.join().unwrap();
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
