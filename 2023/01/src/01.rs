use std::collections::HashMap;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut sum = 0;
    for line in input.lines() {
        let mut digits = line.chars().filter_map(|c| c.to_digit(10));
        let first = digits
            .next()
            .expect("At least one digit should be in the line");
        let last = digits.last().unwrap_or(first);
        sum += 10 * first + last;
    }
    sum
}

fn part_2(input: aoc::Input) -> impl ToString {
    let vocab: HashMap<String, u32> = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .into_iter()
    .map(String::from)
    .zip(1..)
    .chain((1..=9).map(|x| (x.to_string(), x)))
    .collect();

    let mut sum = 0;

    for line in input.lines() {
        let mut tail = line;
        let mut digits = vec![];
        while !tail.is_empty() {
            for (name, value) in vocab.iter() {
                if tail.starts_with(name) {
                    digits.push(*value);
                    break;
                }
            }
            // hacky - I hate UTF-8
            let mut chars = tail.chars();
            chars.next();
            tail = chars.as_str();
        }

        assert!(
            !digits.is_empty(),
            "At least one digit should be in the line: {:?}",
            line
        );
        let first = digits[0];
        let last = digits[digits.len() - 1];

        sum += 10 * first + last;
    }
    sum
}
