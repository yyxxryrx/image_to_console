mod color;
mod config;
mod const_value;
mod display;
mod image;
mod runner;
mod types;
mod util;

use crate::runner::{run, run_multiple};
use config::RunMode::*;

fn main() {
    let config = config::parse();
    match config {
        Once(config) => run(config),
        Multiple(configs) => run_multiple(configs),
    }
}
