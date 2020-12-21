#[derive(Clone, PartialEq)]
enum Item {
    Open,
    Tree,
}

fn count_tree(matrix: &[Vec<Item>], right: usize, down: usize) -> u32 {
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

fn solve_part_one(matrix: &[Vec<Item>]) -> u32 {
    count_tree(&matrix, 3, 1)
}

fn solve_part_two(matrix: &[Vec<Item>]) -> u32 {
    let result_a = count_tree(matrix, 1, 1);
    let result_b = count_tree(matrix, 3, 1);
    let result_c = count_tree(matrix, 5, 1);
    let result_d = count_tree(matrix, 7, 1);
    let result_e = count_tree(matrix, 1, 2);
    result_a * result_b * result_c * result_d * result_e
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_3() {
        main();
    }
}

fn main() {
    let content = include_str!("../input.txt");
    let lines = content.trim().split('\n').collect::<Vec<&str>>();

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

    assert_eq!(solve_part_one(&matrix), 286);
    assert_eq!(solve_part_two(&matrix), 3638606400);
}
