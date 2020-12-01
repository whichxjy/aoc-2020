use std::collections::HashMap;
use std::fs;

fn solve_part_one(entries: &[u32], target: u32) {
    // (2020 - entry) -> entry
    let mut num_map = HashMap::new();

    for entry in entries {
        match num_map.get(entry) {
            Some(match_entry) => {
                println!("Two entries: ({}, {})", entry, match_entry);
                println!("Answer: {}", entry * match_entry);
                break;
            }
            None => {
                num_map.insert(target - entry, *entry);
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Fail to read input file");
    let target = 2020;
    let entries = contents
        .split_whitespace()
        .map(|e| e.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    solve_part_one(&entries, target);
}
