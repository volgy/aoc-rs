use counter::Counter;
use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::HashMap;

use super::{HandKind, HandTrait};

lazy_static! {
    static ref LABEL_VALUE: HashMap<char, u32> = {
        "AKQJT98765432"
            .chars()
            .enumerate()
            .map(|(i, l)| (l, i as u32))
            .collect()
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand(String);

impl HandTrait for Hand {
    fn new(hand: &str) -> Self {
        Self(hand.to_owned())
    }

    fn kind(&self) -> HandKind {
        let cards = self.0.chars().collect::<Counter<_>>().most_common_ordered();

        match (cards.len(), cards[0].1) {
            (1, _) => HandKind::FiveOfAKind,
            (2, 4) => HandKind::FourOfAKind,
            (2, _) => HandKind::FullHouse,
            (3, 3) => HandKind::ThreeOfAKind,
            (3, _) => HandKind::TwoPair,
            (4, _) => HandKind::OnePair,
            (..) => HandKind::HighCard,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind().cmp(&other.kind()) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self
                .0
                .chars()
                .map(|l| LABEL_VALUE[&l])
                .cmp(other.0.chars().map(|l| LABEL_VALUE[&l])),
        }
    }
}
