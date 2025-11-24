use itertools::Itertools;
use std::collections::HashMap;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let unique_lengths = [2, 3, 4, 7];
    input
        .lines()
        .map(|line| {
            let (_, four) = line.split_once(" | ").unwrap();
            four.split_whitespace()
                .map(str::len)
                .filter(|len| unique_lengths.contains(len))
                .count()
        })
        .sum::<usize>()
}

#[derive(Debug)]
struct Wiring(HashMap<char, char>);

impl Wiring {
    fn build(digits: Vec<&str>) -> Self {
        let segments = "abcdefg";
        segments
            .chars()
            .permutations(segments.len())
            .find_map(|hypothesis| {
                let map: HashMap<_, _> = segments.chars().zip(hypothesis).collect();
                let wiring = Self(map);
                digits
                    .iter()
                    .all(|pattern| wiring.decode(pattern).is_some())
                    .then_some(wiring)
            })
            .unwrap()
    }

    fn decode(&self, pattern: &str) -> Option<u32> {
        // kind of wasteful to do the sorting here
        let mapped: String = pattern.chars().map(|ch| self.0[&ch]).sorted().collect();
        match mapped.as_str() {
            "abcefg" => Some(0),
            "cf" => Some(1),
            "acdeg" => Some(2),
            "acdfg" => Some(3),
            "bcdf" => Some(4),
            "abdfg" => Some(5),
            "abdefg" => Some(6),
            "acf" => Some(7),
            "abcdefg" => Some(8),
            "abcdfg" => Some(9),
            _ => None,
        }
    }
}

fn part_2(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(|line| {
            let (digits, four) = line.split_once(" | ").unwrap();
            let wiring = Wiring::build(digits.split_whitespace().collect());
            four.split_whitespace()
                .map(|pattern| wiring.decode(pattern).unwrap())
                .rev()
                .enumerate()
                .map(|(i, value)| 10u32.pow(i as u32) * value)
                .sum::<u32>()
        })
        .sum::<u32>()
}
