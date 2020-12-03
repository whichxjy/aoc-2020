use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = BufReader::new(File::open("input.txt").unwrap());

    for line in file.lines() {
        println!("-> {}", line.unwrap());
    }
}
