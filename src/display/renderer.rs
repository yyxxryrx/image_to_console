use crate::config::Config;
use crate::image::processor::ImageProcessorResult;
use crate::util::get_char;
use std::io::Write;

pub fn render(result: ImageProcessorResult, config: Config) {
    print!("\x1bc");
    for _ in 0..result.air_lines {
        println!();
    }
    let output = result.lines.join("\n");
    println!("{}", output);
    println!(
        "\x1b[0;32mImage Size\x1b[0m: \x1b[1m{} x {}\x1b[0m",
        result.width, result.height
    );
    if let Some(file_name) = config.file_name {
        println!("\x1b[0;32mFile Name\x1b[0m: \x1b[1;34m{}\x1b[0m", file_name);
    }

    println!(
        "\x1b[0;32mArgs\x1b[0m: \x1b[1mcenter=\x1b[0;33m{}\x1b[0m resize-height=\x1b[0;33m{}\x1b[0m",
        config.center,
        result.option.resize_height
    );
    if config.show_time {
        println!();
        let duration = result.time.elapsed().unwrap();
        let min = duration.as_secs() / 60;
        let sec = duration.as_secs() % 60;
        let ms = duration.as_millis() % 1000;
        println!("\x1b[1;32mRENDER FINISHED IN \x1b[1;92m{:0>2}:{:0>2}.{:0>3}ms\x1b[0m", min, sec, ms);
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
