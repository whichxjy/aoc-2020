use std::collections::HashMap;
use std::fs;

fn parse_item(item: &str) -> (String, u32) {
    let item = item.trim_end_matches(" bag").trim_end_matches(" bags");
    let pieces = item.splitn(2, ' ').collect::<Vec<&str>>();

    let left_piece = pieces[0];
    let right_piece = pieces[1];

    let count = left_piece.parse::<u32>().unwrap();
    let color = right_piece.to_owned();

    (color, count)
}

fn process_lines(lines: &[&str]) -> HashMap<String, std::vec::Vec<(String, u32)>> {
    let mut color_mp = HashMap::new();

    for line in lines.iter().map(|l| l.trim_end_matches('.')) {
        let parts = line.splitn(2, " contain ").collect::<Vec<&str>>();

        let left_part = parts[0];
        let key_color = left_part.trim_end_matches(" bags").to_owned();

        let right_part = parts[1];
        let items = match right_part {
            "no other bags" => Vec::new(),
            _ => right_part
                .split(", ")
                .map(parse_item)
                .collect::<Vec<(String, u32)>>(),
        };

        color_mp.insert(key_color, items);
    }

    color_mp
}

fn solve_part_one(color_mp: &HashMap<String, std::vec::Vec<(String, u32)>>) {
    fn find_color(
        color_mp: &HashMap<String, std::vec::Vec<(String, u32)>>,
        start_color: &str,
        target_color: &str,
    ) -> bool {
        if start_color == target_color {
            return true;
        }

        for (in_color, _) in color_mp.get(start_color).unwrap() {
            if find_color(color_mp, in_color, target_color) {
                return true;
            }
        }

        false
    }

    let target_color = "shiny gold";
    let count = color_mp
        .keys()
        .filter(|key_color| *key_color != target_color)
        .map(|key_color| find_color(color_mp, key_color, target_color))
        .filter(|r| *r == true)
        .count();

    println!("[Part one]");
    println!("Answer: {}", count);
    assert_eq!(count, 179);
}

fn solve_part_two(color_mp: &HashMap<String, std::vec::Vec<(String, u32)>>) {
    fn count_bags(
        color_mp: &HashMap<String, std::vec::Vec<(String, u32)>>,
        start_color: &str,
    ) -> u32 {
        let items = color_mp.get(start_color).unwrap();

        1 + items
            .iter()
            .map(|(in_color, count)| count * count_bags(color_mp, in_color))
            .sum::<u32>()
    }

    fn count_inner_bags(
        color_mp: &HashMap<String, std::vec::Vec<(String, u32)>>,
        start_color: &str,
    ) -> u32 {
        count_bags(color_mp, start_color) - 1
    }

    let start_color = "shiny gold";
    let count = count_inner_bags(color_mp, start_color);

    println!("[Part two]");
    println!("Answer: {}", count);
    assert_eq!(count, 18925);
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Fail to read input file");
    let lines = contents.trim().split('\n').collect::<Vec<&str>>();

    let color_mp = process_lines(&lines);
    solve_part_one(&color_mp);
    solve_part_two(&color_mp);
}
