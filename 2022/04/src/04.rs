use std::cmp::Ordering;

use itertools::Itertools;

aoc::parts!(1, 2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Assignment {
    start: usize,
    end: usize,
}

impl Assignment {
    fn parse(text: &str) -> Self {
        let mut parts = text.split('-');
        let start = parts.next().unwrap().parse().unwrap();
        let end = parts.next().unwrap().parse().unwrap();
        Self { start, end }
    }

    fn overlaps(&self, other: &Self) -> bool {
        !(self.end < other.start || other.end < self.start)
    }
}

impl PartialOrd for Assignment {
    // Assignment is greater than other assignment if it fully contains it
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.start <= other.start && self.end >= other.end {
            Some(Ordering::Greater)
        } else if self.start >= other.start && self.end <= other.end {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut contained_pairs = 0;
    for line in input.lines() {
        let (left, right) = line
            .split(',')
            .map(Assignment::parse)
            .collect_tuple()
            .unwrap();
        if left < right || left > right {
            contained_pairs += 1;
        }
    }
    contained_pairs
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut overlapping_pairs = 0;
    for line in input.lines() {
        let (left, right) = line
            .split(',')
            .map(Assignment::parse)
            .collect_tuple()
            .unwrap();
        if left.overlaps(&right) {
            overlapping_pairs += 1;
        }
    }
    overlapping_pairs
}
