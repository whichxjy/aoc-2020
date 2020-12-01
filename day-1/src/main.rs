use std::collections::HashMap;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Fail to read input file");

    // (2020 - entry) -> entry
    let mut num_map = HashMap::new();

    for entry in contents
        .split_whitespace()
        .map(|e| e.parse::<u32>().unwrap())
    {
        match num_map.get(&entry) {
            Some(match_entry) => {
                println!("Two entries: ({}, {})", entry, match_entry);
                println!("Answer: {}", entry * match_entry);
                break;
            }
            None => {
                num_map.insert(2020 - entry, entry);
            }
        }
    }
}
