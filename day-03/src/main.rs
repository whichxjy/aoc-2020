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

    while curr_row < row_num {
        if matrix[curr_row][curr_col] == Item::Tree {
            tree_count += 1;
        }

        curr_row += down;
        curr_col = (curr_col + right) % col_num;
    }

    tree_count
}

fn solve_part_one(matrix: &Vec<Vec<Item>>) {
    println!("[Part one]");
    println!("Answer: {}", count_tree(&matrix, 1, 3));
}

fn solve_part_two(matrix: &Vec<Vec<Item>>) {
    let result_a = count_tree(matrix, 1, 1);
    let result_b = count_tree(matrix, 3, 1);
    let result_c = count_tree(matrix, 5, 1);
    let result_d = count_tree(matrix, 7, 1);
    let result_e = count_tree(matrix, 1, 2);
    let result = result_a * result_b * result_c * result_d * result_e;

    println!("[Part two]");
    println!("Answer: {}", result);
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

    solve_part_one(&matrix);
    solve_part_two(&matrix);
}
