aoc::parts!(1);

use itertools::Itertools;

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

    fn arrangements(&self) -> Vec<Vec<char>> {
        let mut arrangements = vec![];
        let n_wildcards = self.pattern.iter().filter(|c| **c == '?').count();
        for comb in (0..n_wildcards)
            .map(|_| ['.', '#'].iter())
            .multi_cartesian_product()
        {
            let mut comb = comb.into_iter();
            let arrangement: Vec<_> = self
                .pattern
                .iter()
                .map(|c| if *c == '?' { *comb.next().unwrap() } else { *c })
                .collect();
            if Self::calc_rle(&arrangement) == self.rle {
                arrangements.push(arrangement);
            }
        }
        arrangements
    }

    fn calc_rle(seq: &[char]) -> Vec<usize> {
        let mut rle = vec![];

        let mut count = 0;
        for ch in seq {
            match ch {
                '#' => count += 1,
                '.' if count == 0 => {}
                '.' => {
                    rle.push(count);
                    count = 0;
                }
                _ => panic!("invalid seq: {seq:?}"),
            }
        }
        if count != 0 {
            rle.push(count);
        }
        rle
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(|l| Record::parse(l).arrangements().len())
        .sum::<usize>()
}

// fn part_2(input: aoc::Input) -> impl ToString {
//     0
// }
