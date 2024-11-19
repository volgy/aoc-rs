use std::{
    i8,
    ops::{Index, IndexMut},
};

use itertools::iproduct;

aoc::parts!(1, 2);

#[derive(Debug, Clone)]
struct Grid {
    width: usize,
    height: usize,
    inner: Vec<i8>,
}

enum Direction {
    LeftToRight,
    RightToLeft,
    TopToDown,
    DownToTop,
}

impl Grid {
    fn from_input(input: aoc::Input) -> Self {
        let mut rows: Vec<Vec<_>> = vec![];
        for line in input.lines() {
            rows.push(
                line.chars()
                    .map(|ch| ch.to_digit(10).unwrap() as i8)
                    .collect(),
            );
        }

        let height = rows.len();
        assert!(height > 0);
        let width = rows[0].len();

        let inner: Vec<_> = rows.into_iter().flatten().collect();
        assert_eq!(inner.len(), width * height);

        Self {
            width,
            height,
            inner,
        }
    }

    fn obstruction_map(&self, direction: Direction) -> Self {
        let mut result = self.clone();

        match direction {
            Direction::LeftToRight | Direction::RightToLeft => {
                // Row-major iteration
                for j in 0..self.height {
                    let mut max = i8::MIN;
                    let i_range: Box<dyn Iterator<Item = usize>> = match direction {
                        Direction::LeftToRight => Box::new(0..self.width),
                        Direction::RightToLeft => Box::new((0..self.width).rev()),
                        _ => unreachable!(),
                    };

                    for i in i_range {
                        result[(i, j)] = max;
                        max = self[(i, j)].max(max);
                    }
                }
            }
            Direction::TopToDown | Direction::DownToTop => {
                // Column-major iteration
                for i in 0..self.width {
                    let mut max = i8::MIN;
                    let j_range: Box<dyn Iterator<Item = usize>> = match direction {
                        Direction::TopToDown => Box::new(0..self.height),
                        Direction::DownToTop => Box::new((0..self.height).rev()),
                        _ => unreachable!(),
                    };

                    for j in j_range {
                        result[(i, j)] = max;
                        max = self[(i, j)].max(max);
                    }
                }
            }
        }

        result
    }

    // naive approach
    fn scenic_score(&self, loc: Coords) -> usize {
        if loc.0 == 0 || loc.0 == self.width - 1 || loc.1 == 0 || loc.1 == self.height - 1 {
            return 0;
        }

        let h = self[loc];
        let (x, y) = loc;

        let view_distance = |trees| {
            let mut count = 0;
            for tree in trees {
                count += 1;
                if tree >= h {
                    break;
                }
            }
            count
        };

        // Look in all four directions using iterators
        let right = view_distance((x + 1..self.width).map(|i| self[(i, y)]));
        let left = view_distance((0..x).rev().map(|i| self[(i, y)]));
        let down = view_distance((y + 1..self.height).map(|j| self[(x, j)]));
        let up = view_distance((0..y).rev().map(|j| self[(x, j)]));

        right * left * down * up
    }
}

type Coords = (usize, usize);

impl Index<Coords> for Grid {
    type Output = i8;
    fn index(&self, index: Coords) -> &Self::Output {
        &self.inner[index.0 + index.1 * self.width]
    }
}

impl IndexMut<Coords> for Grid {
    fn index_mut(&mut self, index: Coords) -> &mut Self::Output {
        &mut self.inner[index.0 + index.1 * self.width]
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let grid = Grid::from_input(input);
    //eprintln!("grid = {:#?}", grid);

    let obs_maps = vec![
        grid.obstruction_map(Direction::LeftToRight),
        grid.obstruction_map(Direction::RightToLeft),
        grid.obstruction_map(Direction::TopToDown),
        grid.obstruction_map(Direction::DownToTop),
    ];

    let mut visible_cnt = 0;
    for i in 0..grid.width {
        for j in 0..grid.height {
            for obs_map in obs_maps.iter() {
                if obs_map[(i, j)] < grid[(i, j)] {
                    visible_cnt += 1;
                    break;
                }
            }
        }
    }
    visible_cnt
}

fn part_2(input: aoc::Input) -> impl ToString {
    let grid = Grid::from_input(input);
    //eprintln!("score(2, 1) = {:#?}", grid.scenic_score((2, 1)));
    iproduct!(0..grid.width, 0..grid.height)
        .map(|loc| grid.scenic_score(loc))
        .max()
        .unwrap()
}
