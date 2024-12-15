use itertools::Itertools;
use std::ops::{Add, Mul};

use aoc::Parse;

aoc::parts!(1, 2);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Vec2D(isize, isize);

impl Vec2D {
    fn new(x: isize, y: isize) -> Self {
        Self(x, y)
    }
}

impl Add for Vec2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<isize> for Vec2D {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

#[derive(Debug, Copy, Clone)]
struct Machine {
    a: Vec2D,
    b: Vec2D,
    target: Vec2D,
}

impl Machine {
    fn parse(lines: &[&str]) -> Self {
        let mut buttons = lines[..2]
            .iter()
            .map(|line| line.as_parser())
            .map(|mut parser| {
                let x: isize = parser.between("X+", ",").parse().unwrap();
                let y: isize = parser.after("Y+").parse().unwrap();
                Vec2D::new(x, y)
            });

        let mut parser = lines[2].as_parser();
        let target = Vec2D::new(
            parser.between("X=", ",").parse().unwrap(),
            parser.after("Y=").parse().unwrap(),
        );

        Self {
            a: buttons.next().unwrap(),
            b: buttons.next().unwrap(),
            target,
        }
    }

    fn min_cost(&self) -> Option<isize> {
        // works if only one solution exists (did we get lucky?)
        let det_a = self.target.0 * self.b.1 - self.target.1 * self.b.0;
        let det_b = self.target.1 * self.a.0 - self.target.0 * self.a.1;
        let det = self.a.0 * self.b.1 - self.a.1 * self.b.0;

        if det_a % det == 0 && det_b % det == 0 {
            Some(3 * det_a / det + det_b / det)
        } else {
            None
        }
    }
}

fn solve(input: aoc::Input, offset: isize) -> isize {
    let machines = input
        .lines()
        .chunks(4)
        .into_iter()
        .map(|lines| {
            let mut machine = Machine::parse(&lines.collect_vec());
            machine.target = machine.target + Vec2D::new(offset, offset);
            machine
        })
        .collect_vec();
    machines.iter().filter_map(|m| m.min_cost()).sum::<isize>()
}

fn part_1(input: aoc::Input) -> impl ToString {
    solve(input, 0)
}

fn part_2(input: aoc::Input) -> impl ToString {
    solve(input, 10_000_000_000_000)
}
