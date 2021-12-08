use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;

fn main() {
    part_a();
}

fn part_a() {
    let file = File::open("input.txt").expect("Unable to open input file.");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_content = line.expect("Unable to read line.");
        let mut crabs: Vec<i32> = line_content
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        crabs.sort();

        let mut sol_a = i32::MAX;
        let mut sol_a_steps = i32::MAX;
        let mut sol_b = i32::MAX;
        let mut sol_b_steps = i32::MAX;
        for i in crabs[0]..=crabs[crabs.len() - 1] {
            let mut acc_a = 0;
            let mut acc_b = 0;
            for j in 0..crabs.len() {
                let d = i - crabs[j];
                acc_a += d.abs();
                acc_b += steps(d.abs());
            }
            if acc_a < sol_a_steps {
                sol_a_steps = acc_a;
                sol_a = i;
            }
            if acc_b < sol_b_steps {
                sol_b_steps = acc_b;
                sol_b = i;
            }
        }

        println!("A: {} {}", sol_a, sol_a_steps);
        println!("B: {} {}", sol_b, sol_b_steps);
    }
}

fn steps(n: i32) -> i32 {
    return (n * (n + 1)) / 2;
}
