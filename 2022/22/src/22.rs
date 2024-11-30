use std::iter::repeat_n;

use regex::Regex;

aoc::parts!(1);

#[derive(Debug, Clone, Copy)]
enum Cell {
    Void,
    Free,
    Wall,
}

#[derive(Debug, Clone, Copy)]
enum Step {
    Left,
    Right,
    Forward(usize),
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Up => Self::Right,
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Up => Self::Left,
        }
    }

    fn dx_dy(&self) -> (isize, isize) {
        match self {
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Up => (0, -1),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct State {
    x: usize,
    y: usize,
    dir: Direction,
}

fn traverse(init: State, steps: &Vec<Step>, map: &Vec<Vec<Cell>>) -> State {
    let mut state = init;

    let size_x = map[0].len();
    let size_y = map.len();

    for step in steps {
        match step {
            Step::Left => state.dir = state.dir.turn_left(),
            Step::Right => state.dir = state.dir.turn_right(),
            &Step::Forward(mut n) => {
                let (dx, dy) = state.dir.dx_dy();
                let mut new_x = state.x;
                let mut new_y = state.y;
                while n > 0 {
                    new_x = (new_x as isize + dx).rem_euclid(size_x as isize) as usize;
                    new_y = (new_y as isize + dy).rem_euclid(size_y as isize) as usize;
                    match map[new_y][new_x] {
                        Cell::Void => {}
                        Cell::Free => {
                            state.x = new_x;
                            state.y = new_y;
                            n -= 1;
                        }
                        Cell::Wall => {
                            n = 0;
                        }
                    }
                }
            }
        }
    }
    state
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut rows: Vec<Vec<Cell>> = vec![];
    let mut lines = input.lines();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        rows.push(
            line.chars()
                .map(|c| match c {
                    ' ' => Cell::Void,
                    '.' => Cell::Free,
                    '#' => Cell::Wall,
                    _ => unreachable!(),
                })
                .collect(),
        );
    }

    let size_x = rows.iter().map(|r| r.len()).max().unwrap();
    for row in rows.iter_mut() {
        row.extend(repeat_n(Cell::Void, size_x - row.len()));
    }

    let mut steps = vec![];

    let re = Regex::new(r"R|L|\d+").unwrap();
    for m in re.find_iter(lines.next().unwrap()) {
        steps.push(match m.as_str() {
            "L" => Step::Left,
            "R" => Step::Right,
            num => Step::Forward(num.parse().unwrap()),
        });
    }

    let start_x = rows[0]
        .iter()
        .position(|c| matches!(c, Cell::Free))
        .unwrap();

    let final_state = traverse(
        State {
            x: start_x,
            y: 0,
            dir: Direction::Right,
        },
        &steps,
        &rows,
    );

    4 * (final_state.x + 1) + 1_000 * (final_state.y + 1) + final_state.dir as usize
}

// fn part_2(input: aoc::Input) -> impl ToString {
//     0
// }
