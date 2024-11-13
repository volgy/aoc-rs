use itertools::Itertools;

aoc::parts!(1, 2);

fn parse(line: &str) -> (i32, i32) {
    line.chars()
        .filter_map(|c| match c {
            'A' | 'X' => Some(0),
            'B' | 'Y' => Some(1),
            'C' | 'Z' => Some(2),
            _ => None,
        })
        .collect_tuple()
        .unwrap()
}

fn part_1(input: aoc::Input) -> impl ToString {
    fn score((foe, me): (i32, i32)) -> i32 {
        me + 1
            + match (me - foe).rem_euclid(3) {
                0 => 3,
                1 => 6,
                _ => 0,
            }
    }

    input.lines().map(parse).map(score).sum::<i32>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    fn score((foe, me): (i32, i32)) -> i32 {
        match me {
            0 => (foe - 1).rem_euclid(3) + 1,
            1 => 3 + foe + 1,
            _ => 6 + (foe + 1).rem_euclid(3) + 1,
        }
    }
    input.lines().map(parse).map(score).sum::<i32>()
}
