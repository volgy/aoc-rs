aoc::parts!(1, 2);

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

trait HandTrait: Ord {
    fn new(hand: &str) -> Self;
    fn kind(&self) -> HandKind;
}

mod part1;
mod part2;

fn evaluate<T: HandTrait>(input: aoc::Input) -> u32 {
    input
        .lines()
        .map(|s| {
            let (hand, bid) = s.split_once(' ').unwrap();
            (T::new(hand), bid.parse::<u32>().unwrap())
        })
        .sorted()
        .rev()
        .enumerate()
        .fold(0, |acc, (rank, (_, bid))| acc + (rank as u32 + 1) * bid)
}

fn part_1(input: aoc::Input) -> impl ToString {
    evaluate::<part1::Hand>(input)
}

fn part_2(input: aoc::Input) -> impl ToString {
    evaluate::<part2::Hand>(input)
}
