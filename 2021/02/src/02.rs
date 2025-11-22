aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let (pos, depth) = input.lines().fold((0, 0), |(pos, depth), line| {
        let (instruction, value) = line.split_once(' ').unwrap();
        let value: i64 = value.parse().unwrap();
        match instruction {
            "forward" => (pos + value, depth),
            "down" => (pos, depth + value),
            "up" => (pos, depth - value),
            _ => unreachable!(),
        }
    });
    pos * depth
}

fn part_2(input: aoc::Input) -> impl ToString {
    let (pos, depth, _) = input.lines().fold((0, 0, 0), |(pos, depth, aim), line| {
        let (instruction, value) = line.split_once(' ').unwrap();
        let value: i64 = value.parse().unwrap();
        match instruction {
            "forward" => (pos + value, depth + aim * value, aim),
            "down" => (pos, depth, aim + value),
            "up" => (pos, depth, aim - value),
            _ => unreachable!(),
        }
    });
    pos * depth
}
