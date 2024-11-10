aoc::parts!(1, 2);

use std::{
    cmp::{max, Ordering},
    collections::HashMap,
};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct Balls {
    red: u32,
    green: u32,
    blue: u32,
}

impl PartialOrd for Balls {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else if self.red >= other.red && self.green >= other.green && self.blue >= other.blue {
            Some(Ordering::Greater)
        } else if self.red <= other.red && self.green <= other.green && self.blue <= other.blue {
            Some(Ordering::Less)
        } else {
            None
        }
    }
}

/// Return the minimum number of balls per game
fn min_balls_per_game(input: aoc::Input) -> HashMap<u32, Balls> {
    let mut result = HashMap::new();

    for line in input.lines() {
        let (game_id, game) = line
            .strip_prefix("Game ")
            .unwrap()
            .split_once(": ")
            .unwrap();
        let game_id: u32 = game_id.parse().unwrap();
        let mut min_balls = Balls::default();
        for draw_str in game.split("; ") {
            let mut draw = Balls::default();
            for ball_str in draw_str.split(", ") {
                let (count, color) = ball_str.split_once(' ').unwrap();
                let count: u32 = count.parse().unwrap();
                match color {
                    "red" => draw.red = count,
                    "green" => draw.green = count,
                    "blue" => draw.blue = count,
                    _ => panic!("invalid color"),
                }
            }
            min_balls.red = max(min_balls.red, draw.red);
            min_balls.green = max(min_balls.green, draw.green);
            min_balls.blue = max(min_balls.blue, draw.blue);
        }

        result.insert(game_id, min_balls);
    }
    result
}

fn part_1(input: aoc::Input) -> impl ToString {
    let limits = Balls {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut score = 0;
    for (game_id, min_balls) in min_balls_per_game(input).iter() {
        if limits >= *min_balls {
            score += game_id;
        }
    }
    score
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut score = 0;
    for min_balls in min_balls_per_game(input).values() {
        score += min_balls.red * min_balls.green * min_balls.blue;
    }
    score
}
