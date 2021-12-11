use std::collections::VecDeque;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part_a();
}

fn part_a() {
    let file = File::open("input.txt").expect("Unable to open input file.");
    let reader = BufReader::new(file);

    let mut syntax_score: u64 = 0;
    let mut autocomplete_scores: Vec<u64> = Vec::new();

    for line in reader.lines() {
        let line_content = line.expect("Unable to read line.");
        let mut stack: VecDeque<char> = VecDeque::new();
        let mut corrupted = false;
        for c in line_content.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push_back(c),
                ')' => {
                    if stack.pop_back().unwrap() != '(' {
                        syntax_score += 3;
                        corrupted = true;
                        break;
                    }
                }
                ']' => {
                    if stack.pop_back().unwrap() != '[' {
                        syntax_score += 57;
                        corrupted = true;
                        break;
                    }
                }
                '}' => {
                    if stack.pop_back().unwrap() != '{' {
                        syntax_score += 1197;
                        corrupted = true;
                        break;
                    }
                }
                '>' => {
                    if stack.pop_back().unwrap() != '<' {
                        syntax_score += 25137;
                        corrupted = true;
                        break;
                    }
                }
                _ => (),
            }
        }

        if !corrupted {
            let mut autocomplete_score: u64 = 0;
            while !stack.is_empty() {
                autocomplete_score *= 5;
                let c = stack.pop_back().unwrap();
                autocomplete_score += match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => 0,
                };
            }
            autocomplete_scores.push(autocomplete_score);
        }
    }

    println!("A: {}", syntax_score);

    autocomplete_scores.sort();
    println!("B: {}", autocomplete_scores[autocomplete_scores.len() / 2]);
}
