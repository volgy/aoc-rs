use itertools::iproduct;
use std::collections::HashSet;

aoc::parts!(1, 2);

type Pos = (isize, isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn turn_right(self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }

    fn next(self, pos: Pos) -> Pos {
        match self {
            Dir::Up => (pos.0, pos.1 - 1),
            Dir::Right => (pos.0 + 1, pos.1),
            Dir::Down => (pos.0, pos.1 + 1),
            Dir::Left => (pos.0 - 1, pos.1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Guard {
    pos: Pos,
    dir: Dir,
}

fn parse(input: aoc::Input) -> (Guard, (isize, isize), HashSet<Pos>) {
    let mut guard_pos = None;
    let mut obstacles = HashSet::new();
    let (mut max_x, mut max_y) = (0, 0);

    input.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, ch)| {
            let pos = (x as isize, y as isize);
            max_x = max_x.max(pos.0);
            max_y = max_y.max(pos.1);
            match ch {
                '#' => {
                    obstacles.insert(pos);
                    true
                }
                '^' => {
                    guard_pos = Some(pos);
                    true
                }
                _ => false,
            };
        });
    });

    (
        Guard {
            pos: guard_pos.unwrap(),
            dir: Dir::Up,
        },
        (max_x, max_y),
        obstacles,
    )
}

fn part_1(input: aoc::Input) -> impl ToString {
    let (mut guard, (max_x, max_y), obstacles) = parse(input);
    let mut visited = HashSet::new();

    while (0..=max_x).contains(&guard.pos.0) && (0..=max_y).contains(&guard.pos.1) {
        visited.insert(guard.pos);
        guard.pos = loop {
            let next_pos = guard.dir.next(guard.pos);
            if obstacles.contains(&next_pos) {
                guard.dir = guard.dir.turn_right();
            } else {
                break next_pos;
            }
        }
    }

    visited.len()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let (guard, (max_x, max_y), obstacles) = parse(input);
    let mut n_obstacles = 0;

    // Brute force is (borderline) feasible
    for pos in iproduct!(0isize..=max_x, 0isize..=max_y) {
        if guard.pos == pos || obstacles.contains(&pos) {
            continue;
        }

        let mut guard = guard;
        let mut obstacles = obstacles.clone();
        obstacles.insert(pos);

        let mut states = HashSet::new();
        let mut found_loop = false;

        while (0..=max_x).contains(&guard.pos.0) && (0..=max_y).contains(&guard.pos.1) {
            if !states.insert(guard) {
                found_loop = true;
                break;
            }
            guard.pos = loop {
                let next_pos = guard.dir.next(guard.pos);
                if obstacles.contains(&next_pos) {
                    guard.dir = guard.dir.turn_right();
                } else {
                    break next_pos;
                }
            };
        }

        if found_loop {
            n_obstacles += 1;
        }
    }

    n_obstacles
}
