use crate::{
    config::{Config, cli::Cli},
    types::{
        ClapResizeMode,
        ImageType::{Image, Path},
    },
};
use image_to_console_core::{
    AutoResizeOption, CustomResizeOption, ResizeMode,
    processor::{ImageProcessor, ImageProcessorOptions},
    quantette::ColorSpace,
};

pub trait CreateIPFromConfig {
    fn from_config(config: &Config) -> Result<Self, String>
    where
        Self: Sized;
}

impl CreateIPFromConfig for ImageProcessor {
    fn from_config(config: &Config) -> Result<Self, String> {
        let option = ImageProcessorOptions {
            mode: config.mode,
            center: config.center,
            full: config.full_resolution,
            #[cfg(feature = "sixel_support")]
            dither: !config.disable_dither,
            resize_mode: config.resize_mode,
            black_background: config.black_background,
            enable_compression: config.enable_compression,
            #[cfg(feature = "sixel_support")]
            max_colors: config.max_colors,
            #[cfg(feature = "sixel_support")]
            color_space: ColorSpace::from(&config.color_space),
        };
        match config.image.clone() {
            Image(image) => Ok(Self::new(image, option)),
            Path(path) => {
                let image = image::open(path).map_err(|e| e.to_string())?;
                Ok(Self::new(image, option))
            }
            #[cfg(any(feature = "gif_player", feature = "video_player"))]
            _ => Err(String::from("cannot init")),
        }
    }
}

impl From<&Cli> for ResizeMode {
    fn from(cli: &Cli) -> Self {
        match cli.resize_mode {
            ClapResizeMode::Auto => Self::Auto(AutoResizeOption {
                width: !(cli.without_resize_width || cli.no_resize),
                height: !(cli.without_resize_height || cli.no_resize),
            }),
            ClapResizeMode::Custom => Self::Custom(CustomResizeOption {
                width: cli.width,
                height: cli.height,
            }),
            ClapResizeMode::None => Self::None,
        }
    }
}

impl From<Config> for image_to_console_renderer::config::Config {
    fn from(config: Config) -> Self {
        Self {
            fps: config.fps,
            clear: config.clear,
            pause: config.pause,
            center: config.center,
            output: config.output,
            file_name: config.file_name,
            show_time: config.show_time,
            disable_info: config.disable_info,
            disable_print: config.disable_print,
            show_file_name: config.show_file_name,
            #[cfg(feature = "audio_support")]
            audio: config.audio,
            #[cfg(feature = "sixel_support")]
            mode: config.mode,
        }
    }
}

impl From<&Config> for image_to_console_renderer::config::Config {
    fn from(config: &Config) -> Self {
        Self {
            fps: config.fps,
            clear: config.clear,
            pause: config.pause,
            center: config.center,
            show_time: config.show_time,
            output: config.output.clone(),
            disable_info: config.disable_info,
            file_name: config.file_name.clone(),
            disable_print: config.disable_print,
            show_file_name: config.show_file_name,
            #[cfg(feature = "audio_support")]
            audio: config.audio.clone(),
            #[cfg(feature = "sixel_support")]
            mode: config.mode,
        }
    }
}

#[cfg(feature = "video_player")]
pub fn pick_audio(
    path: &std::path::Path,
    target: &std::path::Path,
) -> Result<(), ffmpeg_next::Error> {
    let mut input_ctx = ffmpeg_next::format::input(path)?;
    let index = input_ctx
        .streams()
        .best(ffmpeg_next::media::Type::Audio)
        .ok_or(ffmpeg_next::error::Error::StreamNotFound)?
        .index();
    let input_stream = input_ctx.stream(index).unwrap();

    let mut output_ctx = ffmpeg_next::format::output(target)?;
    let mut output_stream = output_ctx.add_stream_with(
        &ffmpeg_next::codec::Context::from_parameters(input_stream.parameters())?,
    )?;

    output_stream.set_parameters(input_stream.parameters());
    output_ctx.write_header()?;

    for (stream, mut packet) in input_ctx.packets() {
        if stream.index() == index {
            packet.set_stream(0);
            packet.write_interleaved(&mut output_ctx)?;
        }
    }

    output_ctx.write_trailer()?;
    Ok(())
}

#[derive(Debug)]
struct StringSpan<'a> {
    content: &'a mut std::str::Chars<'a>,
    offset: usize,
    row: usize,
    col: usize,
    eof: bool,
    line: String,
    lines: Vec<String>,
    end_points: Vec<(usize, usize)>,
}

impl<'a> StringSpan<'a> {
    fn new(content: &'a mut std::str::Chars<'a>) -> Self {
        Self {
            col: 1,
            row: 1,
            content,
            offset: 0,
            eof: false,
            lines: vec![],
            end_points: vec![],
            line: String::new(),
        }
    }

    fn peek_to(&mut self, offset: usize) {
        if self.eof {
            return;
        }
        while self.offset < offset && self.next().is_some() {}
    }

    fn next(&mut self) -> Option<char> {
        if self.eof {
            return None;
        }
        let Some(char) = self.content.next() else {
            self.eof = true;
            self.lines.push(self.line.clone());
            self.end_points.push((self.row, self.col));
            return None;
        };
        if char == '\n' {
            self.lines.push(self.line.clone());
            self.end_points.push((self.row, self.col));

            self.col = 1;
            self.row += 1;
            self.offset += 1;
            self.line = String::new();
            return Some(char);
        }
        self.col += 1;
        self.offset += 1;
        self.line.push(char);
        Some(char)
    }

    fn read_line(&mut self) -> String {
        let row = self.row;
        while row == self.row && self.next().is_some() {}
        self.lines.last().unwrap().clone()
    }
}

#[cfg(feature = "dot_file")]
pub fn show_error<T>(error: toml::de::Error, source: T, path: &std::path::Path)
where
    T: AsRef<str>,
{
    use image_to_console_colored::{colors::TerminalColor, prelude::ToColoredText, styles::Styles};

    let source = source.as_ref();

    println!(
        "{}{}",
        "error"
            .to_colored_text()
            .set_foreground_color(TerminalColor::LightRed),
        format!(": {}", error.message())
            .to_colored_text()
            .set_foreground_color(TerminalColor::LightWhite)
    );

    let Some(span) = error.span() else {
        return;
    };

    let mut chars = source.chars();
    let mut s_span = StringSpan::new(&mut chars);

    s_span.peek_to(span.start);

    let start_col = s_span.col;

    let line = s_span.read_line();

    let (line_row, _) = *s_span.end_points.last().unwrap();

    let air = " ".repeat((line_row.ilog10() + 1) as usize);

    let mut split = error.message().split(',');
    let mut msg = split.next().unwrap();
    if split.next().is_none() {
        msg = "";
    }

    let head = |s: String| {
        s.to_colored_text()
            .set_style(Styles::Bold)
            .set_foreground_color(TerminalColor::LightCyan)
            .to_string()
    };

    println!(
        "{} {}:{line_row}:{start_col}",
        head(format!("{air}-->")),
        path.display()
    );
    println!("{}", head(format!("{air} |")));
    println!("{} {line}", head(format!("{line_row} |")));
    println!(
        "{}{}{}",
        head(format!("{air} |")),
        " ".repeat(start_col),
        format!("{} {}", "^".repeat(span.end - span.start), msg)
            .to_colored_text()
            .set_foreground_color(TerminalColor::LightRed)
    );
    println!();
}
