use lazy_static::lazy_static;
use maplit::{hashmap, hashset};
use regex::Regex;
use std::collections::HashMap;

type Passport = HashMap<String, String>;

lazy_static! {
    static ref KV_RE: Regex = Regex::new(
        r"(?x)
        ^(?P<key>[[:word:]]+)
        :
        (?P<val>.+)$"
    )
    .unwrap();
}

fn parse_passport(text: &str) -> Passport {
    let mut passport = Passport::new();

    for item in text.trim().split_whitespace() {
        let cap = KV_RE.captures(item).expect("Fail to parse kv");
        let key = cap["key"].to_owned();
        let val = cap["val"].to_owned();
        passport.insert(key, val);
    }

    passport
}

fn solve_part_one(passports: &[Passport]) -> u32 {
    let mut valid_count = 0;

    for passport in passports {
        let mut needed_keys = hashset! {
            "byr",
            "iyr",
            "eyr",
            "hgt",
            "hcl",
            "ecl",
            "pid",
        };

        for key in passport.keys() {
            needed_keys.remove(key.as_str());
        }

        if needed_keys.is_empty() {
            valid_count += 1;
        }
    }

    valid_count
}

fn solve_part_two(passports: &[Passport]) -> u32 {
    type Checker = Box<dyn Fn(&str) -> bool>;

    // [byr]
    fn is_valid_byr(val: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\d{4}$").unwrap();
        }

        if !RE.is_match(val) {
            return false;
        }

        let number = val.parse::<u32>().unwrap();
        (number >= 1920) && (number <= 2002)
    }

    // [iyr]
    fn is_valid_iyr(val: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\d{4}$").unwrap();
        }

        if !RE.is_match(val) {
            return false;
        }

        let number = val.parse::<u32>().unwrap();
        (number >= 2010) && (number <= 2020)
    }

    // [eyr]
    fn is_valid_eyr(val: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\d{4}$").unwrap();
        }

        if !RE.is_match(val) {
            return false;
        }

        let number = val.parse::<u32>().unwrap();
        (number >= 2020) && (number <= 2030)
    }

    // [hgt]
    fn is_valid_hgt(val: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?x)
                ^(?P<number>\d+)
                (?P<suffix>cm|in)$"
            )
            .unwrap();
        }

        let cap = match RE.captures(val) {
            Some(cap) => cap,
            None => return false,
        };

        let number = cap["number"].parse::<u32>().unwrap();
        match &cap["suffix"] {
            "cm" => (number >= 150) && (number <= 193),
            _ => (number >= 59) && (number <= 76),
        }
    }

    // [hcl]
    fn is_valid_hcl(val: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#([0-9a-f]{6})$").unwrap();
        }

        RE.is_match(val)
    }

    // [ecl]
    fn is_valid_ecl(val: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        }

        RE.is_match(val)
    }

    // [pid]
    fn is_valid_pid(val: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([[:digit:]]{9})$").unwrap();
        }

        RE.is_match(val)
    }

    let checkers: HashMap<&str, Checker> = {
        hashmap! {
            "byr" => Box::new(is_valid_byr) as Checker,
            "iyr" => Box::new(is_valid_iyr) as Checker,
            "eyr" => Box::new(is_valid_eyr) as Checker,
            "hgt" => Box::new(is_valid_hgt) as Checker,
            "hcl" => Box::new(is_valid_hcl) as Checker,
            "ecl" => Box::new(is_valid_ecl) as Checker,
            "pid" => Box::new(is_valid_pid) as Checker,
        }
    };

    let mut valid_count = 0;

    for passport in passports {
        let mut needed_keys = hashset! {
            "byr",
            "iyr",
            "eyr",
            "hgt",
            "hcl",
            "ecl",
            "pid",
        };

        for (key, val) in passport {
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

    valid_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_4() {
        main();
    }
}

fn main() {
    let content = include_str!("../input.txt");
    let passports = content
        .split("\n\n")
        .map(parse_passport)
        .collect::<Vec<Passport>>();

    assert_eq!(solve_part_one(&passports), 264);
    assert_eq!(solve_part_two(&passports), 224);
}
