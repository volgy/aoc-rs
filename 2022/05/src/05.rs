use aoc::Parse;
use std::collections::VecDeque;

aoc::parts!(1, 2);

fn crane(input: aoc::Input, move_multiple: bool) -> String {
    let mut stacks = vec![];
    let mut lines = input.lines();

    // hacky stack parsing
    for line in &mut lines {
        if line.is_empty() {
            break;
        }
        for (i, ch) in line.chars().enumerate() {
            if ch.is_alphabetic() {
                let i_stack = (i - 1) / 4;
                while stacks.len() <= i_stack {
                    stacks.push(VecDeque::new())
                }
                stacks[i_stack].push_back(ch);
            }
        }
    }

    for line in &mut lines {
        let mut parser = line.as_parser();
        let count: usize = parser.between("move ", " ").parse().unwrap();
        let from: usize = parser.between("from ", " ").parse().unwrap();
        let to: usize = parser.after("to ").parse().unwrap();

        let crates: VecDeque<_> = (0..count)
            .map(|_| stacks[from - 1].pop_front().unwrap())
            .collect();

        if move_multiple {
            for ch in crates.into_iter().rev() {
                stacks[to - 1].push_front(ch);
            }
        } else {
            for ch in crates {
                stacks[to - 1].push_front(ch);
            }
        }
    }

    stacks.iter().filter_map(|s| s.front()).collect::<String>()
}

fn part_1(input: aoc::Input) -> impl ToString {
    crane(input, false)
}

fn part_2(input: aoc::Input) -> impl ToString {
    crane(input, true)
}
