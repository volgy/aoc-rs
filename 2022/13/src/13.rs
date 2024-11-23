use std::cmp::Ordering;

use itertools::Itertools;

aoc::parts!(1, 2);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Int(i32),
    List(Vec<Packet>),
}

impl Packet {
    fn parse_recursively(input: &[u8]) -> Packet {
        assert_eq!(input[0], b'[');
        assert_eq!(input[input.len() - 1], b']');
        let input = &input[1..input.len() - 1];
        let mut contents = vec![];
        let mut i = 0;
        while i < input.len() {
            match input[i] {
                b'[' => {
                    let mut depth = 1;
                    let mut j = i + 1;
                    while j < input.len() && depth > 0 {
                        match input[j] {
                            b'[' => depth += 1,
                            b']' => depth -= 1,
                            _ => (),
                        }
                        j += 1;
                    }
                    contents.push(Packet::parse_recursively(&input[i..j]));
                    i = j;
                }
                b',' => i += 1,
                b']' => break,
                _ => {
                    let mut j = i;
                    while j < input.len() && input[j].is_ascii_digit() {
                        j += 1;
                    }
                    let num = std::str::from_utf8(&input[i..j]).unwrap().parse().unwrap();
                    contents.push(Packet::Int(num));
                    i = j;
                }
            }
        }
        Packet::List(contents)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Packet::Int(a), Packet::Int(b)) => a.partial_cmp(b),
            (Packet::List(a), Packet::List(b)) => a.partial_cmp(b),
            (Packet::Int(a), Packet::List(b)) => vec![Packet::Int(*a)].partial_cmp(b),
            (Packet::List(a), Packet::Int(b)) => a.partial_cmp(&vec![Packet::Int(*b)]),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut sum_correct = 0;
    for (idx, (left, right)) in input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| Packet::parse_recursively(l.as_bytes()))
        .tuples()
        .enumerate()
    {
        if left < right {
            sum_correct += idx + 1
        }
    }
    sum_correct
}

fn part_2(input: aoc::Input) -> impl ToString {
    let dividers = [
        Packet::parse_recursively("[[2]]".as_bytes()),
        Packet::parse_recursively("[[6]]".as_bytes()),
    ];
    let ordered: Vec<_> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| Packet::parse_recursively(l.as_bytes()))
        .chain(dividers.iter().cloned())
        .sorted()
        .collect();

    (ordered.iter().position(|p| p == &dividers[0]).unwrap() + 1)
        * (ordered.iter().position(|p| p == &dividers[1]).unwrap() + 1)
}
