#[derive(Debug)]
enum ActionKind {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Debug)]
struct Action {
    action_kind: ActionKind,
    val: u32,
}

enum FaceDir {
    North,
    South,
    East,
    West,
}

struct Ship {
    face_dir: FaceDir,
    pos_x: i32,
    pos_y: i32,
}

fn parse_actions(lines: &[&str]) -> Vec<Action> {
    lines
        .iter()
        .map(|line| {
            let action_kind = match &line[0..=0] {
                "N" => ActionKind::North,
                "S" => ActionKind::South,
                "E" => ActionKind::East,
                "W" => ActionKind::West,
                "L" => ActionKind::Left,
                "R" => ActionKind::Right,
                "F" => ActionKind::Forward,
                _ => panic!("Invalid char"),
            };

            let val = line[1..].parse::<u32>().unwrap();

            Action { action_kind, val }
        })
        .collect()
}

fn main() {
    let content = include_str!("../input.txt");
    let lines = content.lines().collect::<Vec<&str>>();
    let actions = parse_actions(&lines);

    println!("{:#?}", actions);
}
