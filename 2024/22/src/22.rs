use itertools::Itertools;
use std::{collections::HashMap, iter::successors};

aoc::parts!(1, 2);

fn secrets(seed: u64) -> impl Iterator<Item = u64> {
    successors(Some(seed), |value| {
        let step1 = (value ^ (value << 6)) & 0xFFFFFF;
        let step2 = (step1 ^ (step1 >> 5)) & 0xFFFFFF;
        Some((step2 ^ (step2 << 11)) & 0xFFFFFF)
    })
}

fn part_1(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(|l| secrets(l.parse().unwrap()).nth(2000).unwrap())
        .sum::<u64>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut sum_first_prices = HashMap::new();
    for secrets in input
        .lines()
        .map(|l| secrets(l.parse().unwrap()).map(|secret| (secret % 10) as i8))
    {
        let mut first_prices = HashMap::new();
        for (deltas, price) in secrets
            .take(2000)
            .tuple_windows()
            .map(|(prev, next)| (next - prev, next))
            .tuple_windows()
            .map(|((d1, _), (d2, _), (d3, _), (d4, price))| ((d1, d2, d3, d4), price))
        {
            first_prices.entry(deltas).or_insert(price);
        }

        for (deltas, price) in first_prices {
            *sum_first_prices.entry(deltas).or_insert(0) += price as i64;
        }
    }

    *sum_first_prices.values().max().unwrap()
}
