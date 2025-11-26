use std::collections::HashSet;

use aoc::IterUnwrap;
use itertools::Itertools;

aoc::parts!(1, 2);

const N: usize = 10;

#[derive(Debug)]
struct Octopuses([[u8; N]; N]);

impl Octopuses {
    fn from_lines(lines: &[&str]) -> Self {
        Self(
            lines
                .iter()
                .map(|l| l.bytes().map(|b| b - b'0').collect_n())
                .collect_n(),
        )
    }

    fn neighbors(&self, (x, y): (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|&(dx, dy)| dx != 0 || dy != 0)
            .filter_map(move |(dx, dy)| {
                let nx = x.checked_add_signed(dx)?;
                let ny = y.checked_add_signed(dy)?;
                (nx < N && ny < N).then_some((nx, ny))
            })
    }

    fn step(&mut self) -> usize {
        self.0.iter_mut().flatten().for_each(|o| *o += 1);

        let mut flashed = HashSet::new();
        loop {
            let mut changed = false;
            for (i, j) in (0..N).cartesian_product(0..N) {
                if self.0[i][j] > 9 && flashed.insert((i, j)) {
                    changed = true;
                    for (ni, nj) in self.neighbors((i, j)) {
                        self.0[ni][nj] += 1;
                    }
                }
            }
            if !changed {
                break;
            }
        }

        for &(i, j) in &flashed {
            self.0[i][j] = 0;
        }

        flashed.len()
    }
}
fn part_1(input: aoc::Input) -> impl ToString {
    let mut octopuses = Octopuses::from_lines(input.as_lines());
    (0..100).map(|_| octopuses.step()).sum::<usize>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut octopuses = Octopuses::from_lines(input.as_lines());
    (1..).find(|_| octopuses.step() == N * N).unwrap()
}
