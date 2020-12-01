use std::collections::HashMap;
use std::fs;

fn solve_part_one(entries: &[u32], target: u32) {
    // (2020 - entry) -> entry
    let mut num_map = HashMap::new();

    for entry in entries {
        match num_map.get(entry) {
            Some(match_entry) => {
                println!("[Part one]");
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

fn solve_part_two(entries: &[u32], target: u32) {
    let mut entries = entries.to_vec();
    entries.sort_unstable();

    for i in 0..entries.len() {
        if i > 0 && entries[i] == entries[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = entries.len() - 1;

        while left < right {
            let three_sum = entries[i] + entries[left] + entries[right];

            if three_sum < target {
                left += 1;

                while left < right && entries[left] == entries[left - 1] {
                    left += 1;
                }
            } else if three_sum > target {
                right -= 1;

                while left < right && entries[right] == entries[right + 1] {
                    right -= 1;
                }
            } else {
                println!("[Part two]");
                println!(
                    "Three entries: ({}, {}, {})",
                    entries[i], entries[left], entries[right]
                );
                println!("Answer: {}", entries[i] * entries[left] * entries[right]);

                left += 1;
                right -= 1;

                while left < right && entries[left] == entries[left - 1] {
                    left += 1;
                }
                while left < right && entries[right] == entries[right + 1] {
                    right -= 1;
                }
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
    solve_part_two(&entries, target);
}
