aoc::parts!(1, 2);

const PAIRS: &[(char, char, u64, u64)] = &[
    ('(', ')', 3, 1),
    ('[', ']', 57, 2),
    ('{', '}', 1197, 3),
    ('<', '>', 25137, 4),
];

fn get_closing(open: char) -> Option<char> {
    PAIRS
        .iter()
        .find_map(|(o, c, ..)| (*o == open).then_some(*c))
}

fn get_invalid_score(close: char) -> Option<u64> {
    PAIRS
        .iter()
        .find_map(|(_, c, s, _)| (*c == close).then_some(*s))
}

fn get_completion_score(close: char) -> Option<u64> {
    PAIRS
        .iter()
        .find_map(|(_, c, _, s)| (*c == close).then_some(*s))
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut score = 0;
    for line in input.lines() {
        let mut stack = Vec::new();
        for ch in line.chars() {
            if let Some(close) = get_closing(ch) {
                stack.push(close);
            } else if stack.pop() != Some(ch) {
                score += get_invalid_score(ch).unwrap();
                break;
            }
        }
    }
    score
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut scores = vec![];
    'outer: for line in input.lines() {
        let mut stack = Vec::new();
        for ch in line.chars() {
            if let Some(close) = get_closing(ch) {
                stack.push(close);
            } else if stack.pop() != Some(ch) {
                continue 'outer;
            }
        }

        if stack.is_empty() {
            continue;
        }

        scores.push(
            stack
                .into_iter()
                .rev()
                .fold(0, |acc, ch| 5 * acc + get_completion_score(ch).unwrap()),
        );
    }
    scores.sort();
    scores[scores.len() / 2]
}
