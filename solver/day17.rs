use solver::CompositeSolution;
use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day17Solver {}

impl Day17Solver {}

impl super::Solver for Day17Solver {
    fn solve_both(
        &self,
        reader_provider: &dyn Fn() -> BufReader<File>,
    ) -> Result<CompositeSolution, String> {
        let reader = reader_provider();

        for line in reader.lines() {
            let line_content = match line {
                Ok(l) => l,
                Err(e) => return Err(e.to_string()),
            };

            let (min_x, max_x, min_y, max_y) = parse_input(&line_content);
            // println!("{} {} {} {}", min_x, max_x, min_y, max_y);

            let mut max_height = 0;
            let mut hits = 0;
            for s_x in 0..=max_x {
                // We skip starting x velocities that can't even arrive at the x range.
                if (s_x * (1 + s_x)) / 2 < min_x {
                    continue;
                }

                for s_y in min_y..(min_y.abs()) {
                    // println!("Launching with ({}, {})", s_x, s_y.abs());
                    let mut x_vel = s_x;
                    let mut x_pos = 0;
                    let mut y_vel = s_y;
                    let mut y_pos = 0;
                    // println!("Starting on: p:({}, {}), v:({}, {})", x_pos, y_pos, x_vel, y_vel);

                    while x_pos <= max_x && y_pos >= min_y {
                        if x_pos >= min_x && y_pos <= max_y {
                            hits += 1;
                            if s_y > 0 {
                                let my_height = (s_y.abs() * (s_y.abs() + 1)) / 2;
                                if my_height > max_height {
                                    max_height = my_height;
                                    // println!("Found new max_height of {}, by launching from ({}, {}) and hit the target at ({}, {})", max_height, s_x, s_y, x_pos, y_pos);
                                }
                            }
                            break;
                        }
                        x_pos += x_vel;
                        x_vel = max(x_vel - 1, 0);
                        y_pos += y_vel;
                        y_vel -= 1;
                        // println!(" - Keep going: p:({}, {}), v:({}, {})", x_pos, y_pos, x_vel, y_vel);
                    }
                }
            }
            return Ok(CompositeSolution(max_height.to_string(), hits.to_string()));
        }

        Err("Missing Input".to_string())
    }
}

fn parse_input(input_line: &str) -> (i32, i32, i32, i32) {
    // Input is like:
    // target area: x={x_1}..{x_2}, y={y_1}..{y_2}
    let after_x_equals_pos = input_line.find("x=").unwrap() + 2;
    let comma_pos = input_line.find(",").unwrap();
    let after_y_equals_pos = input_line.find("y=").unwrap() + 2;

    let x_parts: Vec<i32> = input_line[after_x_equals_pos..comma_pos]
        .split("..")
        .map(|x| x.parse().unwrap())
        .collect();
    let y_parts: Vec<i32> = input_line[after_y_equals_pos..]
        .split("..")
        .map(|x| x.parse().unwrap())
        .collect();

    (x_parts[0], x_parts[1], y_parts[0], y_parts[1])
}
