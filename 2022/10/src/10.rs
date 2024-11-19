use core::str;

aoc::parts!(1, 2);

fn build_trace(input: aoc::Input) -> Vec<i32> {
    let mut x_trace = vec![1];

    for line in input.lines() {
        let last_x = *x_trace.last().unwrap();
        if line == "noop" {
            x_trace.push(last_x);
        } else {
            let dx: i32 = line.split_once(" ").unwrap().1.parse().unwrap();
            x_trace.push(last_x);
            x_trace.push(last_x + dx);
        }
    }
    x_trace
}

fn part_1(input: aoc::Input) -> impl ToString {
    let x_trace = build_trace(input);
    [20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|c| c as i32 * x_trace[c - 1])
        .sum::<i32>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let x_trace = build_trace(input);
    let screen = x_trace
        .into_iter()
        .enumerate()
        .map(|(i, x)| {
            if ((i % 40) as i32 - x).abs() < 2 {
                '#'
            } else {
                '.'
            }
        })
        .collect::<String>();
    for line in screen.as_bytes().chunks(40).map(str::from_utf8) {
        println!("{}", line.unwrap());
    }
    "RKPJBPLA" // actual answer is derived manually (looking at the printed results)
}
