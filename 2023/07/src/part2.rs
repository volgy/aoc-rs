use counter::Counter;
use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::HashMap;

use super::{HandKind, HandTrait};

lazy_static! {
    static ref LABEL_VALUE: HashMap<char, u32> = {
        "AKQT98765432J"
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
        let has_joker = self.0.contains('J');
        let is_joker = |idx: usize| cards[idx].0 == 'J';

        match (cards.len(), cards[0].1) {
            (1, _) => HandKind::FiveOfAKind,
            (2, 4) => {
                if has_joker {
                    HandKind::FiveOfAKind
                } else {
                    HandKind::FourOfAKind
                }
            }
            (2, _) => {
                if has_joker {
                    HandKind::FiveOfAKind
                } else {
                    HandKind::FullHouse
                }
            }
            (3, 3) => {
                if has_joker {
                    HandKind::FourOfAKind
                } else {
                    HandKind::ThreeOfAKind
                }
            }
            (3, _) => {
                if is_joker(0) || is_joker(1) {
                    HandKind::FourOfAKind
                } else if is_joker(2) {
                    HandKind::FullHouse
                } else {
                    HandKind::TwoPair
                }
            }

            (4, _) => {
                if has_joker {
                    HandKind::ThreeOfAKind
                } else {
                    HandKind::OnePair
                }
            }

            (..) => {
                if has_joker {
                    HandKind::OnePair
                } else {
                    HandKind::HighCard
                }
            }
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
