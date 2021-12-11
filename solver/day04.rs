use solver::CompositeSolution;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day04Solver {}

impl super::Solver for Day04Solver {
    fn solve_both(
        &self,
        reader_provider: &dyn Fn() -> BufReader<File>,
    ) -> Result<CompositeSolution, String> {
        let mut reader = reader_provider();

        let mut draws_line = String::new();
        match reader.read_line(&mut draws_line) {
            Err(e) => return Err(e.to_string()),
            _ => (),
        }

        let mut current: Vec<Vec<u32>> = Vec::new();
        let mut boards: Vec<BingoBoard> = Vec::new();

        for line in reader.lines() {
            let line_content = match line {
                Ok(l) => l,
                Err(e) => return Err(e.to_string()),
            };
            if line_content.is_empty() {
                if current.len() > 0 {
                    boards.push(BingoBoard::new(current));
                }
                current = Vec::new();
                continue;
            }

            current.push(
                line_content
                    .split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect(),
            );
        }
        if current.len() > 0 {
            boards.push(BingoBoard::new(current));
        }

        let draws: Vec<u32> = draws_line
            .trim_end() // Remove CR LF bytes from string.
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        let n_boards = boards.len();
        let mut remaining = boards.len();

        let mut first = 0;
        let mut last = 0;

        for d in draws {
            for b in &mut boards {
                if b.is_bingo() {
                    continue;
                }
                b.mark(d);
                if b.is_bingo() {
                    if remaining == n_boards {
                        first = b.score();
                    } else if remaining == 1 {
                        last = b.score();
                    }
                    remaining -= 1;
                }
            }
        }

        Ok(CompositeSolution(first.to_string(), last.to_string()))
    }
}

struct BingoBoard {
    rows: Vec<HashSet<u32>>,
    columns: Vec<HashSet<u32>>,
    last_removed: u32,
}

impl BingoBoard {
    fn new(board: Vec<Vec<u32>>) -> BingoBoard {
        let row_count = board.len();
        let col_count = board[0].len();
        let mut rs = vec![HashSet::new(); row_count];
        let mut cs = vec![HashSet::new(); col_count];

        for i in 0..row_count {
            for j in 0..col_count {
                let val = board[i][j];
                rs[i].insert(val);
                cs[j].insert(val);
            }
        }

        return BingoBoard {
            rows: rs,
            columns: cs,
            last_removed: 0,
        };
    }

    fn mark(&mut self, number: u32) {
        let mut removed = false;
        for row in &mut self.rows {
            if row.remove(&number) {
                removed = true;
            }
        }
        for col in &mut self.columns {
            if col.remove(&number) {
                removed = true;
            }
        }
        if removed {
            self.last_removed = number;
        }
    }

    fn is_bingo(&self) -> bool {
        for row in &self.rows {
            if row.len() == 0 {
                return true;
            }
        }
        for col in &self.columns {
            if col.len() == 0 {
                return true;
            }
        }
        return false;
    }

    fn score(&self) -> u32 {
        let mut sum: u32 = 0;
        // We only need to sum rows (or columns), since the
        // numbers are repeated. Otherwise we would get twice the
        // count.
        for row in &self.rows {
            for v in row {
                sum += v;
            }
        }
        return sum * self.last_removed;
    }
}
