use solver::CompositeSolution;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day01Solver{}

impl Day01Solver {

    fn line_to_depth(&self, line: &std::io::Result<String>) -> Result<i32, String> {
        match line {
            Ok(v) => match v.parse() {
                Ok(vp) => Ok(vp),
                Err(e) => {
                    let mut error = String::from("Unable to parse line: ");
                    error.push_str(&e.to_string());
                    Err(error)
                },
            },
            Err(e) => {
                let mut error = String::from("Unable to read line: ");
                error.push_str(&e.to_string());
                Err(error)
            },
        }
    }

}

impl super::Solver for Day01Solver {
    fn solve_part_one(&self, reader_provider: &dyn Fn() -> BufReader<File>) -> Result<String, String> {
        let reader = reader_provider();

        let mut last_depth: i32 = i32::MIN;
        let mut increments = 0;

        for line in reader.lines() {
            let depth: i32 = match self.line_to_depth(&line) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };
            if last_depth != i32::MIN && depth > last_depth {
                increments = increments + 1;
            }
            last_depth = depth;
        }

        Ok(increments.to_string())
    }

    fn solve_part_two(&self, reader_provider: &dyn Fn() -> BufReader<File>) -> Result<String, String> {
        let reader = reader_provider();

        let mut deq: VecDeque<i32> = VecDeque::new();
        let mut increments = 0;
    
        for line in reader.lines() {
            let depth: i32 = match self.line_to_depth(&line) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };
            if deq.len() == 3 {
                // Let say that S_{i} = a_{i} + a_{i+1} + a_{i+2}, if S_{i+1} > S_{i}
                // then a_{i+1} + a_{i+2} + a_{i+3} > a_{i} + a_{i+1} + a_{i+2}
                // then a_{i+3} > a_{i}.
                // So we only need to compare the last element of the new group with
                // the oldest element of the previous one.
                if depth > deq.pop_front().unwrap() {
                    increments = increments + 1;
                }
            }
            deq.push_back(depth);
        }

        Ok(increments.to_string())
    }

    fn solve_both(&self, reader_provider: &dyn Fn() -> BufReader<File>) -> Result<CompositeSolution, String> {
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
