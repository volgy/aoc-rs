use itertools::Itertools;
aoc::parts!(1, 2);

fn parse(input: aoc::Input) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let (mut left, mut right) = parse(input);
    left.sort();
    right.sort();

    left.iter()
        .zip(right)
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let (left, right) = parse(input);

    left.iter()
        .map(|l| l * right.iter().filter(|&r| r == l).count() as i32)
        .sum::<i32>()
}
