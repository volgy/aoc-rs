use itertools::Itertools;

aoc::parts!(1, 2);

fn parse_ranges<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Vec<(usize, usize)> {
    lines
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (s, e) = l.split_once('-').unwrap();
            (s.parse().unwrap(), e.parse().unwrap())
        })
        .collect()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut lines = input.lines();
    let ranges: Vec<(usize, usize)> = parse_ranges(lines.by_ref());

    lines
        .filter(|l| {
            let n: usize = l.parse().unwrap();
            ranges.iter().any(|&(s, e)| n >= s && n <= e)
        })
        .count()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut ranges = parse_ranges(&mut input.lines());
    ranges.sort();
    ranges
        .into_iter()
        .coalesce(|(s1, e1), (s2, e2)| {
            if s2 > e1 {
                Err(((s1, e1), (s2, e2)))
            } else {
                Ok((s1, e1.max(e2)))
            }
        })
        .map(|(s, e)| e - s + 1)
        .sum::<usize>()
}
