#[derive(Debug, Clone, PartialEq)]
enum SeatKind {
    Floor,
    Empty,
    Occupied,
}

#[derive(Debug, Clone)]
struct Layout {
    row_num: usize,
    col_num: usize,
    seat_map: Vec<Vec<SeatKind>>,
}

fn parse_layout(lines: &[&str]) -> Layout {
    let row_num = lines.len();
    let col_num = lines[0].len();

    let mut seat_map = Vec::new();
    for line in lines {
        let seat_line = line
            .chars()
            .map(|ch| match ch {
                '.' => SeatKind::Floor,
                'L' => SeatKind::Empty,
                '#' => SeatKind::Occupied,
                _ => panic!("Invalid char"),
            })
            .collect::<Vec<SeatKind>>();

        seat_map.push(seat_line);
    }

    Layout {
        row_num,
        col_num,
        seat_map,
    }
}

fn solve_part_one(layout: &Layout) -> usize {
    fn get_seat_adjs(layout: &Layout, row: usize, col: usize) -> Vec<(usize, usize)> {
        let offsets: Vec<(i32, i32)> = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        offsets
            .iter()
            .map(|(row_offset, col_offset)| (row as i32 + row_offset, col as i32 + col_offset))
            .filter(|(next_row, next_col)| {
                *next_row >= 0
                    && *next_col >= 0
                    && *next_row < layout.row_num as i32
                    && *next_col < layout.col_num as i32
            })
            .map(|(next_row, next_col)| (next_row as usize, next_col as usize))
            .collect()
    }

    fn map_seat(layout: &Layout, row: usize, col: usize) -> SeatKind {
        match layout.seat_map[row][col] {
            SeatKind::Floor => SeatKind::Floor,
            SeatKind::Empty => match get_seat_adjs(layout, row, col)
                .iter()
                .map(|(next_row, next_col)| layout.seat_map[*next_row][*next_col].clone())
                .all(|seat_kind| seat_kind != SeatKind::Occupied)
            {
                true => SeatKind::Occupied,
                false => SeatKind::Empty,
            },
            SeatKind::Occupied => {
                let occupied_adjs_count = get_seat_adjs(layout, row, col)
                    .iter()
                    .map(|(next_row, next_col)| layout.seat_map[*next_row][*next_col].clone())
                    .filter(|seat_kind| *seat_kind == SeatKind::Occupied)
                    .count();

                if occupied_adjs_count >= 4 {
                    SeatKind::Empty
                } else {
                    SeatKind::Occupied
                }
            }
        }
    }

    let mut layout = layout.to_owned();

    loop {
        let mut is_changed = false;

        let mut new_seat_map = layout.seat_map.to_owned();
        for (row, seat_line) in new_seat_map.iter_mut().enumerate() {
            for (col, seat) in seat_line.iter_mut().enumerate() {
                let new_seat = map_seat(&layout, row, col);

                if new_seat != layout.seat_map[row][col] {
                    is_changed = true;
                }

                *seat = new_seat;
            }
        }

        layout.seat_map = new_seat_map;

        if !is_changed {
            break;
        }
    }

    layout
        .seat_map
        .iter()
        .flatten()
        .filter(|&seat_kind| *seat_kind == SeatKind::Occupied)
        .count()
}

fn solve_part_two(layout: &Layout) -> usize {
    fn get_visible_seats(layout: &Layout, row: usize, col: usize) -> Vec<(usize, usize)> {
        let offsets: Vec<(i32, i32)> = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut seats = Vec::new();

        for (row_offset, col_offset) in offsets {
            let mut curr_row = row as i32;
            let mut curr_col = col as i32;

            loop {
                curr_row += row_offset;
                curr_col += col_offset;

                if curr_row < 0
                    || curr_col < 0
                    || curr_row >= layout.row_num as i32
                    || curr_col >= layout.col_num as i32
                {
                    break;
                }

                match layout.seat_map[curr_row as usize][curr_col as usize] {
                    SeatKind::Floor => continue,
                    _ => {
                        seats.push((curr_row as usize, curr_col as usize));
                        break;
                    }
                }
            }
        }

        seats
    }

    fn map_seat(layout: &Layout, row: usize, col: usize) -> SeatKind {
        match layout.seat_map[row][col] {
            SeatKind::Floor => SeatKind::Floor,
            SeatKind::Empty => match get_visible_seats(layout, row, col)
                .iter()
                .map(|(next_row, next_col)| layout.seat_map[*next_row][*next_col].clone())
                .all(|seat_kind| seat_kind != SeatKind::Occupied)
            {
                true => SeatKind::Occupied,
                false => SeatKind::Empty,
            },
            SeatKind::Occupied => {
                let occupied_adjs_count = get_visible_seats(layout, row, col)
                    .iter()
                    .map(|(next_row, next_col)| layout.seat_map[*next_row][*next_col].clone())
                    .filter(|seat_kind| *seat_kind == SeatKind::Occupied)
                    .count();

                if occupied_adjs_count >= 5 {
                    SeatKind::Empty
                } else {
                    SeatKind::Occupied
                }
            }
        }
    }

    let mut layout = layout.to_owned();

    loop {
        let mut is_changed = false;

        let mut new_seat_map = layout.seat_map.to_owned();
        for (row, seat_line) in new_seat_map.iter_mut().enumerate() {
            for (col, seat) in seat_line.iter_mut().enumerate() {
                let new_seat = map_seat(&layout, row, col);

                if new_seat != layout.seat_map[row][col] {
                    is_changed = true;
                }

                *seat = new_seat;
            }
        }

        layout.seat_map = new_seat_map;

        if !is_changed {
            break;
        }
    }

    layout
        .seat_map
        .iter()
        .flatten()
        .filter(|&seat_kind| *seat_kind == SeatKind::Occupied)
        .count()
}

fn main() {
    let content = include_str!("../input.txt");
    let lines = content.lines().collect::<Vec<&str>>();
    let layout = parse_layout(&lines);

    assert_eq!(solve_part_one(&layout), 2324);
    assert_eq!(solve_part_two(&layout), 2068);
}
