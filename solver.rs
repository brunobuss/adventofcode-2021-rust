mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;

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
    ) -> Result<String, String> {
        match self.solve_both(reader_provider) {
            Ok(cs) => Ok(cs.0),
            Err(e) => Err(e),
        }
    }

    fn solve_part_two(
        &self,
        reader_provider: &dyn Fn() -> BufReader<File>,
    ) -> Result<String, String> {
        match self.solve_both(reader_provider) {
            Ok(cs) => Ok(cs.1),
            Err(e) => Err(e),
        }
    }

    fn solve_both(
        &self,
        reader_provider: &dyn Fn() -> BufReader<File>,
    ) -> Result<CompositeSolution, String> {
        let part_one = match self.solve_part_one(reader_provider) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let part_two = match self.solve_part_two(reader_provider) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        Ok(CompositeSolution(part_one, part_two))
    }
}

pub fn get_solver_for(day: &u8) -> Option<Box<dyn Solver>> {
    match day {
        0 => Some(Box::new(STest {})),
        1 => Some(Box::new(day01::Day01Solver {})),
        2 => Some(Box::new(day02::Day02Solver {})),
        3 => Some(Box::new(day03::Day03Solver {})),
        4 => Some(Box::new(day04::Day04Solver {})),
        5 => Some(Box::new(day05::Day05Solver {})),
        12 => Some(Box::new(day12::Day12Solver {})),
        13 => Some(Box::new(day13::Day13Solver {})),
        14 => Some(Box::new(day14::Day14Solver {})),
        15 => Some(Box::new(day15::Day15Solver {})),
        16 => Some(Box::new(day16::Day16Solver {})),
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
