// mod config;
// mod solver;

use crate::config::{ChallengeParts, Config};
use crate::solver::{CompositeSolution, Solver};

use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};

pub fn execute(config: &Config) {
    let file = File::open(&config.input_file).expect("Unable to open input file.");
    let reader = BufReader::new(file);

    let solver: Box<dyn Solver> = match config.day {
        0 => Box::new(STest {}),
        _ => {
            eprintln!("Unable to find solver for day {}", config.day);
            return;
        }
    };

    let result: Result<String, String> = match config.part {
        ChallengeParts::PartOne => solver.solve_part_one(reader.lines()),
        ChallengeParts::PartTwo => solver.solve_part_two(reader.lines()),
        ChallengeParts::Both => match solver.solve_both(reader.lines()) {
            Ok(v) => Ok(v.to_string()),
            Err(e) => Err(e),
        },
    };

    match result {
        Ok(v) => println!("Solution: {}", v),
        Err(e) => println!("Unable to compute solution: {}", e),
    }
}

struct STest {}

impl Solver for STest {
    fn solve_part_one(&self, _input: Lines<BufReader<File>>) -> Result<String, String> {
        Ok("PartOne".to_string())
    }

    fn solve_part_two(&self, _input: Lines<BufReader<File>>) -> Result<String, String> {
        Err("BugPartTwoOnly".to_string())
    }

    fn solve_both(&self, _input: Lines<BufReader<File>>) -> Result<CompositeSolution, String> {
        Ok(CompositeSolution(
            "PartOne".to_string(),
            "PartTwo".to_string(),
        ))
    }
}
