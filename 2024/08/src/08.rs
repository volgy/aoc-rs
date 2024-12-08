use itertools::Itertools;
use std::collections::{HashMap, HashSet};

aoc::parts!(1, 2);

type Pos = (isize, isize);

#[derive(Debug)]
struct City {
    extent: Pos,
    antennas: HashMap<char, Vec<Pos>>,
}

impl City {
    fn parse(input: aoc::Input) -> Self {
        let mut extent = (0, 0);
        let mut antennas: HashMap<_, Vec<_>> = HashMap::new();

        input.lines().enumerate().for_each(|(y, l)| {
            l.chars().enumerate().for_each(|(x, ch)| {
                let pos = (x as isize, y as isize);
                extent = (extent.0.max(pos.0 + 1), extent.1.max(pos.1 + 1));
                if ch != '.' {
                    antennas.entry(ch).or_default().push(pos);
                }
            });
        });

        Self { extent, antennas }
    }

    fn antinodes(&self, brightrf: impl Fn(Pos, Pos) -> Vec<Pos>) -> HashSet<Pos> {
        self.antennas
            .values()
            .flat_map(|group| {
                group
                    .iter()
                    .tuple_combinations()
                    .flat_map(|(&a, &b)| brightrf(a, b))
            })
            .filter(|pos| {
                (0..self.extent.0).contains(&pos.0) && (0..self.extent.1).contains(&pos.1)
            })
            .collect()
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    City::parse(input)
        .antinodes(|a, b| {
            vec![
                (2 * a.0 - b.0, 2 * a.1 - b.1),
                (2 * b.0 - a.0, 2 * b.1 - a.1),
            ]
        })
        .len()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let city = City::parse(input);
    city.antinodes(|a, b| {
        let d = ((b.0 - a.0), (b.1 - a.1));
        ((a.0 % d.0.abs())..(city.extent.0))
            .step_by(d.0.unsigned_abs())
            .map(|x| (x, a.1 - (a.0 - x) / d.0 * d.1))
            .collect_vec()
    })
    .len()
}
