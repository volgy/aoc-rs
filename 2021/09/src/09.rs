use itertools::Itertools;
use std::{collections::HashSet, ops::Index};

aoc::parts!(1, 2);

struct Map {
    heights: Vec<Vec<u8>>,
    cols: usize,
    rows: usize,
}

type Pos = (usize, usize);

impl Map {
    fn from_lines<'a, I: Iterator<Item = &'a str>>(lines: I) -> Self {
        let heights = lines
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect_vec()
            })
            .collect_vec();
        let rows = heights.len();
        let cols = heights[0].len();

        Self {
            heights,
            cols,
            rows,
        }
    }

    fn neighbors(&self, pos: Pos) -> impl Iterator<Item = Pos> {
        let (i, j) = pos;
        [
            i.checked_sub(1).map(|ni| (ni, j)),
            (i + 1 < self.rows).then(|| (i + 1, j)),
            j.checked_sub(1).map(|nj| (i, nj)),
            (j + 1 < self.cols).then(|| (i, j + 1)),
        ]
        .into_iter()
        .flatten()
    }

    fn low_points(&self) -> Vec<Pos> {
        (0..self.rows)
            .cartesian_product(0..self.cols)
            .filter(|&pos| {
                let height = self[pos];
                self.neighbors(pos).all(|npos| height < self[npos])
            })
            .collect()
    }

    fn basin_size(&self, bottom: Pos) -> usize {
        let mut visited = HashSet::new();
        self.flood(&mut visited, bottom);
        visited.len()
    }

    fn flood(&self, visited: &mut HashSet<Pos>, pos: Pos) {
        if self[pos] == 9 || !visited.insert(pos) {
            return;
        }
        for npos in self.neighbors(pos) {
            self.flood(visited, npos);
        }
    }
}

impl Index<Pos> for Map {
    type Output = u8;

    fn index(&self, pos: Pos) -> &Self::Output {
        &self.heights[pos.0][pos.1]
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let map = Map::from_lines(input.lines());
    map.low_points()
        .into_iter()
        .map(|p| map[p] as u32 + 1)
        .sum::<u32>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let map = Map::from_lines(input.lines());
    map.low_points()
        .into_iter()
        .map(|p| map.basin_size(p))
        .sorted()
        .rev()
        .take(3)
        .product::<usize>()
}
