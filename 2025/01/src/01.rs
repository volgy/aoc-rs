aoc::parts!(1, 2);

const TICKS: isize = 100;
const START: isize = 50;

fn rotations(input: aoc::Input) -> Vec<isize> {
    input
        .lines()
        .map(|l| {
            let (dir, rest) = l.split_at(1);
            let value: isize = rest.parse().unwrap();
            match dir {
                "L" => -value,
                "R" => value,
                _ => unreachable!(),
            }
        })
        .collect()
}
fn part_1(input: aoc::Input) -> impl ToString {
    rotations(input)
        .into_iter()
        .scan(START, |dial, r| {
            *dial = (*dial + r).rem_euclid(TICKS);
            Some(*dial)
        })
        .filter(|&d| d == 0)
        .count()
}

fn part_2(input: aoc::Input) -> impl ToString {
    rotations(input)
        .into_iter()
        .scan(START, |dial, r| {
            let zeros = if r >= 0 {
                (*dial + r).div_euclid(TICKS)
            } else {
                let dist = (TICKS - *dial).rem_euclid(TICKS);
                (dist - r).div_euclid(TICKS)
            };
            *dial = (*dial + r).rem_euclid(TICKS);
            Some(zeros)
        })
        .sum::<isize>()
}
