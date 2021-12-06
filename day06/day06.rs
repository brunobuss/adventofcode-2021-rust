use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;

fn main() {
    part_a();
}

fn part_a() {
    let file = File::open("input.txt").expect("Unable to open input file.");
    let reader = BufReader::new(file);

    let max_delay: usize = 8;
    let days_to_simulate: usize = 256;

    // Notice that we initialize dp[days_to_simulate][..] with zeroes by doing this,
    // so we don't need to special case the first iteration below.
    let mut dp: Vec<Vec<u64>> = vec![vec![0; max_delay+1]; days_to_simulate+1];
    for day in (0..days_to_simulate).rev() {
        for delay in 0..=max_delay {
            dp[day][delay] = if delay == 0 {
                1 + dp[day+1][6] + dp[day+1][8]
            } else {
                dp[day+1][delay-1]
            };
        }
    }

    for line in reader.lines() {
        let line_content = line.expect("Unable to read line.");
        let fish: Vec<usize> = line_content.split(',')
                                .map(|x| x.parse().unwrap())
                                .collect();

        let mut acc = 0;
        for f in fish {
            acc += 1 + dp[0][f];
        }
        println!("Total Fishes: {}", acc);
    }
}