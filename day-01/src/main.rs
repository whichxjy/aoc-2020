use std::cmp::Ordering;
use std::collections::HashMap;

fn solve_part_one(entries: &[u32], target: u32) -> u32 {
    // (target - entry) -> entry
    let mut num_map = HashMap::new();

    let mut result = 0;

    for entry in entries {
        match num_map.get(entry) {
            Some(match_entry) => {
                result = entry * match_entry;
                break;
            }
            None => {
                num_map.insert(target - entry, *entry);
            }
        }
    }

    result
}

fn solve_part_two(entries: &[u32], target: u32) -> u32 {
    let mut entries = entries.to_vec();
    entries.sort_unstable();

    let mut result = 0;

    for i in 0..entries.len() {
        if i > 0 && entries[i] == entries[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = entries.len() - 1;

        while left < right {
            let three_sum = entries[i] + entries[left] + entries[right];

            match three_sum.cmp(&target) {
                Ordering::Less => {
                    left += 1;

                    while left < right && entries[left] == entries[left - 1] {
                        left += 1;
                    }
                }
                Ordering::Greater => {
                    right -= 1;

                    while left < right && entries[right] == entries[right + 1] {
                        right -= 1;
                    }
                }
                Ordering::Equal => {
                    result = entries[i] * entries[left] * entries[right];

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

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1() {
        main();
    }
}

fn main() {
    let content = include_str!("../input.txt");
    let entries = content
        .split_whitespace()
        .map(|e| e.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let target = 2020;

    assert_eq!(solve_part_one(&entries, target), 691771);
    assert_eq!(solve_part_two(&entries, target), 232508760);
}
