use itertools::Itertools;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut n_safe = 0;
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|w| w.parse::<i32>().unwrap())
                .tuple_windows()
                .map(|(a, b)| b - a)
                .peekable()
        })
        .for_each(|mut diff| {
            let sign = diff.peek().unwrap().signum();
            if diff.all(|d| (d * sign) > 0 && (d * sign) <= 3) {
                n_safe += 1;
            }
        });
    n_safe
}

fn part_2(input: aoc::Input) -> impl ToString {
    let reports = input.lines().map(|l| {
        l.split_whitespace()
            .map(|w| w.parse::<i32>().unwrap())
            .collect_vec()
    });

    let mut n_safe = 0;

    // Let's just brute force it
    for report in reports {
        'trials: for skip in 0..=report.len() {
            let mut trial_diff = report
                .iter()
                .enumerate()
                .filter_map(|(i, d)| if i == skip { None } else { Some(*d) })
                .tuple_windows()
                .map(|(a, b)| b - a)
                .peekable();

            let sign = trial_diff.peek().unwrap().signum();
            if trial_diff.all(|d| (d * sign) > 0 && (d * sign) <= 3) {
                n_safe += 1;
                break 'trials;
            }
        }
    }

    n_safe
}
