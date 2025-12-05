use itertools::{intersperse, repeat_n};
use std::collections::HashMap;

aoc::parts!(1, 2);

#[derive(Debug)]
struct Record {
    pattern: Vec<char>,
    rle: Vec<usize>,
}

impl Record {
    fn parse(line: &str) -> Self {
        let (pattern_str, rle_str) = line.split_once(' ').unwrap();
        let pattern: Vec<_> = pattern_str.chars().collect();
        let rle: Vec<_> = rle_str.split(',').map(|s| s.parse().unwrap()).collect();
        Self { pattern, rle }
    }

    fn unfold(&self) -> Self {
        let pattern = intersperse(repeat_n(self.pattern.clone(), 5), vec!['?'])
            .flatten()
            .collect();
        let rle = repeat_n(self.rle.clone(), 5).flatten().collect();
        Self { pattern, rle }
    }

    fn matches(&self) -> usize {
        let mut cache = HashMap::new();

        fn solve(
            pattern: &[char],
            rle: &[usize],
            pi: usize,
            ri: usize,
            cache: &mut HashMap<(usize, usize), usize>,
        ) -> usize {
            if let Some(&result) = cache.get(&(pi, ri)) {
                return result;
            }

            if ri == rle.len() {
                let valid = pattern[pi..].iter().all(|&c| c != '#');
                return if valid { 1 } else { 0 };
            }

            if pi >= pattern.len() {
                return 0;
            }

            let mut result = 0;
            let group_len = rle[ri];

            if pattern[pi] == '.' || pattern[pi] == '?' {
                result += solve(pattern, rle, pi + 1, ri, cache);
            }

            if pattern[pi] == '#' || pattern[pi] == '?' {
                if pi + group_len <= pattern.len() {
                    let can_place = pattern[pi..pi + group_len].iter().all(|&c| c != '.');
                    let followed_by_dot_or_end =
                        pi + group_len == pattern.len() || pattern[pi + group_len] != '#';

                    if can_place && followed_by_dot_or_end {
                        let next_pi = (pi + group_len + 1).min(pattern.len());
                        result += solve(pattern, rle, next_pi, ri + 1, cache);
                    }
                }
            }

            cache.insert((pi, ri), result);
            result
        }

        solve(&self.pattern, &self.rle, 0, 0, &mut cache)
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(|l| Record::parse(l).matches())
        .sum::<usize>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(|l| Record::parse(l).unfold().matches())
        .sum::<usize>()
}
