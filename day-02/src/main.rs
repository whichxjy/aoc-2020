use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Item {
    lowest: usize,
    highest: usize,
    letter: char,
    password: String,
}

fn parse_item_from_line(line: &str) -> Option<Item> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?x)
            ^(?P<lowest>\d+)
            -
            (?P<highest>\d+)
            \s
            (?P<letter>[[:word:]])
            :\s
            (?P<password>[[:word:]]+)$"
        )
        .unwrap();
    }

    let cap = match RE.captures(line) {
        Some(cap) => cap,
        None => return None,
    };

    let lowest = cap["lowest"].parse::<usize>().unwrap();
    let highest = cap["highest"].parse::<usize>().unwrap();
    let letter = cap["letter"].parse::<char>().unwrap();
    let password = cap["password"].to_owned();

    Some(Item {
        lowest,
        highest,
        letter,
        password,
    })
}

fn solve_part_one(items: &[Item]) {
    fn is_valid(item: &Item) -> bool {
        let count = item.password.chars().filter(|c| c == &item.letter).count();
        (count >= item.lowest) && (count <= item.highest)
    }

    let valid_count = items.iter().filter(|item| is_valid(item)).count();
    println!("[Part one]");
    println!("Answer: {}", valid_count);
}

fn solve_part_two(items: &[Item]) {
    fn is_valid(item: &Item) -> bool {
        let lowest_letter = item.password.chars().nth(item.lowest - 1).unwrap();
        let highest_letter = item.password.chars().nth(item.highest - 1).unwrap();
        (lowest_letter == item.letter) ^ (highest_letter == item.letter)
    }

    let valid_count = items.iter().filter(|item| is_valid(item)).count();
    println!("[Part two]");
    println!("Answer: {}", valid_count);
}

fn main() {
    let file = BufReader::new(File::open("input.txt").unwrap());
    let lines = file.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let items = lines
        .into_iter()
        .map(|l| parse_item_from_line(&l).unwrap())
        .collect::<Vec<Item>>();

    solve_part_one(&items);
    solve_part_two(&items);
}
