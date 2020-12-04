use lazy_static::lazy_static;
use maplit::hashset;
use regex::Regex;
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

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Fail to read input file");
    let passports = contents
        .split("\n\n")
        .map(|p| p.trim())
        .collect::<Vec<&str>>();

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

    println!("Answer: {}", valid_count);
}
