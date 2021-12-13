use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Day12Solver {}

impl Day12Solver {}

impl super::Solver for Day12Solver {
    fn solve_part_one(
        &self,
        reader_provider: &dyn Fn() -> BufReader<File>,
    ) -> Result<String, String> {
        let reader = reader_provider();

        let mut edges: HashMap<String, Vec<String>> = HashMap::new();

        for line in reader.lines() {
            let line_content = match line {
                Ok(l) => l,
                Err(e) => return Err(e.to_string()),
            };
            let parts: Vec<&str> = line_content.split('-').collect();

            let edges_for_a = edges.entry(parts[0].to_string()).or_insert(Vec::new());
            edges_for_a.push(parts[1].to_string());
            let edges_for_b = edges.entry(parts[1].to_string()).or_insert(Vec::new());
            edges_for_b.push(parts[0].to_string());
        }

        let mut visited: HashSet<String> = HashSet::new();
        visited.insert("start".to_string());
        let count = dfs_counting_paths(&edges, &mut visited, "start".to_string());
        Ok(count.to_string())
    }

    fn solve_part_two(
        &self,
        reader_provider: &dyn Fn() -> BufReader<File>,
    ) -> Result<String, String> {
        let reader = reader_provider();

        let mut edges: HashMap<String, Vec<String>> = HashMap::new();

        for line in reader.lines() {
            let line_content = match line {
                Ok(l) => l,
                Err(e) => return Err(e.to_string()),
            };
            let parts: Vec<&str> = line_content.split('-').collect();

            let edges_for_a = edges.entry(parts[0].to_string()).or_insert(Vec::new());
            edges_for_a.push(parts[1].to_string());
            let edges_for_b = edges.entry(parts[1].to_string()).or_insert(Vec::new());
            edges_for_b.push(parts[0].to_string());
        }

        let mut visited: HashSet<String> = HashSet::new();
        visited.insert("start".to_string());
        let count = dfs_counting_paths_part_two(&edges, &mut visited, "start".to_string(), false);
        Ok(count.to_string())
    }
}

fn dfs_counting_paths(
    edges: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    node: String,
) -> u64 {
    if node == "end" {
        return 1;
    }
    let mut count: u64 = 0;
    // println!("Visiting {}", node);
    for e in edges.get(&node).unwrap() {
        if e.chars().next().unwrap().is_lowercase() && visited.contains(e) {
            continue;
        }
        visited.insert(e.to_string());
        count += dfs_counting_paths(edges, visited, e.to_string());
        visited.remove(e);
    }
    count
}

fn dfs_counting_paths_part_two(
    edges: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    node: String,
    small_cave_twice: bool,
) -> u64 {
    if node == "end" {
        return 1;
    }
    let mut count: u64 = 0;
    // println!("Visiting {}", node);
    for e in edges.get(&node).unwrap() {
        if e == "start" {
            // We never want to visit start again.
            continue;
        }
        let mut override_small_cave_twice = false;
        if e.chars().next().unwrap().is_lowercase() && visited.contains(e) {
            if small_cave_twice {
                continue;
            } else {
                override_small_cave_twice = true;
            }
        }
        visited.insert(e.to_string());
        count += dfs_counting_paths_part_two(
            edges,
            visited,
            e.to_string(),
            small_cave_twice || override_small_cave_twice,
        );
        if !override_small_cave_twice {
            visited.remove(e);
        }
    }
    count
}
