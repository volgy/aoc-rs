use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut lines = input.lines();
    let towels =
        Regex::new(format!("^({})*$", lines.next().unwrap().replace(", ", "|")).as_str()).unwrap();
    lines
        .skip(1)
        .filter(|design| towels.is_match(design))
        .count()
}

fn ways<'a>(design: &'a str, towel_options: &[&str], cache: &mut HashMap<&'a str, usize>) -> usize {
    if let Some(&count) = cache.get(design) {
        return count;
    }

    let count = if design.is_empty() {
        1
    } else {
        towel_options
            .iter()
            .filter(|&&towel| design.starts_with(towel))
            .map(|&towel| ways(&design[towel.len()..], towel_options, cache))
            .sum()
    };

    cache.insert(design, count);
    count
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut lines = input.lines();
    let towel_options = lines.next().unwrap().split(", ").collect_vec();
    let mut cache = HashMap::new();
    lines
        .skip(1)
        .map(|design| ways(design, &towel_options, &mut cache))
        .sum::<usize>()
}
