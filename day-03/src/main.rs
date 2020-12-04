use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, PartialEq)]
enum Item {
    Open,
    Tree,
}

fn count_tree(matrix: &Vec<Vec<Item>>, right: usize, down: usize) -> u32 {
    let row_num = matrix.len();
    let col_num = matrix[0].len();

    let mut curr_row = 0;
    let mut curr_col = 0;
    let mut tree_count = 0;

    for _ in 0..row_num {
        if matrix[curr_row][curr_col] == Item::Tree {
            tree_count += 1;
        }

        curr_row += right;
        curr_col = (curr_col + down) % col_num;
    }

    tree_count
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

    println!("Answer: {}", count_tree(&matrix, 1, 3));
}
