use std::fs;

// Operation
#[derive(Debug)]
enum Opr {
    Acc,
    Jmp,
    Nop,
}

// Instruction
#[derive(Debug)]
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

fn solve_part_one(insts: &[Inst]) -> u32 {
    for inst in insts {
        println!("{:?}", inst);
    }
    0
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
    solve_part_one(&insts);
}
