use itertools::{repeat_n, Itertools};

aoc::parts!(1, 2);

fn parse(input: aoc::Input) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|l| {
            let (lhs, rhs) = l.split_once(":").unwrap();
            (
                lhs.parse().unwrap(),
                rhs.split_whitespace().map(|n| n.parse().unwrap()).collect(),
            )
        })
        .collect()
}

fn calibrate(input: aoc::Input, ops: &[fn(i64, i64) -> i64]) -> i64 {
    parse(input)
        .iter()
        .filter_map(|(lhs, rhs)| {
            repeat_n(ops, rhs.len())
                .multi_cartesian_product()
                .find_map(|ops| {
                    let result = rhs
                        .iter()
                        .skip(1)
                        .zip(&ops)
                        .fold(rhs[0], |acc, (n, op)| op(acc, *n));
                    (result == *lhs).then_some(lhs)
                })
        })
        .sum()
}

fn part_1(input: aoc::Input) -> impl ToString {
    calibrate(input, &[i64::wrapping_add, i64::wrapping_mul])
}

fn part_2(input: aoc::Input) -> impl ToString {
    fn concat(a: i64, b: i64) -> i64 {
        (a.to_string() + &b.to_string()).parse().unwrap()
    }
    calibrate(input, &[i64::wrapping_add, i64::wrapping_mul, concat])
}
