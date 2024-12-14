use std::collections::HashSet;

use itertools::Itertools;

aoc::parts!(1, 2);

type Pos = (usize, usize);

#[derive(Debug)]
struct Topo {
    map: Vec<Vec<u32>>,
    size: (usize, usize),
    trailheads: Vec<Pos>,
}

impl Topo {
    fn parse(input: aoc::Input) -> Self {
        let mut trailheads = vec![];
        let map = input
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .enumerate()
                    .map(|(x, d)| {
                        if d == 0 {
                            trailheads.push((x, y))
                        }
                        d
                    })
                    .collect_vec()
            })
            .collect_vec();

        let size = (map[0].len(), map.len());

        Self {
            map,
            size,
            trailheads,
        }
    }

    fn n_reachable(&self, pos: Pos, peaks: &mut Option<HashSet<Pos>>) -> u32 {
        let height = self.map[pos.1][pos.0];
        if height == 9 {
            if let Some(peaks) = peaks {
                if peaks.insert(pos) {
                    1
                } else {
                    0
                }
            } else {
                1
            }
        } else {
            [(1, 0), (0, 1), (-1, 0), (0, -1)]
                .iter()
                .filter_map(|&(dx, dy)| {
                    let next_pos = (pos.0.wrapping_add_signed(dx), pos.1.wrapping_add_signed(dy));
                    ((0..self.size.0).contains(&next_pos.0)
                        && (0..self.size.1).contains(&next_pos.1)
                        && self.map[next_pos.1][next_pos.0] == height + 1)
                        .then(|| self.n_reachable(next_pos, peaks))
                })
                .sum()
        }
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let topo = Topo::parse(input);
    topo.trailheads
        .iter()
        .map(|&trailhead| topo.n_reachable(trailhead, &mut Some(HashSet::new())))
        .sum::<u32>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let topo = Topo::parse(input);
    topo.trailheads
        .iter()
        .map(|&trailhead| topo.n_reachable(trailhead, &mut None))
        .sum::<u32>()
}
