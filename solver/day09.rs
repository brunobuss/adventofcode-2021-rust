use std::collections::HashSet;
use std::collections::VecDeque;
use std::convert::TryInto;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;

fn main() {
    part_a();
}

fn part_a() {
    let file = File::open("input.txt").expect("Unable to open input file.");
    let reader = BufReader::new(file);

    // Lets read the floor plan as save in a matrix.
    let mut floor_plan: Vec<Vec<u8>> = Vec::new();
    for line in reader.lines() {
        let line_content = line.expect("Unable to read line.");
        let mut row: Vec<u8> = Vec::new();

        for c in line_content.chars() {
            let digit: u8 = c.to_digit(10).unwrap().try_into().unwrap();
            row.push(digit);
        }
        floor_plan.push(row);
    }

    let rows = floor_plan.len();
    let columns = floor_plan[0].len();

    // Now for the first part of the problem, we will find and sum the risk
    // of all lowest points.
    // While we're doing it, we also save their coordinates in a vector,
    // so we can use them to compute the second part.
    let mut total_risk: u64 = 0;
    let mut lowest_points: Vec<(usize, usize)> = Vec::new();

    for i in 0..rows {
        for j in 0..columns {
            if (i > 0 && floor_plan[i][j] >= floor_plan[i - 1][j])
                || (i < (rows - 1) && floor_plan[i][j] >= floor_plan[i + 1][j])
                || (j > 0 && floor_plan[i][j] >= floor_plan[i][j - 1])
                || (j < (columns - 1) && floor_plan[i][j] >= floor_plan[i][j + 1])
            {
                continue;
            }
            let risk: u64 = (floor_plan[i][j] + 1) as u64;
            total_risk += risk;
            lowest_points.push((i, j));
        }
    }
    println!("A: {}", total_risk);

    // For the second part, we will do a search/floodfill from each of the lowest points.
    // The number of unique coordinates we are able to visit is the size of that basin.
    let mut basins: Vec<u32> = Vec::new();
    for lp in lowest_points {
        let mut basin_size: u32 = 0;
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        queue.push_back(lp);
        while !queue.is_empty() {
            let pos = queue.pop_front().unwrap();
            if visited.contains(&pos) {
                continue;
            }
            basin_size += 1;
            visited.insert(pos);

            let i = pos.0;
            let j = pos.1;

            // Maybe convert these conditions into a macro?
            if (i > 0)
                && (floor_plan[i - 1][j] != 9)
                && (floor_plan[i][j] < floor_plan[i - 1][j])
                && !visited.contains(&(i - 1, j))
            {
                queue.push_back((i - 1, j));
            }
            if (i < (rows - 1))
                && (floor_plan[i + 1][j] != 9)
                && (floor_plan[i][j] < floor_plan[i + 1][j])
                && !visited.contains(&(i + 1, j))
            {
                queue.push_back((i + 1, j));
            }
            if (j > 0)
                && (floor_plan[i][j - 1] != 9)
                && (floor_plan[i][j] < floor_plan[i][j - 1])
                && !visited.contains(&(i, j - 1))
            {
                queue.push_back((i, j - 1));
            }
            if (j < (columns - 1))
                && (floor_plan[i][j + 1] != 9)
                && (floor_plan[i][j] < floor_plan[i][j + 1])
                && !visited.contains(&(i, j + 1))
            {
                queue.push_back((i, j + 1));
            }
        }
        basins.push(basin_size);
    }
    basins.sort();
    let bcount = basins.len();
    let answer_b = basins[bcount - 1] * basins[bcount - 2] * basins[bcount - 3];
    println!("B: {}", answer_b);
}
