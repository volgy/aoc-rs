aoc::parts!(1, 2);

use std::collections::{HashMap, HashSet};

fn count_wins(card: &str) -> u64 {
    let (winners, numbers) = card.split_once(':').unwrap().1.split_once('|').unwrap();
    let winners: HashSet<_> = winners.split_whitespace().collect();
    numbers
        .split_whitespace()
        .filter(|n| winners.contains(n))
        .count() as u64
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut value = 0;
    for line in input.lines() {
        let wins = count_wins(line);
        if wins > 0 {
            value += 1 << (wins - 1);
        }
    }
    value
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut deck = HashMap::new();

    for (i_current, line) in input.lines().enumerate() {
        let n_current = deck.entry(i_current).or_insert(0);
        *n_current += 1;
        let multiplier = *n_current;
        let win_range = count_wins(line) as usize;

        for idx in (i_current + 1)..=(i_current + win_range) {
            *deck.entry(idx).or_insert(0) += multiplier;
        }
    }

    deck.values().sum::<u32>()
}
