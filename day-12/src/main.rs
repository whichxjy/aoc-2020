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

enum FaceDir {
    North,
    South,
    East,
    West,
}

// North: y - 1
// South: y + 1
// East: x + 1
// West: x - 1
struct Ship {
    face_dir: FaceDir,
    pos_x: i32,
    pos_y: i32,
}

impl Ship {
    fn do_action(&mut self, action: &Action) {
        match action.action_kind {
            ActionKind::North => self.pos_y -= action.val as i32,
            ActionKind::South => self.pos_y += action.val as i32,
            ActionKind::East => self.pos_x += action.val as i32,
            ActionKind::West => self.pos_x -= action.val as i32,
            ActionKind::Left => self.turn_left(action.val),
            ActionKind::Right => self.turn_right(action.val),
            ActionKind::Forward => match self.face_dir {
                FaceDir::North => self.do_action(&Action {
                    action_kind: ActionKind::North,
                    val: action.val,
                }),
                FaceDir::South => self.do_action(&Action {
                    action_kind: ActionKind::South,
                    val: action.val,
                }),
                FaceDir::East => self.do_action(&Action {
                    action_kind: ActionKind::East,
                    val: action.val,
                }),
                FaceDir::West => self.do_action(&Action {
                    action_kind: ActionKind::West,
                    val: action.val,
                }),
            },
        }
    }

    fn turn_left(&mut self, degree: u32) {
        let times = degree / 90;

        for _ in 0..times {
            self.face_dir = match self.face_dir {
                FaceDir::North => FaceDir::West,
                FaceDir::South => FaceDir::East,
                FaceDir::East => FaceDir::North,
                FaceDir::West => FaceDir::South,
            };
        }
    }

    fn turn_right(&mut self, degree: u32) {
        let times = degree / 90;

        for _ in 0..times {
            self.face_dir = match self.face_dir {
                FaceDir::North => FaceDir::East,
                FaceDir::South => FaceDir::West,
                FaceDir::East => FaceDir::South,
                FaceDir::West => FaceDir::North,
            };
        }
    }
}

fn solve_part_one(actions: &[Action]) -> u32 {
    let mut ship = Ship {
        face_dir: FaceDir::East,
        pos_x: 0,
        pos_y: 0,
    };

    for action in actions {
        ship.do_action(action);
    }

    ship.pos_x.abs() as u32 + ship.pos_y.abs() as u32
}

fn main() {
    let content = include_str!("../input.txt");
    let lines = content.lines().collect::<Vec<&str>>();
    let actions = parse_actions(&lines);

    assert_eq!(solve_part_one(&actions), 1482);
}
