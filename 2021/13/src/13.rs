use std::collections::{HashSet, VecDeque};

use aoc::Parse;

aoc::parts!(1, 2);

#[derive(Debug)]
enum Fold {
    Left(usize),
    Up(usize),
}

struct Paper {
    dots: HashSet<(usize, usize)>,
    folds: VecDeque<Fold>,
}

impl Paper {
    fn from_lines<'a>(mut lines: impl Iterator<Item = &'a str>) -> Self {
        let dots = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .map(|l| {
                let (x, y) = l.split_once(',').unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect();

        let folds = lines
            .map(|l| {
                let (axis, pos) = l.as_parser().after("fold along ").split_once('=').unwrap();
                let pos: usize = pos.parse().unwrap();
                match axis {
                    "x" => Fold::Left(pos),
                    "y" => Fold::Up(pos),
                    _ => unreachable!(),
                }
            })
            .collect();
        Self { dots, folds }
    }

    fn n_dots(&self) -> usize {
        self.dots.len()
    }

    fn fold_once(&mut self) {
        let fold = self.folds.pop_front().unwrap();
        self.dots = self
            .dots
            .iter()
            .map(|&(x, y)| match fold {
                Fold::Left(pos) => (x.min(2 * pos - x), y),
                Fold::Up(pos) => (x, y.min(2 * pos - y)),
            })
            .collect();
    }

    fn fold_all(&mut self) {
        while !self.folds.is_empty() {
            self.fold_once();
        }
    }

    fn render(&self) {
        let (size_x, size_y) = self.dots.iter().fold((0, 0), |(size_x, size_y), &(x, y)| {
            (size_x.max(x), size_y.max(y))
        });
        for y in 0..=size_y {
            for x in 0..=size_x {
                let ch = if self.dots.contains(&(x, y)) {
                    '#'
                } else {
                    '.'
                };
                print!("{}", ch);
            }
            println!();
        }
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut paper = Paper::from_lines(input.lines());
    paper.fold_once();
    paper.n_dots()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut paper = Paper::from_lines(input.lines());
    paper.fold_all();
    paper.render();
    0
}
