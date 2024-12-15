use std::collections::HashSet;

use deflate::deflate_bytes;
use itertools::Itertools;
use regex::Regex;

aoc::parts!(1, 2);

type Vec2D = (isize, isize);

#[derive(Debug)]
struct Robot {
    pos: Vec2D,
    vel: Vec2D,
}

#[derive(Debug)]
struct Restroom {
    robots: Vec<Robot>,
    size: Vec2D,
}

impl Restroom {
    fn parse(input: aoc::Input) -> Self {
        let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
        let mut size = (0, 0);

        let robots: Vec<_> = input
            .lines()
            .map(|line| {
                let captures = re.captures(line).unwrap();
                let mut values = captures
                    .iter()
                    .skip(1)
                    .map(|m| m.unwrap().as_str().parse().unwrap());

                let pos = (values.next().unwrap(), values.next().unwrap());
                let vel = (values.next().unwrap(), values.next().unwrap());
                size = (size.0.max(pos.0 + 1), size.1.max(pos.1 + 1));
                Robot { pos, vel }
            })
            .collect();

        Self { robots, size }
    }

    fn safety_factor(&self, seconds: isize) -> usize {
        let (div_x, div_y) = (self.size.0 / 2, self.size.1 / 2);
        self.robots
            .iter()
            .map(|r| {
                (
                    (r.pos.0 + r.vel.0 * seconds).rem_euclid(self.size.0),
                    (r.pos.1 + r.vel.1 * seconds).rem_euclid(self.size.1),
                )
            })
            .filter(|&(x, y)| x != div_x && y != div_y)
            .into_group_map_by(|&(x, y)| (x < div_x, y < div_y))
            .values()
            .map(|v| v.len())
            .product()
    }

    fn render(&self, seconds: isize) -> String {
        let locs: HashSet<_> = self
            .robots
            .iter()
            .map(|r| {
                (
                    (r.pos.0 + r.vel.0 * seconds).rem_euclid(self.size.0),
                    (r.pos.1 + r.vel.1 * seconds).rem_euclid(self.size.1),
                )
            })
            .collect();

        (0..self.size.1)
            .flat_map(|y| {
                let locs = &locs;
                (0..self.size.0)
                    .map(move |x| if locs.contains(&(x, y)) { '#' } else { ' ' })
                    .chain("\n".chars())
            })
            .collect()
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    Restroom::parse(input).safety_factor(100)
}

fn part_2(input: aoc::Input) -> impl ToString {
    let restroom = Restroom::parse(input);
    let mut compressed_min = usize::MAX;
    let mut n_iter_min = 0;

    for n_iter in 0..10_000 {
        let image = restroom.render(n_iter);
        let compressed_size = deflate_bytes(image.as_bytes()).len();

        if compressed_size < compressed_min {
            println!("{}\niteration: {}\n", image, n_iter);
            compressed_min = compressed_size;
            n_iter_min = n_iter;
        }
    }

    n_iter_min
}
