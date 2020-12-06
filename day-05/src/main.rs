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

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Fail to read input file");
    let lines = contents.split_whitespace().collect::<Vec<&str>>();

    let mut highest_seat_id = 0;

    for line in lines {
        let seat_id = determine_seat_id(&line);

        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        }
    }

    println!("Answer: {}", highest_seat_id);
}
