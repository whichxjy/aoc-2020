use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Item {
    lowest: u32,
    highest: u32,
    letter: char,
    password: String,
}

impl Item {
    fn is_valid(&self) -> bool {
        let count = self.password.chars().filter(|c| c == &self.letter).count() as u32;
        (count >= self.lowest) && (count <= self.highest)
    }
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
            (?P<password>[[:word:]]+)$
            "
        )
        .unwrap();
    }

    let cap = match RE.captures(line) {
        Some(cap) => cap,
        None => return None,
    };

    let lowest = cap["lowest"].parse::<u32>().unwrap();
    let highest = cap["highest"].parse::<u32>().unwrap();
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
    let valid_count = items.iter().filter(|item| item.is_valid()).count();
    println!("[Part one]");
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
}
