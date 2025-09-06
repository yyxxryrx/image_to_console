extern crate core;

mod color;
mod config;
mod const_value;
mod display;
mod runner;
mod types;
mod util;

use crate::runner::{run, run_multiple, run_video};
use config::RunMode::*;

fn main() {
    let config = config::parse();
    match config {
        Once(config) => run(config),
        Video(config) => run_video(config),
        Multiple(configs) => run_multiple(configs),
    }
}
