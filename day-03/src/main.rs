use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, PartialEq)]
enum Item {
    Open,
    Tree,
}

fn main() {
    let file = BufReader::new(File::open("input.txt").unwrap());
    let lines = file.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    let row_num = lines.len();
    let col_num = lines[0].len();

    let mut matrix = vec![vec![Item::Open; col_num]; row_num];

    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                matrix[row][col] = Item::Tree;
            }
        }
    }

    let mut curr_row = 0;
    let mut curr_col = 0;
    let mut tree_count = 0;

    for _ in 0..row_num {
        if matrix[curr_row][curr_col] == Item::Tree {
            tree_count += 1;
        }

        curr_row += 1;
        curr_col = (curr_col + 3) % col_num;
    }

    println!("Answer: {}", tree_count);
}
