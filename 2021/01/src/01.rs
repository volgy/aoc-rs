use aoc::Parse;
use itertools::Itertools;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(|line| line.parse_uw::<i64>())
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

fn part_2(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(|line| line.parse_uw::<i64>())
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}
