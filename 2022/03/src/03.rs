use itertools::Itertools;
use std::collections::HashSet;

aoc::parts!(1, 2);

fn priority(item: u8) -> u8 {
    match item as char {
        'a'..='z' => item - 'a' as u8 + 1,
        'A'..='Z' => item - 'A' as u8 + 27,
        _ => panic!("invalid item"),
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut sum = 0;
    for line in input.lines() {
        let mid = line.len() / 2;
        let left: HashSet<_> = line[..mid].bytes().collect();
        let right: HashSet<_> = line[mid..].bytes().collect();
        let p_sum: i32 = left.intersection(&right).map(|&x| priority(x) as i32).sum();
        sum += p_sum;
    }
    sum
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut sum = 0;
    for mut group in &input.lines().chunks(3) {
        let first: HashSet<_> = group.next().unwrap().bytes().collect();
        let rest = group
            .map(|l| l.bytes().collect::<HashSet<_>>())
            .collect::<Vec<_>>();
        let common = first.iter().filter(|x| rest.iter().all(|s| s.contains(x)));
        sum += common.map(|&x| priority(x) as i32).sum::<i32>();
    }
    sum
}
