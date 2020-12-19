use std::collections::HashMap;
use std::fs;

fn parse_item(item: &str) -> (String, u32) {
    let item = item.trim_end_matches(" bags");
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
    println!("{:#?}", color_mp);
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Fail to read input file");
    let lines = contents.trim().split('\n').collect::<Vec<&str>>();

    let color_mp = process_lines(&lines);
    solve_part_one(&color_mp);
}
