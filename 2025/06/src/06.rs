use aoc::Parse;

aoc::parts!(1, 2);

type ArgParser = fn(&[&str]) -> Vec<Vec<i64>>;

fn parse_horizontal_args(lines: &[&str]) -> Vec<Vec<i64>> {
    let args: Vec<Vec<i64>> = lines
        .iter()
        .map(|l| l.split_whitespace().map(|s| s.parse_uw()).collect())
        .collect();

    (0..args[0].len())
        .map(|i| args.iter().map(|row| row[i]).collect())
        .collect()
}

fn parse_vertical_args(lines: &[&str]) -> Vec<Vec<i64>> {
    // transpose chars
    let cols: Vec<String> = (0..lines[0].len())
        .map(|i| {
            lines
                .iter()
                .map(|row| row.chars().nth(i).unwrap())
                .collect()
        })
        .collect();

    cols.split(|col| col.trim().is_empty())
        .filter(|group| !group.is_empty())
        .map(|group| group.iter().map(|s| s.trim().parse().unwrap()).collect())
        .collect()
}

fn eval(lines: &[&str], arg_parser: ArgParser) -> i64 {
    let args = arg_parser(&lines[..lines.len() - 1]);
    let op_line = lines.last().unwrap();

    op_line
        .split_whitespace()
        .zip(args)
        .map(|(op, arg)| match op {
            "+" => arg.into_iter().sum::<i64>(),
            "*" => arg.into_iter().product::<i64>(),
            _ => unreachable!(),
        })
        .sum()
}

fn part_1(input: aoc::Input) -> impl ToString {
    eval(input.as_lines(), parse_horizontal_args)
}

fn part_2(input: aoc::Input) -> impl ToString {
    eval(input.as_lines(), parse_vertical_args)
}
