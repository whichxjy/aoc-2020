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
    opr: Opr,
    arg: i32,
}

fn parse_insts(lines: &[&str]) -> Vec<Inst> {
    lines
        .iter()
        .map(|line| {
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

            Inst { opr, arg }
        })
        .collect::<Vec<Inst>>()
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Fail to read input file");
    let lines = contents.trim().split('\n').collect::<Vec<&str>>();

    let insts = parse_insts(&lines);

    for inst in insts {
        println!("{:?}", inst);
    }
}
