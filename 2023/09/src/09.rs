aoc::parts!(1, 2);

fn parse(input: aoc::Input) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(str::split_whitespace)
        .map(|i| i.map(|s| s.parse().unwrap()).collect())
        .collect()
}

fn predict(series: &[i32]) -> (i32, i32) {
    let mut turtles = vec![series.to_owned()];
    loop {
        let bottom = turtles.last().unwrap();
        let new_bottom: Vec<_> = bottom
            .iter()
            .zip(bottom.iter().skip(1))
            .map(|(i, j)| j - i)
            .collect();
        if new_bottom.is_empty() || new_bottom.iter().all(|x| *x == 0) {
            break;
        }
        turtles.push(new_bottom);
    }

    let future = turtles.iter().map(|t| t.last().unwrap()).sum();
    let past = turtles
        .iter()
        .zip([1, -1].iter().cycle())
        .map(|(t, sign)| sign * t.first().unwrap())
        .sum();
    (past, future)
}

fn part_1(input: aoc::Input) -> impl ToString {
    parse(input)
        .iter()
        .map(|s| predict(s.as_ref()).1)
        .sum::<i32>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    parse(input)
        .iter()
        .map(|s| predict(s.as_ref()).0)
        .sum::<i32>()
}
