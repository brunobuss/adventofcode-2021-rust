use solver::CompositeSolution;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day14Solver {}

impl Day14Solver {}

impl super::Solver for Day14Solver {
    fn solve_both(
        &self,
        reader_provider: &dyn Fn() -> BufReader<File>,
    ) -> Result<CompositeSolution, String> {
        let mut reader = reader_provider();

        let mut starting_pattern = String::new();
        match reader.read_line(&mut starting_pattern) {
            Err(e) => return Err(e.to_string()),
            _ => (),
        }

        let mut extensions: HashMap<String, String> = HashMap::new();

        for line in reader.lines() {
            let line_content = match line {
                Ok(l) => l,
                Err(e) => return Err(e.to_string()),
            };
            if line_content.trim().is_empty() {
                continue;
            }

            let parts: Vec<String> = line_content
                .split("->")
                .map(|x| x.trim().to_string())
                .collect();

            extensions.insert(parts[0].to_string(), parts[1].to_string());
        }

        let mut pd: Vec<HashMap<String, HashMap<char, u64>>> = vec![HashMap::new(); 40];

        for (k, v) in &extensions {
            let mut counter: HashMap<char, u64> = HashMap::new();
            for s in k.chars() {
                *counter.entry(s).or_insert(0) += 1;
            }
            for s in v.chars() {
                *counter.entry(s).or_insert(0) += 1;
            }
            pd[39].insert(k.to_string(), counter);
        }

        for i in (0..39).rev() {
            for (k, v) in &extensions {
                let mut counter: HashMap<char, u64> = HashMap::new();

                let (p1, p2) = expand_and_break(k, &extensions);
                let m1 = pd[i + 1].get(&p1).unwrap();
                for (ki, vi) in m1 {
                    *counter.entry(*ki).or_insert(0) += vi;
                }
                let m2 = pd[i + 1].get(&p2).unwrap();
                for (ki, vi) in m2 {
                    *counter.entry(*ki).or_insert(0) += vi;
                }
                // We counted this twice in the PD, so lets decrese by one here.
                for s in v.chars() {
                    *counter.entry(s).or_insert(0) -= 1;
                }

                pd[i].insert(k.to_string(), counter);
            }
        }

        // To get the 10 steps solution (part 1), we just look at an earlier row in the PD.
        let (min10c, max10c) = count_for(&starting_pattern, &pd[30]);
        let (min40c, max40c) = count_for(&starting_pattern, &pd[0]);
        Ok(CompositeSolution(
            (max10c - min10c).to_string(),
            (max40c - min40c).to_string(),
        ))
    }
}

fn expand_and_break(s: &String, extensions: &HashMap<String, String>) -> (String, String) {
    let mut p1 = String::from(s.get(0..1).unwrap());
    p1.push_str(extensions.get(s).unwrap());

    let mut p2 = String::from(extensions.get(s).unwrap());
    p2.push_str(s.get(1..2).unwrap());

    (p1, p2)
}

fn count_for(pattern: &String, counters: &HashMap<String, HashMap<char, u64>>) -> (u64, u64) {
    let mut final_counter: HashMap<char, u64> = HashMap::new();
    for pos in 1..pattern.trim().len() {
        let p = pattern.get(pos - 1..=pos).unwrap();
        let c = counters.get(&p.to_string()).unwrap();
        for (k, v) in c {
            *final_counter.entry(*k).or_insert(0) += v;
        }
        if pos >= 2 {
            // For the 2nd tuple onwards, we need to decrease the count of the first character,
            // because it was already counted in the previoud tuple.
            // For example: ABC, AB->D, BC->F, the final string is ADBFC, but both AB and BC
            // will count B character.
            let first_char = p.chars().next().unwrap();
            *final_counter.entry(first_char).or_insert(0) -= 1;
        }
    }
    let min_letter_c = final_counter.values().min().unwrap();
    let max_letter_c = final_counter.values().max().unwrap();
    (*min_letter_c, *max_letter_c)
}
