aoc::parts!(1);

fn hash(s: &str) -> u32 {
    s.bytes().fold(0, |acc, b| ((acc + (b as u32)) * 17) & 0xFF)
}

fn part_1(input: aoc::Input) -> impl ToString {
    input.raw().split(',').map(|s| hash(s)).sum::<u32>()
}

// fn part_2(input: aoc::Input) -> impl ToString {
//     0
// }
