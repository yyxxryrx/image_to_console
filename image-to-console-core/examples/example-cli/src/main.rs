use clap::Parser;
use image::error::ImageResult;
use image_to_console_core::{processor::{ImageProcessorOptions, ImageProcessorOptionsCreate}, protocol::Protocol};

#[derive(Debug, Parser)]
struct Cli {
    #[clap(short, long, default_value = "auto")]
    pub protocol: Protocol,
    pub file: String,
}

fn main() -> ImageResult<()> {
    let cli = Cli::parse();
    let img = image::open(cli.file)?;
    let result = ImageProcessorOptions::default()
        .option_display_mode(cli.protocol.builder().build())
        .create_processor(img)
        .process();
    println!("{}", result.unwrap().display());
    Ok(())
}
