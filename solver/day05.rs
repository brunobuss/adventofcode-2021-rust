use solver::CompositeSolution;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;

pub struct Day05Solver {}

impl super::Solver for Day05Solver {
    fn solve_both(
        &self,
        reader_provider: &dyn Fn() -> BufReader<File>,
    ) -> Result<CompositeSolution, String> {
        let reader = reader_provider();

        let mut ocean_part_one: Vec<Vec<u16>> = vec![vec![0; 1024]; 1024];
        let mut ocean_part_two: Vec<Vec<u16>> = vec![vec![0; 1024]; 1024];

        for line in reader.lines() {
            let line_content = match line {
                Ok(l) => l,
                Err(e) => return Err(e.to_string()),
            };

            let points: Vec<&str> = line_content.split("->").map(|p| p.trim()).collect();
            let p1 = Point::from(points[0]);
            let p2 = Point::from(points[1]);

            if p1.x == p2.x {
                let (s, e) = if p1.y < p2.y {
                    (p1.y, p2.y + 1)
                } else {
                    (p2.y, p1.y + 1)
                };
                for i in s..e {
                    ocean_part_one[p1.x][i] += 1;
                    ocean_part_two[p1.x][i] += 1;
                }
            } else if p1.y == p2.y {
                let (s, e) = if p1.x < p2.x {
                    (p1.x, p2.x + 1)
                } else {
                    (p2.x, p1.x + 1)
                };
                for i in s..e {
                    ocean_part_one[i][p1.y] += 1;
                    ocean_part_two[i][p1.y] += 1;
                }
            } else {
                let (sx, sy, up, c) = if p1.x < p2.x {
                    if p1.y < p2.y {
                        (p1.x, p1.y, true, p2.x - p1.x + 1)
                    } else {
                        (p1.x, p1.y, false, p2.x - p1.x + 1)
                    }
                } else {
                    if p2.y < p1.y {
                        (p2.x, p2.y, true, p1.x - p2.x + 1)
                    } else {
                        (p2.x, p2.y, false, p1.x - p2.x + 1)
                    }
                };
                for i in 0..c {
                    if up {
                        ocean_part_two[sx + i][sy + i] += 1;
                    } else {
                        ocean_part_two[sx + i][sy - i] += 1;
                    }
                }
            }
        }

        let mut count_part_one = 0;
        for row in ocean_part_one {
            for val in row {
                if val > 1 {
                    count_part_one += 1;
                }
            }
        }

        let mut count_part_two = 0;
        for row in ocean_part_two {
            for val in row {
                if val > 1 {
                    count_part_two += 1;
                }
            }
        }

        Ok(CompositeSolution(
            count_part_one.to_string(),
            count_part_two.to_string(),
        ))
    }
}

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from(string: &str) -> Point {
        let coords: Vec<usize> = string.split(',').map(|x| x.parse().unwrap()).collect();
        return Point {
            x: coords[0],
            y: coords[1],
        };
    }
}
