use itertools::Itertools;

aoc::parts!(1, 2);

fn invalids((start, end): (usize, usize), n_rep: usize) -> Vec<usize> {
    let stem_len = start.to_string().len() / n_rep;
    let stem = start.to_string()[..stem_len].parse::<usize>().unwrap_or(0);
    (stem..)
        .map(|s| s.to_string().repeat(n_rep).parse::<usize>().unwrap())
        .take_while(|&x| x <= end)
        .filter(|&x| x >= start)
        .collect()
}

fn ranges<'a>(input: aoc::Input<'a>) -> impl Iterator<Item = (usize, usize)> + use<'a> {
    input.as_lines()[0].split(',').map(|p| {
        p.split('-')
            .map(|s| s.parse().unwrap())
            .collect_tuple()
            .unwrap()
    })
}

fn part_1(input: aoc::Input) -> impl ToString {
    ranges(input).flat_map(|r| invalids(r, 2)).sum::<usize>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    ranges(input)
        .flat_map(|r| {
            let max_n_reps = r.1.to_string().len();
            (2..=max_n_reps).flat_map(move |n| invalids(r, n))
        })
        .unique()
        .sum::<usize>()
}
