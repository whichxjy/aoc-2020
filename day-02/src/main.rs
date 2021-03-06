use lazy_static::lazy_static;
use regex::Regex;

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

fn solve_part_one(items: &[Item]) -> u32 {
    fn is_valid(item: &Item) -> bool {
        let count = item.password.chars().filter(|c| c == &item.letter).count();
        (count >= item.lowest) && (count <= item.highest)
    }

    items.iter().filter(|item| is_valid(item)).count() as u32
}

fn solve_part_two(items: &[Item]) -> u32 {
    fn is_valid(item: &Item) -> bool {
        let lowest_letter = item.password.chars().nth(item.lowest - 1).unwrap();
        let highest_letter = item.password.chars().nth(item.highest - 1).unwrap();
        (lowest_letter == item.letter) ^ (highest_letter == item.letter)
    }

    items.iter().filter(|item| is_valid(item)).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_2() {
        main();
    }
}

fn main() {
    let content = include_str!("../input.txt");
    let items = content
        .lines()
        .into_iter()
        .map(|l| parse_item_from_line(&l).unwrap())
        .collect::<Vec<Item>>();

    assert_eq!(solve_part_one(&items), 434);
    assert_eq!(solve_part_two(&items), 509);
}
