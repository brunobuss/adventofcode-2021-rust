mod config;
mod runner;
mod solver;

use config::Config;
use std::{env, process};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Unable to parse arguments: {}", err);
        process::exit(1);
    });

    runner::execute(&config);
}
