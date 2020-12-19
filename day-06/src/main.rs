use std::collections::{HashMap, HashSet};
use std::fs;

fn solve_part_one(groups: &[&str]) -> u32 {
    groups
        .iter()
        .map(|group| {
            let mut st = HashSet::new();

            for person in group.split_whitespace() {
                for ch in person.chars() {
                    st.insert(ch);
                }
            }

            st.len() as u32
        })
        .sum()
}

fn solve_part_two(groups: &[&str]) -> u32 {
    groups
        .iter()
        .map(|group| {
            let mut hm = HashMap::new();

            let people = group.split_whitespace().collect::<Vec<&str>>();

            for person in &people {
                for ch in person.chars() {
                    let next_count: usize = match hm.get(&ch) {
                        Some(count) => count + 1,
                        None => 1,
                    };
                    hm.insert(ch, next_count);
                }
            }

            hm.values().filter(|&v| *v == people.len()).count() as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_6() {
        main();
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Fail to read input file");
    let groups = contents.split("\n\n").collect::<Vec<&str>>();

    assert_eq!(solve_part_one(&groups), 6549);
    assert_eq!(solve_part_two(&groups), 3466);
}
