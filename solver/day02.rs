use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part_a();
    part_b();
}

fn part_a() {
    let file = File::open("input.txt").expect("Unable to open input file.");
    let reader = BufReader::new(file);

    let mut horizontal = 0;
    let mut depth = 0;

    for line in reader.lines() {
        let line_content = line.expect("Unable to read line.");
        let line_elements: Vec<&str> = line_content.split(' ').collect();
        let command = line_elements[0];
        let value: i32 = line_elements[1].parse().unwrap();

        match command {
            "forward" => horizontal += value,
            "down" => depth += value,
            "up" => depth -= value,
            &_ => (),
        }
    }

    println!(
        "Part A solution. Horizontal: {}. Depth: {}. Answer: {}.",
        horizontal,
        depth,
        horizontal * depth
    );
}

fn part_b() {
    let file = File::open("input.txt").expect("Unable to open input file.");
    let reader = BufReader::new(file);

    let mut aim = 0;
    let mut horizontal = 0;
    let mut depth = 0;

    for line in reader.lines() {
        let line_content = line.expect("Unable to read line.");
        let line_elements: Vec<&str> = line_content.split(' ').collect();
        let command = line_elements[0];
        let value: i32 = line_elements[1].parse().unwrap();

        match command {
            "forward" => {
                horizontal += value;
                depth += aim * value;
            }
            "down" => aim += value,
            "up" => aim -= value,
            &_ => (),
        }
    }

    println!(
        "Part B solution. Horizontal: {}. Depth: {}. Answer: {}.",
        horizontal,
        depth,
        horizontal * depth
    );
}
