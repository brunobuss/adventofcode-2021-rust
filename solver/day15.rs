use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day15Solver {}

impl Day15Solver {}

impl super::Solver for Day15Solver {
    fn solve_part_one(
        &self,
        reader_provider: &dyn Fn() -> BufReader<File>,
    ) -> Result<String, String> {
        let reader = reader_provider();

        let mut cavern: Vec<Vec<u32>> = Vec::new();

        for line in reader.lines() {
            let line_content = match line {
                Ok(l) => l,
                Err(e) => return Err(e.to_string()),
            };

            let vec_line: Vec<u32> = line_content
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect();

            cavern.push(vec_line);
        }

        let shortest = shortest_path(&cavern, 1);
        Ok(shortest.to_string())
    }

    fn solve_part_two(
        &self,
        reader_provider: &dyn Fn() -> BufReader<File>,
    ) -> Result<String, String> {
        let reader = reader_provider();

        let mut cavern: Vec<Vec<u32>> = Vec::new();

        for line in reader.lines() {
            let line_content = match line {
                Ok(l) => l,
                Err(e) => return Err(e.to_string()),
            };

            let vec_line: Vec<u32> = line_content
                .chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect();

            cavern.push(vec_line);
        }

        let shortest = shortest_path(&cavern, 5);
        Ok(shortest.to_string())
    }
}

fn shortest_path(cavern: &Vec<Vec<u32>>, mul: usize) -> u32 {
    let o_rows = cavern.len();
    let rows = o_rows * mul;
    let o_cols = cavern[0].len();
    let cols = o_cols * mul;
    let starting = Point {
        x: 0,
        y: 0,
        dist: 0,
    };

    let mut dist_heap = BinaryHeap::new();
    dist_heap.push(Reverse(starting));

    let mut visited: Vec<Vec<bool>> = vec![vec![false; cols]; rows];
    let mut shortest = 0;

    while !dist_heap.is_empty() {
        let n = match dist_heap.pop().unwrap() {
            Reverse(v) => v,
        };
        if visited[n.x][n.y] {
            continue;
        }

        // Have we arrived? Then take the distance so far.
        if (n.x == rows - 1) && (n.y == cols - 1) {
            shortest = n.dist;
            break;
        }

        if n.x > 0 && visited[n.x - 1][n.y] == false {
            dist_heap.push(Reverse(Point {
                x: n.x - 1,
                y: n.y,
                dist: n.dist + get_risk(cavern, o_rows, o_cols, n.x - 1, n.y),
            }));
        }
        if n.x < rows - 1 && visited[n.x + 1][n.y] == false {
            dist_heap.push(Reverse(Point {
                x: n.x + 1,
                y: n.y,
                dist: n.dist + get_risk(cavern, o_rows, o_cols, n.x + 1, n.y),
            }));
        }
        if n.y > 0 && visited[n.x][n.y - 1] == false {
            dist_heap.push(Reverse(Point {
                x: n.x,
                y: n.y - 1,
                dist: n.dist + get_risk(cavern, o_rows, o_cols, n.x, n.y - 1),
            }));
        }
        if n.y < cols - 1 && visited[n.x][n.y + 1] == false {
            dist_heap.push(Reverse(Point {
                x: n.x,
                y: n.y + 1,
                dist: n.dist + get_risk(cavern, o_rows, o_cols, n.x, n.y + 1),
            }));
        }

        visited[n.x][n.y] = true;
    }

    shortest
}

fn get_risk(cavern: &Vec<Vec<u32>>, o_rows: usize, o_cols: usize, row: usize, col: usize) -> u32 {
    let risk_offset = (row / o_rows) + (col / o_cols);
    let real_row = row % o_rows;
    let real_col = col % o_cols;
    let risk_with_offset = cavern[real_row][real_col] + (risk_offset as u32);
    (risk_with_offset % 10) + (risk_with_offset / 10)
}

#[derive(Debug, Eq)]
struct Point {
    x: usize,
    y: usize,
    dist: u32,
}

impl std::cmp::Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}
