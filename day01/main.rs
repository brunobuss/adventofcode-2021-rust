use std::collections::VecDeque;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part_a();
    part_b();
}

fn part_a() {
    let file = File::open("input.txt").expect("Unable to open input file.");
    let reader = BufReader::new(file);

    let mut last_depth: Option<i32> = None;
    let mut increments = 0;

    for line in reader.lines() {
        let depth: i32 = line.expect("Unable to read line.").parse().unwrap();
        match last_depth {
            Some(ld) => {
                if depth > ld {
                    increments = increments + 1;
                }
            }
            None => (),
        }
        last_depth = Some(depth);
    }

    println!("Part A solution: {}", increments);
}

fn part_b() {
    let file = File::open("input.txt").expect("Unable to open input file.");
    let reader = BufReader::new(file);

    let mut deq: VecDeque<i32> = VecDeque::new();
    let mut increments = 0;

    for line in reader.lines() {
        let depth: i32 = line.expect("Unable to read line.").parse().unwrap();
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

    println!("Part B solution: {}", increments);
}
