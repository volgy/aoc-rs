use aoc::Parse;
use itertools::Itertools;
use itertools::MinMaxResult;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let crabs = input.as_lines()[0]
        .ints_iter::<u32>()
        .sorted()
        .collect_vec();
    let median = crabs[crabs.len() / 2]; // does not matter if even or odd in this problem
    crabs.iter().map(|&c| c.abs_diff(median)).sum::<u32>()
}

// Original idea was to use the rounded mean, but this does not properly minimizes the d*(d+1)
// distance formula. Falling back to basic brute-force.
fn part_2(input: aoc::Input) -> impl ToString {
    let crabs = input.as_lines()[0].ints_iter::<u32>().collect_vec();

    let MinMaxResult::MinMax(&min, &max) = crabs.iter().minmax() else {
        panic!("not enough crabs")
    };

    let (_, fuel) = (min..=max)
        .map(|pos| {
            let fuel = crabs
                .iter()
                .map(|&c| {
                    let d = c.abs_diff(pos);
                    d * (d + 1) / 2
                })
                .sum::<u32>();
            (pos, fuel)
        })
        .min_by_key(|&(_, fuel)| fuel)
        .unwrap();
    fuel
}
