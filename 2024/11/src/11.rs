use std::{collections::HashMap, str::from_utf8};

aoc::parts!(1, 2);

fn stone_growth(stone: u64, n_blinks: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    if n_blinks == 0 {
        return 1;
    }

    if let Some(&growth) = cache.get(&(stone, n_blinks)) {
        return growth;
    }

    let growth = if stone == 0 {
        stone_growth(1, n_blinks - 1, cache)
    } else {
        let stone_str = stone.to_string();
        if stone_str.len() % 2 == 0 {
            stone_str
                .as_bytes()
                .chunks(stone_str.len() / 2)
                .map(|chunk| {
                    stone_growth(
                        from_utf8(chunk).unwrap().parse().unwrap(),
                        n_blinks - 1,
                        cache,
                    )
                })
                .sum()
        } else {
            stone_growth(stone * 2024, n_blinks - 1, cache)
        }
    };

    cache.insert((stone, n_blinks), growth);
    growth
}

fn line_growth(input: aoc::Input, n_blinks: u64) -> u64 {
    let mut cache = HashMap::new();
    input
        .raw()
        .split_whitespace()
        .map(|s| stone_growth(s.parse().unwrap(), n_blinks, &mut cache))
        .sum()
}

fn part_1(input: aoc::Input) -> impl ToString {
    line_growth(input, 25)
}

fn part_2(input: aoc::Input) -> impl ToString {
    line_growth(input, 75)
}
