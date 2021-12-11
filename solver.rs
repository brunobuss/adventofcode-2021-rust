use std::fmt::{Display, Error, Formatter};
use std::fs::File;
use std::io::{BufReader, Lines};

pub struct CompositeSolution(pub String, pub String);

impl Display for CompositeSolution {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "PartOne: {}, PartTwo: {}", self.0, self.1)
    }
}

pub trait Solver {
    fn solve_part_one(&self, input: Lines<BufReader<File>>) -> Result<String, String>;
    fn solve_part_two(&self, input: Lines<BufReader<File>>) -> Result<String, String>;
    fn solve_both(&self, input: Lines<BufReader<File>>) -> Result<CompositeSolution, String>;
}

pub fn get_solver_for(day: &u8) -> Option<Box<dyn Solver>> {
    match day {
        0 => Some(Box::new(STest {})),
        _ => None,
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
