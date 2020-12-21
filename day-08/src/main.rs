use std::collections::HashSet;
use std::fs;

// Operation
#[derive(Debug, Clone, PartialEq)]
enum Opr {
    Acc,
    Jmp,
    Nop,
}

// Instruction
#[derive(Debug, Clone)]
struct Inst {
    idx: usize,
    opr: Opr,
    arg: i32,
}

fn parse_insts(lines: &[&str]) -> Vec<Inst> {
    lines
        .iter()
        .enumerate()
        .map(|(idx, line)| {
            let pieces = line.splitn(2, ' ').collect::<Vec<&str>>();

            let left_piece = pieces[0];
            let right_piece = pieces[1];

            let opr = match left_piece {
                "acc" => Opr::Acc,
                "jmp" => Opr::Jmp,
                "nop" => Opr::Nop,
                _ => panic!("No such operation: {}", left_piece),
            };

            let arg = right_piece.parse::<i32>().unwrap();

            Inst { idx, opr, arg }
        })
        .collect::<Vec<Inst>>()
}

fn solve_part_one(insts: &[Inst]) -> i32 {
    let mut accumulator = 0;
    let mut curr_inst = &insts[0];
    let mut executed_inst_idxes = HashSet::new();

    while !executed_inst_idxes.contains(&curr_inst.idx) {
        executed_inst_idxes.insert(curr_inst.idx);

        // Execute current inst.
        let next_inst_idx = match curr_inst.opr {
            Opr::Acc => {
                accumulator += curr_inst.arg;
                curr_inst.idx + 1
            }
            Opr::Jmp => ((curr_inst.idx as i32) + curr_inst.arg) as usize,
            Opr::Nop => curr_inst.idx + 1,
        };

        // Set next inst.
        curr_inst = &insts[next_inst_idx];
    }

    accumulator
}

fn solve_part_two(insts: &[Inst]) -> i32 {
    fn flip_target_inst(insts: &mut [Inst], change_idx: usize) {
        match insts[change_idx].opr {
            Opr::Acc => panic!("Invalid inst"),
            Opr::Jmp => insts[change_idx].opr = Opr::Nop,
            Opr::Nop => insts[change_idx].opr = Opr::Jmp,
        }
    }

    fn check_by_index(insts: &mut Vec<Inst>, change_idx: usize) -> Option<i32> {
        if insts[change_idx].opr == Opr::Acc {
            return None;
        }

        // Change target inst.
        flip_target_inst(insts, change_idx);

        let mut accumulator = 0;
        let mut curr_inst_idx = 0;
        let mut executed_inst_idxes = HashSet::new();

        loop {
            if curr_inst_idx >= insts.len() {
                return Some(accumulator);
            }

            if executed_inst_idxes.contains(&curr_inst_idx) {
                break;
            }

            executed_inst_idxes.insert(curr_inst_idx);

            // Execute current inst.
            let curr_inst = &insts[curr_inst_idx];
            let next_inst_idx = match curr_inst.opr {
                Opr::Acc => {
                    accumulator += curr_inst.arg;
                    curr_inst.idx + 1
                }
                Opr::Jmp => ((curr_inst.idx as i32) + curr_inst.arg) as usize,
                Opr::Nop => curr_inst.idx + 1,
            };

            // Set next inst.
            curr_inst_idx = next_inst_idx;
        }

        // Repair target inst.
        flip_target_inst(insts, change_idx);
        None
    }

    let mut insts = insts.to_vec();

    (0..insts.len())
        .find_map(|i| check_by_index(&mut insts, i))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_8() {
        main();
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Fail to read input file");
    let lines = contents.trim().split('\n').collect::<Vec<&str>>();
    let insts = parse_insts(&lines);

    assert_eq!(solve_part_one(&insts), 1563);
    assert_eq!(solve_part_two(&insts), 767);
}
