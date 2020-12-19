use std::collections::{HashMap, HashSet};
use std::fs;

fn solve_part_one(groups: &[&str]) {
    let mut sum = 0;

    for group in groups {
        let mut st = HashSet::new();

        for person in group.split_whitespace() {
            for ch in person.chars() {
                st.insert(ch);
            }
        }

        sum += st.len();
    }

    println!("[Part one]");
    println!("Answer: {}", sum);
    assert_eq!(sum, 6549);
}

fn solve_part_two(groups: &[&str]) {
    let mut sum = 0;

    for group in groups {
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

        sum += hm.values().filter(|&v| *v == people.len()).count();
    }

    println!("[Part two]");
    println!("Answer: {}", sum);
    assert_eq!(sum, 3466);
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Fail to read input file");
    let groups = contents.split("\n\n").collect::<Vec<&str>>();

    solve_part_one(&groups);
    solve_part_two(&groups);
}
