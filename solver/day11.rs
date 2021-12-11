use std::collections::VecDeque;
use std::convert::TryInto;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    part_a();
}

fn part_a() {
    let file = File::open("input.txt").expect("Unable to open input file.");
    let reader = BufReader::new(file);

    let mut octopuses: Vec<Vec<u8>> = Vec::with_capacity(10);
    for line in reader.lines() {
        let line_content = line.expect("Unable to read line.");

        let mut row: Vec<u8> = Vec::with_capacity(10);
        for c in line_content.chars() {
            let digit: u8 = c.to_digit(10).unwrap().try_into().unwrap();
            row.push(digit);
        }
        octopuses.push(row);
    }

    let mut step: u32 = 0;
    let mut flash_all = false;
    while !flash_all {
        step += 1;

        let mut flash_count: u64 = 0;
        let mut flash_queue: VecDeque<(i8, i8)> = VecDeque::new();
        for i in 0..10 {
            for j in 0..10 {
                if bump_octopus((i, j), &mut octopuses, &mut flash_queue, true) {
                    flash_count += 1;
                }
            }
        }
        while !flash_queue.is_empty() {
            let pos = flash_queue.pop_front().unwrap();
            let i = pos.0;
            let j = pos.1;
            if bump_octopus((i - 1, j - 1), &mut octopuses, &mut flash_queue, false) {
                flash_count += 1;
            }
            if bump_octopus((i - 1, j), &mut octopuses, &mut flash_queue, false) {
                flash_count += 1;
            }
            if bump_octopus((i - 1, j + 1), &mut octopuses, &mut flash_queue, false) {
                flash_count += 1;
            }
            if bump_octopus((i, j - 1), &mut octopuses, &mut flash_queue, false) {
                flash_count += 1;
            }
            if bump_octopus((i, j + 1), &mut octopuses, &mut flash_queue, false) {
                flash_count += 1;
            }
            if bump_octopus((i + 1, j - 1), &mut octopuses, &mut flash_queue, false) {
                flash_count += 1;
            }
            if bump_octopus((i + 1, j), &mut octopuses, &mut flash_queue, false) {
                flash_count += 1;
            }
            if bump_octopus((i + 1, j + 1), &mut octopuses, &mut flash_queue, false) {
                flash_count += 1;
            }
        }

        if flash_count == 100 {
            flash_all = true;
        }
    }

    println!("A: {}", step);
}

fn bump_octopus(
    pos: (i8, i8),
    octopuses: &mut Vec<Vec<u8>>,
    queue: &mut VecDeque<(i8, i8)>,
    inc_zero: bool,
) -> bool {
    if pos.0 < 0 || pos.0 > 9 || pos.1 < 0 || pos.1 > 9 {
        return false;
    }

    let i = pos.0 as usize;
    let j = pos.1 as usize;

    if octopuses[i][j] == 9 {
        octopuses[i][j] = 0;
        queue.push_back(pos);
        return true;
    }
    if octopuses[i][j] > 0 || inc_zero {
        octopuses[i][j] += 1;
    }
    return false;
}
