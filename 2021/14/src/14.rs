use itertools::Itertools;
use std::{collections::HashMap, iter::once};
aoc::parts!(1, 2);

fn parse(input: aoc::Input) -> (String, HashMap<(char, char), char>) {
    let mut lines = input.lines();
    let polymer = lines.next().unwrap().to_owned();
    lines.next(); // skip blank line

    let rules = lines
        .map(|l| {
            let (lhs, rhs) = l.split_once(" -> ").unwrap();

            (
                lhs.chars().tuple_windows().next().unwrap(),
                rhs.chars().next().unwrap(),
            )
        })
        .collect();

    (polymer, rules)
}

fn part_1(input: aoc::Input) -> impl ToString {
    let (mut polymer, rules) = parse(input);

    // Brute force it for 10 iterations
    for _ in 0..10 {
        let insertions = polymer
            .chars()
            .tuple_windows()
            .map(|pair| rules.get(&pair).copied())
            .chain(once(None))
            .collect::<Vec<_>>();

        polymer = polymer
            .chars()
            .zip(insertions)
            .flat_map(|(a, b)| [Some(a), b])
            .flatten()
            .collect::<String>();
    }

    let mut counts = HashMap::new();
    for c in polymer.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let (polymer, rules) = parse(input);
    let mut pair_freq: HashMap<(char, char), usize> = HashMap::new();
    for pair in polymer.chars().tuple_windows::<(char, char)>() {
        *pair_freq.entry(pair).or_default() += 1;
    }

    let mut counts: HashMap<char, usize> = HashMap::new();
    for c in polymer.chars() {
        *counts.entry(c).or_default() += 1;
    }

    for _ in 0..40 {
        let mut new_pair_freq = HashMap::new();

        for (pair, freq) in pair_freq.into_iter() {
            if let Some(&ins) = rules.get(&pair) {
                *new_pair_freq.entry((pair.0, ins)).or_default() += freq;
                *new_pair_freq.entry((ins, pair.1)).or_default() += freq;
                *counts.entry(ins).or_default() += freq;
            } else {
                *new_pair_freq.entry(pair).or_default() += freq;
            }
        }

        pair_freq = new_pair_freq;
    }

    counts.values().max().unwrap() - counts.values().min().unwrap()
}
