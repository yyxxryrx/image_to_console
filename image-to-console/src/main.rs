extern crate core;

mod config;
mod const_value;
mod runner;
mod types;
mod util;
#[allow(unused)]
mod errors;

use crate::runner::{run, run_multiple};
use config::RunMode::*;

fn main() {
    let config = config::parse();
    match config {
        Once(config) => run(config),
        Multiple(configs) => run_multiple(configs),
        #[cfg(any(feature = "video_player", feature = "gif_player"))]
        Video(config) => crate::runner::run_video(config),
    }
}
