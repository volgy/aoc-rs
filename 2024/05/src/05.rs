use itertools::Itertools;
use std::{cmp::Ordering, collections::HashSet};

aoc::parts!(1, 2);

fn parse_input(input: aoc::Input) -> (HashSet<(&str, &str)>, Vec<Vec<&str>>) {
    let mut lines = input.lines();
    let before: HashSet<_> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split_once("|").unwrap())
        .collect();

    let updates: Vec<_> = lines.map(|line| line.split(",").collect_vec()).collect();

    (before, updates)
}

fn part_1(input: aoc::Input) -> impl ToString {
    let (before, updates) = parse_input(input);

    updates
        .into_iter()
        .filter(|update| {
            update
                .iter()
                .tuple_windows()
                .all(|(p, n)| before.contains(&(p, n)))
        })
        .map(|update| update[update.len() / 2].parse::<i32>().unwrap())
        .sum::<i32>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let (before, updates) = parse_input(input);
    updates
        .into_iter()
        .filter(|update| {
            update
                .iter()
                .tuple_windows()
                .any(|(p, n)| !before.contains(&(p, n)))
        })
        .map(|mut update| {
            update.sort_by(|a, b| {
                if before.contains(&(a, b)) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            update
        })
        .map(|update| update[update.len() / 2].parse::<i32>().unwrap())
        .sum::<i32>()
}
