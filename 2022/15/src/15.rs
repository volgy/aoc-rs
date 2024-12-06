use std::collections::HashSet;

use regex::Regex;

aoc::parts!(1, 2);

type Coord = (i64, i64);

#[inline]
fn manhattan(pos1: Coord, pos2: Coord) -> i64 {
    (pos2.0 - pos1.0).abs() + (pos2.1 - pos1.1).abs()
}

#[derive(Debug, Clone, Copy)]
struct Sensor {
    pos: Coord,
    beacon: Coord,
}

impl Sensor {
    fn new(pos: Coord, beacon: Coord) -> Self {
        Self { pos, beacon }
    }

    fn range(&self) -> i64 {
        manhattan(self.pos, self.beacon)
    }
}

#[derive(Debug)]
struct Field {
    sensors: Vec<Sensor>,
}

impl Field {
    fn parse(input: aoc::Input) -> Self {
        let mut sensors = vec![];
        let re = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$",
        )
        .unwrap();

        for line in input.lines() {
            let captures = re.captures(line).unwrap();
            sensors.push(Sensor::new(
                (captures[1].parse().unwrap(), captures[2].parse().unwrap()),
                (captures[3].parse().unwrap(), captures[4].parse().unwrap()),
            ));
        }
        Self { sensors }
    }

    // set-based brute approach
    fn coverage(&self, y: i64) -> usize {
        let mut covered = HashSet::new();
        for sensor in self.sensors.iter() {
            let range = sensor.range() - (y - sensor.pos.1).abs();
            if range < 1 {
                continue;
            }
            covered.extend((sensor.pos.0 - range)..=(sensor.pos.0 + range));
        }

        // remove beacons
        for sensor in self.sensors.iter() {
            if sensor.beacon.1 == y {
                covered.remove(&sensor.beacon.0);
            }
        }

        covered.len()
    }

    // iterval-based approach
    fn find_uncovered(&self, extent: i64) -> Option<Coord> {
        let mut intervals = vec![]; // allocate only once
        for y in 0..=extent {
            intervals.clear();
            for sensor in self.sensors.iter() {
                let range = sensor.range() - (y - sensor.pos.1).abs();
                intervals.push((sensor.pos.0 - range, sensor.pos.0 + range));
            }
            intervals.sort();
            let mut x = 0;
            let mut intervals_it = intervals.iter();
            while x <= extent {
                if let Some(&(start, end)) = intervals_it.next() {
                    if start > x {
                        return Some((x, y));
                    }
                    x = x.max(end + 1);
                } else {
                    return Some((x, y));
                }
            }
        }
        None
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let field = Field::parse(input);
    // field.coverage(10) // for example data
    field.coverage(2000000)
}

fn part_2(input: aoc::Input) -> impl ToString {
    const EXTENT: i64 = 4_000_000;
    let field = Field::parse(input);
    // let spot = field.find_uncovered(20).unwrap(); // for example data
    let spot = field.find_uncovered(EXTENT).unwrap();
    spot.0 * EXTENT + spot.1
}
