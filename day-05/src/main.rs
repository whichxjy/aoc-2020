use itertools::{max, min};
use std::collections::HashSet;
use std::fs;

enum Half {
    LowerHalf,
    UpperHalf,
}

fn find_index(start: u32, end: u32, half_seq: &[Half]) -> u32 {
    let mut left = start;
    let mut right = end;

    for half in half_seq {
        let mid = (left + right) / 2;

        match half {
            Half::LowerHalf => {
                right = mid;
            }
            Half::UpperHalf => {
                left = mid + 1;
            }
        }
    }

    left
}

fn determine_half(ch: char) -> Half {
    match ch {
        'F' | 'L' => Half::LowerHalf,
        _ => Half::UpperHalf,
    }
}

fn determine_row(half_seq: &[Half]) -> u32 {
    find_index(0, 127, half_seq)
}

fn determine_col(half_seq: &[Half]) -> u32 {
    find_index(0, 7, half_seq)
}

fn determine_seat_id(line: &str) -> u32 {
    let half_seq = line.chars().map(determine_half).collect::<Vec<Half>>();
    let row = determine_row(&half_seq[0..7]);
    let col = determine_col(&half_seq[7..10]);
    row * 8 + col
}

fn solve_part_one(seat_ids: &HashSet<u32>) -> u32 {
    *max(seat_ids).unwrap()
}

fn solve_part_two(seat_ids: &HashSet<u32>) -> u32 {
    let max_seat_id = *max(seat_ids).unwrap();
    let min_seat_id = *min(seat_ids).unwrap();

    (min_seat_id..=max_seat_id)
        .filter(|x| !seat_ids.contains(x))
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_5() {
        main();
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Fail to read input file");
    let lines = contents.split_whitespace().collect::<Vec<&str>>();
    let seat_ids = lines
        .into_iter()
        .map(determine_seat_id)
        .collect::<HashSet<u32>>();

    assert_eq!(solve_part_one(&seat_ids), 913);
    assert_eq!(solve_part_two(&seat_ids), 717);
}
