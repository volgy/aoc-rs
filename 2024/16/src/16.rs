use std::collections::{BinaryHeap, HashMap, HashSet};

aoc::parts!(1, 2);

type Pos = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    East,
    South,
    West,
    North,
}

impl Dir {
    const DIRS: [Dir; 4] = [Dir::East, Dir::South, Dir::West, Dir::North];

    fn cw(&self) -> Self {
        let next_index = (*self as usize + 1) % Self::DIRS.len();
        Self::DIRS[next_index]
    }

    fn ccw(&self) -> Self {
        let prev_index = (*self as usize + Self::DIRS.len() - 1) % Self::DIRS.len();
        Self::DIRS[prev_index]
    }

    fn dxdy(&self) -> (isize, isize) {
        match self {
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
            Self::North => (0, -1),
        }
    }
}

struct Map {
    spaces: HashSet<Pos>,
    start: Pos,
    end: Pos,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    cost: u64,
    pos: Pos,
    dir: Dir,
    path: Vec<Pos>,
}

impl State {
    fn new(pos: Pos, dir: Dir) -> Self {
        Self {
            cost: 0,
            pos,
            dir,
            path: vec![pos],
        }
    }

    fn step(&self, dir: Dir, step_cost: u64) -> Self {
        let (dx, dy) = dir.dxdy();
        let pos = (
            self.pos.0.wrapping_add_signed(dx),
            self.pos.1.wrapping_add_signed(dy),
        );
        let mut path = self.path.clone();
        path.push(pos);
        Self {
            cost: self.cost + step_cost,
            pos,
            dir,
            path,
        }
    }

    fn step_forward(&self) -> Self {
        self.step(self.dir, 1)
    }

    fn step_right(&self) -> Self {
        self.step(self.dir.cw(), 1001)
    }

    fn step_left(&self) -> Self {
        self.step(self.dir.ccw(), 1001)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Map {
    fn parse(input: aoc::Input) -> Self {
        let mut spaces = HashSet::new();
        let mut start = None;
        let mut end = None;

        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let pos = (x, y);
                match ch {
                    '.' => {
                        spaces.insert(pos);
                    }
                    'S' => {
                        start = Some(pos);
                    }
                    'E' => {
                        end = Some(pos);
                        spaces.insert(pos); // allows to enter into end pos
                    }
                    _ => {}
                }
            }
        }

        Self {
            spaces,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }

    fn min_cost(&self) -> (u64, HashSet<Pos>) {
        let mut states = BinaryHeap::new();
        let mut cheapest_visited = HashMap::new();
        // let mut min_cost = None;
        // let mut min_cost_locs = HashSet::new();
        let mut min_states = vec![];

        states.push(State::new(self.start, Dir::East));

        while let Some(state) = states.pop() {
            if *cheapest_visited
                .entry((state.pos, state.dir))
                .or_insert(state.cost)
                < state.cost
            {
                continue;
            }

            if state.pos == self.end {
                min_states.push(state);
                continue;
            }

            for next_state in [state.step_forward(), state.step_right(), state.step_left()] {
                if self.spaces.contains(&next_state.pos) {
                    states.push(next_state);
                }
            }
        }

        let min_cost = min_states.iter().map(|s| s.cost).min().unwrap();

        (
            min_cost,
            min_states
                .into_iter()
                .filter(|s| s.cost == min_cost)
                .flat_map(|s| s.path)
                .collect(),
        )
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    Map::parse(input).min_cost().0
}

fn part_2(input: aoc::Input) -> impl ToString {
    Map::parse(input).min_cost().1.len()
}
