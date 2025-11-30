use aoc::{IterUnwrap, Parse};
use std::{collections::HashMap, hash::Hash, num::ParseIntError, str::FromStr};

aoc::parts!(1, 2);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Player {
    pos: u64,
    score: u64,
}

impl Player {
    fn advance_by(&mut self, steps: u64) {
        self.pos = ((self.pos - 1 + steps) % 10) + 1;
        self.score += self.pos;
    }
}

impl FromStr for Player {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pos: u64 = s.as_parser().after("starting position: ").parse()?;
        Ok(Self { pos, score: 0 })
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut players: Vec<Player> = input.lines().map(|l| l.parse().unwrap()).collect();

    let mut die = (1..=100).cycle().enumerate();
    'game: loop {
        for player in players.iter_mut() {
            let steps: u64 = die.by_ref().take(3).map(|(_, x)| x).sum();
            player.advance_by(steps);
            if player.score >= 1000 {
                break 'game;
            }
        }
    }
    let rolls = die.next_uw().0 as u64;
    rolls * players.iter().map(|p| p.score).min().unwrap()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let lines = input.as_lines();
    let p1 = lines[0].parse().unwrap();
    let p2 = lines[1].parse().unwrap();

    // Map of (player1, player2) -> count of universes in that state
    let mut universes: HashMap<(Player, Player), u128> = HashMap::new();
    universes.insert((p1, p2), 1);

    // wins[0] for player1, wins[1] for player2
    let mut wins = [0u128, 0u128];

    // Dirac dice roll sums and their multiplicities
    let dirac: [(u64, u128); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    // 0 = player1's turn, 1 = player2's turn
    let mut turn = 0usize;

    while !universes.is_empty() {
        let mut next: HashMap<(Player, Player), u128> = HashMap::new();

        for ((a, b), count) in universes.into_iter() {
            for (steps, mult) in dirac.iter() {
                let mut p = [a, b];
                p[turn].advance_by(*steps);
                let ways = count * (*mult);
                if p[turn].score >= 21 {
                    wins[turn] += ways;
                } else {
                    *next.entry((p[0], p[1])).or_default() += ways;
                }
            }
        }

        universes = next;
        turn ^= 1;
    }

    wins.iter().max().unwrap().to_string()
}
