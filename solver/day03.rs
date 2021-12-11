use solver::CompositeSolution;
use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day03Solver {}

impl Day03Solver {}

impl super::Solver for Day03Solver {
    fn solve_part_one(
        &self,
        reader_provider: &dyn Fn() -> BufReader<File>,
    ) -> Result<String, String> {
        let reader = reader_provider();

        let mut bit_count: Vec<i32> = vec![0; 12];
        let mut len = 0;

        for line in reader.lines() {
            let line_content = match line {
                Ok(l) => l,
                Err(e) => return Err(e.to_string()),
            };
            let chars: Vec<char> = line_content.chars().collect();

            if len == 0 {
                len = chars.len();
            }

            for i in 0..len as usize {
                match chars[i] {
                    '0' => bit_count[i] -= 1,
                    '1' => bit_count[i] += 1,
                    _ => (),
                }
            }
        }

        let mut gamma_rate = 0;
        let mut epsilon_rate = 0;

        for i in 0..len as usize {
            gamma_rate <<= 1;
            epsilon_rate <<= 1;
            if bit_count[i] > 0 {
                // Most common bit = 1
                gamma_rate += 1;
            } else {
                // Most common bit = 0
                // (or both have the same count, since its not specified by the
                //  problem statement, treat this as 0 for now)
                epsilon_rate += 1;
            }
        }

        let answer = gamma_rate * epsilon_rate;
        Ok(answer.to_string())
    }

    fn solve_part_two(
        &self,
        reader_provider: &dyn Fn() -> BufReader<File>,
    ) -> Result<String, String> {
        let reader = reader_provider();

        let mut nums: Vec<i32> = Vec::new();
        let mut len = 0;

        for line in reader.lines() {
            let line_content = match line {
                Ok(l) => l,
                Err(e) => return Err(e.to_string()),
            };
            let bits: Vec<char> = line_content.chars().collect();

            if len == 0 {
                len = bits.len();
            }

            let mut number = 0;
            for i in 0..len as usize {
                number <<= 1;
                if bits[i] == '1' {
                    number += 1;
                }
            }
            nums.push(number);
        }

        nums.sort();
        let oxygen = find_in(&nums, true, 0, len - 1);
        let scrubber = find_in(&nums, false, 0, len - 1);
        let answer = oxygen * scrubber;
        Ok(answer.to_string())
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

fn find_in(v: &[i32], most_common: bool, cur: i32, bit_pos: usize) -> i32 {
    if v.len() == 1 {
        return v[0];
    }

    let target = cur + (1 << bit_pos);
    let bs_res = v.binary_search(&target);
    // We don't care if we find it or not, we just want the index.
    // .into_ok_or_err() in nightly =)
    let pos = match bs_res {
        Ok(i) => i,
        Err(i) => i,
    };
    // Left contains elements with 0 at bit_pos, right elements with 1 at bit_pos.
    let (left, right) = v.split_at(pos);

    // The bit criteria says that if we have the same number of 0s and 1s, then we favour
    // the 1s when considering most common bits.
    if left.len() > right.len() {
        if most_common {
            // We are searching for most common and have more 0s than 1s,
            // so we pass cur since it still has the bit in bit_pos set as 0.
            if left.len() == 1 {
                return left[0];
            }
            return find_in(left, most_common, cur, max(1, bit_pos) - 1);
        } else {
            // We are searching for least common and have more 0s than 1s,
            // so we pass target since it has the bit in bit_pos set as 1.
            if right.len() == 1 {
                return right[0];
            }
            return find_in(right, most_common, target, max(1, bit_pos) - 1);
        }
    } else {
        if most_common {
            // We are searching for most common and have more 1s than 0s,
            // so we pass target since it has the bit in bit_pos set as 1.
            if right.len() == 1 {
                return right[0];
            }
            return find_in(right, most_common, target, max(1, bit_pos) - 1);
        } else {
            // We are searching for least common and have more 1s than 0s,
            // so we pass cur since it still has the bit in bit_pos set as 0.
            if left.len() == 1 {
                return left[0];
            }
            return find_in(left, most_common, cur, max(1, bit_pos) - 1);
        }
    }
}
