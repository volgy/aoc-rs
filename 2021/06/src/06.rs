use aoc::{IterUnwrap, Parse};

aoc::parts!(1, 2);

fn family_size(fish: u8, days: usize) -> usize {
    let mut phase_counts = [0_usize; 9];
    phase_counts[fish as usize] = 1;
    for _ in 0..days {
        phase_counts.rotate_left(1);
        phase_counts[6] += phase_counts[8];
    }
    phase_counts.iter().sum()
}

fn population(input: aoc::Input, days: usize) -> usize {
    input
        .lines()
        .next_uw()
        .ints_iter()
        .map(|f| family_size(f, days))
        .sum::<usize>()
}

fn part_1(input: aoc::Input) -> impl ToString {
    population(input, 80)
}

fn part_2(input: aoc::Input) -> impl ToString {
    population(input, 256)
}
