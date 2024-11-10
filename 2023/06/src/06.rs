aoc::parts!(1, 2);

fn parse1(input: aoc::Input) -> Vec<(u64, u64)> {
    let mut lines = input.lines();

    let mut get_items = |prefix| {
        lines
            .next()
            .unwrap()
            .strip_prefix(prefix)
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
    };

    get_items("Time: ").zip(get_items("Distance: ")).collect()
}

fn parse2(input: aoc::Input) -> (u64, u64) {
    let mut lines = input.lines();

    let mut get_item = |prefix| {
        lines
            .next()
            .unwrap()
            .strip_prefix(prefix)
            .unwrap()
            .chars()
            .filter(char::is_ascii_digit)
            .collect::<String>()
            .parse::<u64>()
            .unwrap()
    };

    (get_item("Time: "), get_item("Distance: "))
}

fn margin(time: u64, distance: u64) -> u64 {
    let d_roots = (((time * time) - 4 * distance) as f64).sqrt();
    let lo = (time as f64 - d_roots) / 2.0 * (1.0 + f64::EPSILON);
    let hi = (time as f64 + d_roots) / 2.0 * (1.0 - f64::EPSILON);
    if d_roots > 0.0 {
        (hi.floor() - lo.ceil() + 1.0) as u64
    } else {
        0
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    parse1(input)
        .into_iter()
        .map(|(t, d)| margin(t, d))
        .product::<u64>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let (time, distance) = parse2(input);
    margin(time, distance)
}
