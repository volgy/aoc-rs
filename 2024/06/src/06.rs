use std::{collections::HashSet, hash::Hash};

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

#[derive(Debug, Clone)]
struct Lab {
    max_pos: Pos,
    obstacles: HashSet<Pos>,
    guard: Guard,
}

impl Lab {
    fn parse(input: aoc::Input) -> Self {
        let mut guard_pos = None;
        let mut obstacles = HashSet::new();
        let mut max_pos = (0, 0);

        input.lines().enumerate().for_each(|(y, l)| {
            l.chars().enumerate().for_each(|(x, ch)| {
                let pos = (x as isize, y as isize);
                max_pos = (max_pos.0.max(pos.0), max_pos.1.max(pos.1));
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

        Self {
            max_pos,
            obstacles,
            guard: Guard {
                pos: guard_pos.unwrap(),
                dir: Dir::Up,
            },
        }
    }

    fn get_out(&mut self) -> Option<HashSet<Pos>> {
        let mut states = HashSet::new();
        let mut visited = HashSet::new();

        while (0..=self.max_pos.0).contains(&self.guard.pos.0)
            && (0..=self.max_pos.1).contains(&self.guard.pos.1)
        {
            if !states.insert(self.guard) {
                return None;
            }
            visited.insert(self.guard.pos);
            self.guard.pos = loop {
                let next_pos = self.guard.dir.next(self.guard.pos);
                if self.obstacles.contains(&next_pos) {
                    self.guard.dir = self.guard.dir.turn_right();
                } else {
                    break next_pos;
                }
            };
        }
        Some(visited)
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut lab = Lab::parse(input);
    lab.get_out().unwrap().len()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let lab = Lab::parse(input);

    let mut candidates = lab.clone().get_out().unwrap();
    candidates.remove(&lab.guard.pos);

    // Brute force is feasible
    candidates
        .iter()
        .filter(|&&pos| {
            let mut lab = lab.clone();
            lab.obstacles.insert(pos);
            lab.get_out().is_none()
        })
        .count()
}
