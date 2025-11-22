use std::collections::HashSet;

aoc::parts!(1, 2);

fn majority_ones<'a, I>(lines: I, pos: usize) -> bool
where
    I: Iterator<Item = &'a str>,
{
    let (mut n_0, mut n_1) = (0, 0);
    for line in lines {
        match line.chars().nth(pos).unwrap() {
            '0' => n_0 += 1,
            '1' => n_1 += 1,
            _ => unreachable!(),
        }
    }
    n_1 >= n_0
}

fn part_1(input: aoc::Input) -> impl ToString {
    let n_digits = input.as_lines()[0].len();

    let gamma = (0..n_digits).fold(0, |acc, i_digit| {
        acc << 1
            | if majority_ones(input.lines(), i_digit) {
                1
            } else {
                0
            }
    });
    let epsilon = !gamma & ((1 << n_digits) - 1);
    gamma * epsilon
}

fn part_2(input: aoc::Input) -> impl ToString {
    let n_digits = input.as_lines()[0].len();

    let mut o2_set: HashSet<_> = input.lines().collect();
    let mut co2_set = o2_set.clone();

    for i_digit in 0..n_digits {
        if o2_set.len() > 1 {
            let target = if majority_ones(o2_set.iter().copied(), i_digit) {
                '1'
            } else {
                '0'
            };
            o2_set.retain(|&line| line.chars().nth(i_digit).unwrap() == target);
        }

        if co2_set.len() > 1 {
            let target = if majority_ones(co2_set.iter().copied(), i_digit) {
                '0'
            } else {
                '1'
            };
            co2_set.retain(|&line| line.chars().nth(i_digit).unwrap() == target);
        }
    }

    let o2_rating = u32::from_str_radix(o2_set.iter().next().unwrap(), 2).unwrap();
    let co2_rating = u32::from_str_radix(co2_set.iter().next().unwrap(), 2).unwrap();
    o2_rating * co2_rating
}
