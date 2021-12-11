mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

use std::fmt::{Display, Error, Formatter};
use std::fs::File;
use std::io::BufReader;

pub struct CompositeSolution(pub String, pub String);

impl Display for CompositeSolution {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "PartOne: {}, PartTwo: {}", self.0, self.1)
    }
}

pub trait Solver {
    fn solve_part_one(
        &self,
        reader_provider: &dyn Fn() -> BufReader<File>,
    ) -> Result<String, String>;
    fn solve_part_two(
        &self,
        reader_provider: &dyn Fn() -> BufReader<File>,
    ) -> Result<String, String>;
    fn solve_both(
        &self,
        reader_provider: &dyn Fn() -> BufReader<File>,
    ) -> Result<CompositeSolution, String>;
}

pub fn get_solver_for(day: &u8) -> Option<Box<dyn Solver>> {
    match day {
        0 => Some(Box::new(STest {})),
        1 => Some(Box::new(day01::Day01Solver {})),
        2 => Some(Box::new(day02::Day02Solver {})),
        3 => Some(Box::new(day03::Day03Solver {})),
        4 => Some(Box::new(day04::Day04Solver {})),
        5 => Some(Box::new(day05::Day05Solver {})),
        _ => None,
    }
}

struct STest {}

impl Solver for STest {
    fn solve_part_one(
        &self,
        _reader_provider: &dyn Fn() -> BufReader<File>,
    ) -> Result<String, String> {
        Ok("PartOne".to_string())
    }

    fn solve_part_two(
        &self,
        _reader_provider: &dyn Fn() -> BufReader<File>,
    ) -> Result<String, String> {
        Err("BugPartTwoOnly".to_string())
    }

    fn solve_both(
        &self,
        _reader_provider: &dyn Fn() -> BufReader<File>,
    ) -> Result<CompositeSolution, String> {
        Ok(CompositeSolution(
            "PartOne".to_string(),
            "PartTwo".to_string(),
        ))
    }
}
