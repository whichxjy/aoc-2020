use std::collections::HashSet;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Fail to read input file");
    let groups = contents.split("\n\n").collect::<Vec<&str>>();

    let mut yes_sum = 0;

    for group in groups {
        let mut st = HashSet::new();

        for people in group.split_whitespace() {
            for ch in people.chars() {
                st.insert(ch);
            }
        }

        yes_sum += st.len();
    }

    println!("Answer: {}", yes_sum);
}
