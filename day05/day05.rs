use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::vec::Vec;

fn main() {
    part_a();
}

fn part_a() {
    let file = File::open("input.txt").expect("Unable to open input file.");
    let reader = BufReader::new(file);

    let mut ocean: Vec<Vec<u16>> = vec![vec![0; 1024]; 1024];

    for line in reader.lines() {
        let line_content = line.expect("Unable to read line.");

        let points: Vec<&str> = line_content.split("->").map(|p| p.trim()).collect();
        let p1 = Point::from(points[0]);
        let p2 = Point::from(points[1]);

        if p1.x == p2.x {
            let (s, e) = if p1.y < p2.y {
                (p1.y, p2.y + 1)
            } else {
                (p2.y, p1.y + 1)
            };
            for i in s..e {
                ocean[p1.x][i] += 1;
            }
        } else if p1.y == p2.y {
            let (s, e) = if p1.x < p2.x {
                (p1.x, p2.x + 1)
            } else {
                (p2.x, p1.x + 1)
            };
            for i in s..e {
                ocean[i][p1.y] += 1;
            }
        } else {
            let (sx, sy, up, c) = if p1.x < p2.x {
                if p1.y < p2.y {
                    (p1.x, p1.y, true, p2.x - p1.x + 1)
                } else {
                    (p1.x, p1.y, false, p2.x - p1.x + 1)
                }
            } else {
                if p2.y < p1.y {
                    (p2.x, p2.y, true, p1.x - p2.x + 1)
                } else {
                    (p2.x, p2.y, false, p1.x - p2.x + 1)
                }
            };
            for i in 0..c {
                if up {
                    ocean[sx + i][sy + i] += 1;
                } else {
                    ocean[sx + i][sy - i] += 1;
                }
            }
        }
    }

    let mut count = 0;
    for row in ocean {
        for val in row {
            // print!("{}", val);
            if val > 1 {
                count += 1;
            }
        }
        // println!("");
    }

    println!("Vents: {}", count);
}

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from(string: &str) -> Point {
        let coords: Vec<usize> = string.split(',').map(|x| x.parse().unwrap()).collect();
        return Point {
            x: coords[0],
            y: coords[1],
        };
    }
}
