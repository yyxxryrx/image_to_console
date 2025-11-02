#[cfg(all(
    feature = "rodio",
    any(feature = "video_player", feature = "gif_player")
))]
use crate::audio_path::AudioPath;
use crate::config::Config;
#[cfg(feature = "gif_player")]
use crate::frame::Frame;
use image_to_console_colored::{colors::TerminalColor, prelude::ToColoredText};
use image_to_console_core::processor::ImageProcessorResult;
#[cfg(any(feature = "video_player", feature = "gif_player"))]
use std::thread::JoinHandle;
use std::{
    fs::File,
    io::{Read, Result, Write},
};

pub fn get_char() -> char {
    let mut buf = vec![0; 1];
    std::io::stdin().lock().read_exact(&mut buf).unwrap();
    buf[0] as char
}

pub fn render(result: ImageProcessorResult, config: Config) -> Result<()> {
    let output = result.lines.join("\n");
    if !config.disable_print {
        if config.clear {
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
            result
                .option
                .mode
                .is_luma()
                .to_string()
                .to_colored_text()
                .set_foreground_color(TerminalColor::Yellow),
        );
        if config.show_time {
            println!();
            let duration = result.time.elapsed();
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
        let mut file = File::create(filename)?;
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
#[cfg(feature = "gif_player")]
pub fn render_gif(results: crossbeam_channel::Receiver<Frame>, config: Config) {
    // Load the audio if exists
    #[cfg(feature = "rodio")]
    let stream_handle = config.audio.get_path().map(|_| {
        rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream")
    });
    #[cfg(feature = "rodio")]
    let audio = config.audio.get_path().map(|path| {
        let file = std::io::BufReader::new(File::open(path).unwrap());
        rodio::play(stream_handle.as_ref().unwrap().mixer(), file).unwrap()
    });
    // calculate the delay
    let delay = config.fps.map(|fps| 100 / fps);
    let start_time = std::time::Instant::now();
    let (st, rt) = crossbeam_channel::unbounded::<JoinHandle<()>>();
    fn play_frame(
        frames: crossbeam_channel::Receiver<Frame>,
        delay: Option<u64>,
        frame_index: usize,
        st: crossbeam_channel::Sender<JoinHandle<()>>,
        is_sixel: bool,
        back_top: bool,
        offset: std::time::Duration,
    ) {
        let frame = frames.recv();
        if frame.is_err() {
            return;
        }
        let frame = frame.unwrap();
        let (frame, index, mut frame_delay) = frame.unpacking();
        if let Some(delay) = delay {
            frame_delay = delay;
        }
        let d = std::time::Duration::from_micros(frame_delay * 10_000).saturating_sub(offset);
        let st2 = st.clone();
        // create a new timer
        let timer = std::time::Instant::now();
        let task = std::thread::spawn(move || {
            std::thread::sleep(d);
            // calculate the time
            let time = timer.elapsed();
            play_frame(frames, delay, index + 1, st2, is_sixel, back_top, time - d);
        });
        st.send(task).unwrap();

        let time = std::time::Instant::now();
        if back_top {
            print!("\x1b[1;1H");
        } else {
            // Save current cursor position
            print!("\r\x1b[s");
        }
        println!("{}", frame);
        println!("Current frame: {index}");
        if !back_top {
            // Back to the saved position
            print!("\x1b[u");
        }
    }
    if config.clear {
        print!("\x1bc");
    }
    #[cfg(feature = "sixel_support")]
    play_frame(
        results,
        delay,
        0,
        st,
        config.mode.is_sixel(),
        config.clear,
        std::time::Duration::default(),
    );
    #[cfg(not(feature = "sixel_support"))]
    play_frame(results, delay, 0, st, false, config.clear);

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
    #[cfg(feature = "rodio")]
    if let Some(audio) = audio {
        audio.stop();
    }
    #[cfg(feature = "rodio")]
    if let Some(stream_handle) = stream_handle {
        std::mem::forget(stream_handle);
    }
}

#[allow(unused)]
#[cfg(feature = "video_player")]
pub fn render_video(
    vrx: crossbeam_channel::Receiver<(String, usize)>,
    #[cfg(feature = "rodio")] audio_path: AudioPath,
    fps: f32,
    is_sixel: bool,
    clear: bool,
) {
    // Load the audio if exists
    #[cfg(feature = "rodio")]
    let stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    #[cfg(feature = "rodio")]
    let file = audio_path
        .get_path()
        .map(|path| std::io::BufReader::new(File::open(path).unwrap()));
    #[cfg(feature = "rodio")]
    let sink = file.map(|file| rodio::play(&stream_handle.mixer(), file).unwrap());

    // calculate the delay
    let start_time = std::time::Instant::now();
    let (st, rt) = crossbeam_channel::unbounded::<JoinHandle<()>>();
    fn play_frame(
        frames: crossbeam_channel::Receiver<(String, usize)>,
        delay: f32,
        st: crossbeam_channel::Sender<JoinHandle<()>>,
        is_sixel: bool,
        back_top: bool,
        offset: std::time::Duration,
    ) {
        let frame = frames.recv();
        if frame.is_err() {
            return;
        }
        let frame = frame.unwrap();
        let (frame, index) = frame;
        let d = std::time::Duration::from_micros((1_000_000f32 / delay).round() as u64)
            .saturating_sub(offset);
        let st2 = st.clone();
        // create a new timer
        let timer = std::time::Instant::now();
        let task = std::thread::spawn(move || {
            std::thread::sleep(d);
            // calculate the time
            let time = timer.elapsed();
            play_frame(frames, delay, st2, is_sixel, back_top, time - d);
        });
        st.send(task).unwrap();

        if back_top {
            print!("\x1b[1;1H");
        } else {
            // Save current cursor position
            print!("\r\x1b[s");
        }

        println!("{}", frame);
        println!("current frame: {index}");

        if !back_top {
            // Back to the saved position
            print!("\x1b[u");
        }
    }

    if clear {
        print!("\x1bc");
    }

    play_frame(
        vrx,
        fps,
        st,
        is_sixel,
        clear,
        std::time::Duration::default(),
    );

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
    // audio_task.join().unwrap();
    // quit the audio stream
    #[cfg(feature = "rodio")]
    std::mem::forget(stream_handle);
}
