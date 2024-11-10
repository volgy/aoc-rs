aoc::parts!(1, 2);

use std::{cmp::Ordering, ops::Add};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Interval {
    start: u64,
    end: u64, // non-inclusive
}

impl Interval {
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    fn overlap(&self, other: &Self) -> Option<Self> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);
        if end > start {
            Some(Self { start, end })
        } else {
            None
        }
    }

    fn is_empty(&self) -> bool {
        self.end <= self.start
    }
}

impl Add<i128> for Interval {
    type Output = Self;

    fn add(self, offset: i128) -> Self {
        Interval::new(
            (self.start as i128 + offset) as u64,
            (self.end as i128 + offset) as u64,
        )
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.end <= other.start {
            Some(Ordering::Less)
        } else if self.start >= other.end {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

type IntervalMap = Vec<(Interval, i128)>;

#[derive(Debug)]
struct Almanac {
    seeds: Vec<Interval>,
    maps: Vec<IntervalMap>,
}

impl Almanac {
    fn parse(input: aoc::Input, as_intervals: bool) -> Self {
        let mut lines = input.lines();

        let raw_seeds = lines
            .next()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap());

        let seeds = if as_intervals {
            raw_seeds
                .tuples()
                .map(|(s, l)| Interval::new(s, s + l))
                .collect_vec()
        } else {
            raw_seeds.map(|s| Interval::new(s, s + 1)).collect_vec()
        };

        let mut maps = Vec::new();
        for line in lines {
            if line.is_empty() {
                continue;
            }
            if line.contains("map:") {
                maps.push(IntervalMap::new());
            } else {
                let interval_map = line
                    .split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .tuples()
                    .map(|(dst, src, len)| {
                        (Interval::new(src, src + len), dst as i128 - src as i128)
                    })
                    .next()
                    .unwrap();

                maps.last_mut().unwrap().push(interval_map);
            }
        }

        // Assuming that the each map does not have overlapping regions
        maps.iter_mut()
            .for_each(|m| m.sort_by(|a, b| a.partial_cmp(b).unwrap()));

        Self { seeds, maps }
    }

    fn map_interval(interval: &Interval, map: &IntervalMap) -> Vec<Interval> {
        let mut interval = *interval;
        let mut mapped = Vec::new();
        for (src, offset) in map {
            if let Some(overlap) = interval.overlap(src) {
                if interval.start < overlap.start {
                    mapped.push(Interval::new(interval.start, overlap.start));
                    interval.start = overlap.start;
                }
                interval.start = overlap.end;
                mapped.push(overlap + *offset);
            }
        }

        if !interval.is_empty() {
            mapped.push(interval);
        }
        mapped
    }

    fn all_mapped(&self) -> Vec<Interval> {
        let mut intervals = self.seeds.clone();
        for map in &self.maps {
            let mut new_intervals = Vec::new();
            for interval in intervals {
                new_intervals.extend(Self::map_interval(&interval, map));
            }
            intervals = new_intervals;
        }
        intervals
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let almanac = Almanac::parse(input, false);
    almanac
        .all_mapped()
        .into_iter()
        .map(|r| r.start)
        .min()
        .unwrap()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let almanac = Almanac::parse(input, true);
    almanac
        .all_mapped()
        .into_iter()
        .map(|r| r.start)
        .min()
        .unwrap()
}
