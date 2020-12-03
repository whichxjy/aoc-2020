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

fn parse_item_from_line(line: &str) -> Option<Item> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?x)
            ^(?P<lowest>\d+)
            -
            (?P<highest>\d+)
            \s
            (?P<letter>[[:word:]])
            :
            \s
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

fn main() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    for line in file.lines() {
        parse_item_from_line(&line.unwrap());
    }
}
