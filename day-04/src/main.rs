use lazy_static::lazy_static;
use maplit::{hashmap, hashset};
use regex::Regex;
use std::collections::HashMap;
use std::fs;

lazy_static! {
    static ref KV_RE: Regex = Regex::new(
        r"(?x)
        ^(?P<key>[[:word:]]+)
        :
        (?P<val>.+)$
        "
    )
    .unwrap();
}

fn parse_kv(passport: &str) -> (String, String) {
    let cap = KV_RE.captures(passport).expect("Fail to parse kv");
    let key = cap["key"].to_owned();
    let val = cap["val"].to_owned();
    (key, val)
}

fn solve_part_one(passports: &[&str]) {
    let mut valid_count = 0;

    for passport in passports {
        let kvs = passport.split_whitespace().collect::<Vec<&str>>();
        let mut needed_keys = hashset! {
            "byr",
            "iyr",
            "eyr",
            "hgt",
            "hcl",
            "ecl",
            "pid",
        };

        for kv in kvs {
            let (key, _) = parse_kv(kv);
            needed_keys.remove(key.as_str());
        }

        if needed_keys.is_empty() {
            valid_count += 1;
        }
    }

    println!("[Part one]");
    println!("Answer: {}", valid_count);
}

fn solve_part_two(passports: &[&str]) {
    type Checker = Box<dyn Fn(&str) -> bool>;

    fn is_valid_byr(val: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\d{4}$").unwrap();
        }

        if !RE.is_match(val) {
            return false;
        }

        let byr = val.parse::<u32>().unwrap();
        (byr >= 1920) && (byr <= 2002)
    }

    let checkers: HashMap<&str, Checker> = {
        hashmap! {
            "byr" => Box::new(is_valid_byr) as Checker
        }
    };

    let mut valid_count = 0;

    for passport in passports {
        let kvs = passport.split_whitespace().collect::<Vec<&str>>();
        let mut needed_keys = hashset! {
            "byr",
            "iyr",
            "eyr",
            "hgt",
            "hcl",
            "ecl",
            "pid",
        };

        for kv in kvs {
            let (key, val) = parse_kv(kv);

            let checker = match checkers.get(key.as_str()) {
                Some(checker) => checker,
                None => continue,
            };

            if checker(&val) {
                needed_keys.remove(key.as_str());
            }
        }

        if needed_keys.is_empty() {
            valid_count += 1;
        }
    }

    println!("[Part two]");
    println!("Answer: {}", valid_count);
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Fail to read input file");
    let passports = contents
        .split("\n\n")
        .map(|p| p.trim())
        .collect::<Vec<&str>>();

    solve_part_one(&passports);
    solve_part_two(&passports);
}
