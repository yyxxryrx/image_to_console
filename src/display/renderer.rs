use crate::color::{colors::TerminalColor, prelude::ToColoredText};
use crate::config::Config;
use crate::image::processor::ImageProcessorResult;
use crate::util::get_char;
use std::io::Write;

pub fn render(result: ImageProcessorResult, config: Config) {
    if !config.disable_print {
        print!("\x1bc");
        for _ in 0..result.air_lines {
            println!();
        }
        let output = result.lines.join("\n");
        println!("{}", output);
    }
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
        "{}: \x1b[1mcenter={} resize-height={} full-resolution={}",
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
            .resize_height
            .to_string()
            .to_colored_text()
            .set_foreground_color(TerminalColor::Yellow),
        result
            .option
            .full
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
    if let Some(output) = config.output {
        let mut file = std::fs::File::create(output).expect("Create file failed!");
        file.write_all(result.lines.join("\n").as_bytes()).unwrap();
    }
    if config.pause {
        print!("Press the 'enter' to continue...");
        std::io::stdout().flush().unwrap();
        get_char();
    }
}
