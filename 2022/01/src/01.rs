aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut max = 0;
    let mut sum = 0;
    for line in input.lines() {
        sum = if line.is_empty() {
            0
        } else {
            sum + line.parse::<usize>().unwrap()
        };
        max = max.max(sum);
    }

    max
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut sums = vec![];

    let mut sum = 0;
    for line in input.lines().chain(Some("")) {
        sum = if line.is_empty() {
            sums.push(sum);
            0
        } else {
            sum + line.parse::<usize>().unwrap()
        };
    }

    sums.sort();
    sums.iter().rev().take(3).fold(0, |acc, x| acc + x)
}
