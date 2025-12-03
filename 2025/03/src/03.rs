aoc::parts!(1, 2);

fn joltage<const N: usize>(input: aoc::Input) -> u64 {
    input
        .lines()
        .map(|l| {
            let batteries: Vec<_> = l.chars().map(|c| c.to_digit(10).unwrap()).collect();
            batteries
                .windows(N)
                .map(|w| <[u32; N]>::try_from(w).unwrap())
                .reduce(|mut max, digits| {
                    for i in 0..digits.len() {
                        if digits[i] > max[i] {
                            max[i..].copy_from_slice(&digits[i..]);
                            break;
                        }
                    }
                    max
                })
                .unwrap()
        })
        .map(|m| m.iter().fold(0, |acc, &d| acc * 10 + d as u64))
        .sum::<u64>()
}

fn part_1(input: aoc::Input) -> impl ToString {
    joltage::<2>(input)
}

fn part_2(input: aoc::Input) -> impl ToString {
    joltage::<12>(input)
}
