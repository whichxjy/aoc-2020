use std::collections::HashSet;
use std::fs;

fn solve_part_one(groups: &[&str]) {
    let mut sum = 0;

    for group in groups {
        let mut st = HashSet::new();

        for people in group.split_whitespace() {
            for ch in people.chars() {
                st.insert(ch);
            }
        }

        sum += st.len();
    }

    println!("[Part one]");
    println!("Answer: {}", sum);
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Fail to read input file");
    let groups = contents.split("\n\n").collect::<Vec<&str>>();

    solve_part_one(&groups);
}
