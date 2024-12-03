use regex::Regex;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input.raw())
        .map(|c| c.extract().1)
        .map(|[a, b]| a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap())
        .sum::<i32>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let re = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").unwrap();

    let mut sum = 0;
    let mut enabled = true;
    for cap in re.captures_iter(input.raw()) {
        match cap.get(0).unwrap().as_str() {
            cmd if cmd.starts_with("mul") => {
                if enabled {
                    sum += cap[2].parse::<i32>().unwrap() * cap[3].parse::<i32>().unwrap();
                }
            }
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => unreachable!(),
        }
    }
    sum
}
