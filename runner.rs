// mod config;
// mod solver;

use crate::config::{ChallengeParts, Config};
use crate::solver;

use std::fs::File;
use std::io::BufReader;

pub fn execute(config: &Config) {
    let solver = match solver::get_solver_for(&config.day) {
        Some(s) => s,
        None => {
            eprintln!("Unable to find solver for day {}", &config.day);
            return;
        }
    };

    let reader_provider = || -> BufReader<File> {
        let file = File::open(&config.input_file).expect("Unable to open input file.");
        BufReader::new(file)
    };

    let result: Result<String, String> = match config.part {
        ChallengeParts::PartOne => solver.solve_part_one(&reader_provider),
        ChallengeParts::PartTwo => solver.solve_part_two(&reader_provider),
        ChallengeParts::Both => match solver.solve_both(&reader_provider) {
            Ok(v) => Ok(v.to_string()),
            Err(e) => Err(e),
        },
    };

    match result {
        Ok(v) => println!("Solution: {}", v),
        Err(e) => println!("Unable to compute solution: {}", e),
    }
}
