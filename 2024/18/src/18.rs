use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

aoc::parts!(1, 2);

type Pos = (usize, usize);

#[derive(Debug)]
struct Memory {
    falling: Vec<Pos>,
    max_pos: Pos,
}

impl Memory {
    fn parse(input: aoc::Input) -> Self {
        let falling: Vec<_> = input
            .lines()
            .map(|l| {
                l.split(",")
                    .map(|s| s.parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect();
        // heuristcs to work with example and actual cases
        let max_x = falling.iter().map(|&(x, _y)| x).max().unwrap();
        let max_y = falling.iter().map(|&(_x, y)| y).max().unwrap();
        Self {
            falling,
            max_pos: (max_x, max_y),
        }
    }

    fn escape(&self, n_falled: usize) -> Option<usize> {
        // BFS (unweighted Dijkstra)
        let mut visited: HashSet<_> = self.falling.iter().take(n_falled).cloned().collect(); // simple hack
        let mut states = VecDeque::new();
        states.push_back((0, (0, 0)));
        while let Some((distance, pos)) = states.pop_front() {
            if pos == self.max_pos {
                return Some(distance);
            }
            if !visited.insert(pos) {
                continue;
            }
            for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let next_pos = (pos.0.wrapping_add_signed(dx), pos.1.wrapping_add_signed(dy));
                if (0..=self.max_pos.0).contains(&next_pos.0)
                    && (0..=self.max_pos.1).contains(&next_pos.1)
                {
                    states.push_back(((distance + 1), next_pos));
                }
            }
        }
        None
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let memory = Memory::parse(input);

    // work with example and actual cases
    match memory.max_pos {
        (6, 6) => memory.escape(12).unwrap(),
        (70, 70) => memory.escape(1024).unwrap(),
        _ => unreachable!(),
    }
}

fn part_2(input: aoc::Input) -> impl ToString {
    let memory = Memory::parse(input);

    for (i, (prev, _)) in memory.falling.iter().tuple_windows().enumerate() {
        if memory.escape(i + 1).is_none() {
            return format!("{},{}", prev.0, prev.1);
        }
    }
    unreachable!()
}
