use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;

fn main() {
    part_a();
    part_b();
}

fn part_a() {
    let file = File::open("input.txt").expect("Unable to open input file.");
    let reader = BufReader::new(file);

    let mut acc = 0;
    for line in reader.lines() {
        let line_content = line.expect("Unable to read line.");
        let parts: Vec<&str> = line_content.split('|').map(|x| x.trim()).collect();

        let outputs: Vec<&str> = parts[1].split(' ').collect();

        for output in &outputs {
            match output.len() {
                2 | 3 | 4 | 7 => acc += 1,
                _ => (),
            }
        }
    }
    println!("A: {}", acc);
}

fn part_b() {
    let file = File::open("input.txt").expect("Unable to open input file.");
    let reader = BufReader::new(file);

    let mut acc: u64 = 0;
    for line in reader.lines() {
        let line_content = line.expect("Unable to read line.");
        let parts: Vec<&str> = line_content.split('|').map(|x| x.trim()).collect();

        let mut inputs: Vec<u16> = parts[0].split(' ').map(|x| str_to_u16(x)).collect();
        // We sort by the number of 1 bits we have, as this is the same as sorting by
        // how many segments they have on.
        inputs.sort_by_key(|a| a.count_ones());

        let mut inputs_to_digits: HashMap<u16, u64> = HashMap::new();
        // These are known, as in part 1 we have only one possibility for them (by the count of segments).
        inputs_to_digits.insert(inputs[0], 1); // input[0] represents the digit 1
        inputs_to_digits.insert(inputs[1], 7);
        inputs_to_digits.insert(inputs[2], 4);
        inputs_to_digits.insert(inputs[9], 8);

        // Now we can find out about the digits with 6 segments (0, 6, 9)
        // because we can test them against the known ones.
        let mut index_of_6 = 0;
        for i in 6..=8 {
            let contains_1 = str_contains(inputs[i], inputs[0]);
            let contains_4 = str_contains(inputs[i], inputs[2]);
            let contains_7 = str_contains(inputs[i], inputs[1]);
            if contains_1 && contains_4 && contains_7 {
                inputs_to_digits.insert(inputs[i], 9);
            } else if contains_1 && contains_7 {
                inputs_to_digits.insert(inputs[i], 0);
            } else {
                index_of_6 = i;
                inputs_to_digits.insert(inputs[i], 6);
            }
        }

        // Now we figure out the digits with 5 segments, because we need 6
        // (that we just found above) to know between 5 and 2.
        for i in 3..=5 {
            let contains_1 = str_contains(inputs[i], inputs[0]);
            let contained_by_6 = str_contains(inputs[index_of_6], inputs[i]);
            if contains_1 {
                inputs_to_digits.insert(inputs[i], 3);
            } else if contained_by_6 {
                inputs_to_digits.insert(inputs[i], 5);
            } else {
                inputs_to_digits.insert(inputs[i], 2);
            }
        }

        let outputs: Vec<u16> = parts[1].split(' ').map(|x| str_to_u16(x)).collect();
        let output_val = inputs_to_digits[&outputs[0]] * 1000
            + inputs_to_digits[&outputs[1]] * 100
            + inputs_to_digits[&outputs[2]] * 10
            + inputs_to_digits[&outputs[3]];
        acc += output_val;
    }
    println!("B: {}", acc);
}

// We encode the string as bits, so we can deal only with
// integers and bitwise-and later instead of comparing
// strings/chars.
fn str_to_u16(a: &str) -> u16 {
    let mut val: u16 = 0;
    for c in a.chars() {
        match c {
            'a' => val += 1,
            'b' => val += 2,
            'c' => val += 4,
            'd' => val += 8,
            'e' => val += 16,
            'f' => val += 32,
            'g' => val += 64,
            _ => (),
        }
    }
    return val;
}

// Returns true iif all segments encoded in b are
// present in a.
fn str_contains(a: u16, b: u16) -> bool {
    return (a & b) == b;
}
