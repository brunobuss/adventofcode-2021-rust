use solver::CompositeSolution;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day02Solver {}

impl super::Solver for Day02Solver {
    fn solve_both(
        &self,
        reader_provider: &dyn Fn() -> BufReader<File>,
    ) -> Result<CompositeSolution, String> {
        let reader = reader_provider();

        let mut part_one_horizontal = 0;
        let mut part_one_depth = 0;

        let mut part_two_aim = 0;
        let mut part_two_horizontal = 0;
        let mut part_two_depth = 0;

        for line in reader.lines() {
            let line_content = match line {
                Ok(l) => l,
                Err(e) => return Err(e.to_string()),
            };
            let line_elements: Vec<&str> = line_content.split(' ').collect();
            let command = line_elements[0];
            let value: i32 = match line_elements[1].parse() {
                Ok(v) => v,
                Err(e) => return Err(e.to_string()),
            };

            match command {
                "forward" => {
                    // Part One
                    part_one_horizontal += value;
                    // Part Two
                    part_two_horizontal += value;
                    part_two_depth += part_two_aim * value;
                }
                "down" => {
                    // Part One
                    part_one_depth += value;
                    // Part Two
                    part_two_aim += value;
                }
                "up" => {
                    // Part One
                    part_one_depth -= value;
                    // Part Two
                    part_two_aim -= value;
                }
                &_ => (),
            }
        }

        let part_one_sol = part_one_horizontal * part_one_depth;
        let part_two_sol = part_two_horizontal * part_two_depth;
        Ok(CompositeSolution(
            part_one_sol.to_string(),
            part_two_sol.to_string(),
        ))
    }
}
