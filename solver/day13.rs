use solver::CompositeSolution;
use std::cmp::max;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day13Solver {}

impl Day13Solver {}

impl super::Solver for Day13Solver {
    fn solve_both(
        &self,
        reader_provider: &dyn Fn() -> BufReader<File>,
    ) -> Result<CompositeSolution, String> {
        let reader = reader_provider();

        let mut folding = false;
        let mut dots: HashSet<(u16, u16)> = HashSet::new();
        let mut merged: HashSet<(u16, u16)> = HashSet::new();
        let mut max_x = u16::MIN;
        let mut max_y = u16::MIN;

        let mut dots_after_first_fold: usize = 0;

        for line in reader.lines() {
            let line_content = match line {
                Ok(l) => l,
                Err(e) => return Err(e.to_string()),
            };

            if line_content.trim().is_empty() {
                folding = true;
                continue;
            }

            if !folding {
                let coords: Vec<u16> = line_content
                    .split(',')
                    .map(|x| x.parse().unwrap())
                    .collect();
                let x = coords[0];
                let y = coords[1];
                max_x = max(max_x, x);
                max_y = max(max_y, y);
                dots.insert((x, y));
            } else {
                let parts: Vec<&str> = line_content.split('=').collect();

                let folding_up = parts[0].ends_with('y');
                let folding_point: u16 = parts[1].parse().unwrap();

                merged.clear();
                for d in &dots {
                    if folding_up {
                        if d.1 > folding_point {
                            merged.insert((d.0, folding_point - (d.1 - folding_point)));
                        } else {
                            merged.insert(*d);
                        }
                    } else {
                        if d.0 > folding_point {
                            merged.insert((folding_point - (d.0 - folding_point), d.1));
                        } else {
                            merged.insert(*d);
                        }
                    }
                }

                if folding_up {
                    max_y = folding_point;
                } else {
                    max_x = folding_point;
                }

                dots.clear();
                dots.extend(&merged);
                if dots_after_first_fold == 0 {
                    dots_after_first_fold = dots.len();
                }
            }
        }

        let mut answer_part_two = String::from("\n");
        for j in 0..max_y {
            for i in 0..max_x {
                if dots.contains(&(i, j)) {
                    answer_part_two.push('#');
                } else {
                    answer_part_two.push('.');
                }
            }
            answer_part_two.push_str("\n")
        }

        Ok(CompositeSolution(
            dots_after_first_fold.to_string(),
            answer_part_two,
        ))
    }
}
