aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let unmixed: Vec<_> = input.lines().map(|l| l.parse::<isize>().unwrap()).collect();

    let mut mixed: Vec<_> = (0..unmixed.len()).collect();
    for (id, offset) in unmixed.iter().enumerate() {
        let idx = mixed.iter().position(|&i| i == id).unwrap();
        mixed.remove(idx);
        let new_idx = (idx as isize + offset).rem_euclid(mixed.len() as isize) as usize;
        mixed.insert(new_idx, id);
    }

    let zero_id = unmixed.iter().position(|&i| i == 0).unwrap();
    let zero_idx = mixed.iter().position(|&i| i == zero_id).unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|i| unmixed[mixed[(zero_idx + i) % mixed.len()]])
        .sum::<isize>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    const DECRYPTION_KEY: isize = 811_589_153;
    let unmixed: Vec<_> = input.lines().map(|l| l.parse::<isize>().unwrap()).collect();

    let mut mixed: Vec<_> = (0..unmixed.len()).collect();
    for _ in 0..10 {
        for (id, offset) in unmixed.iter().enumerate() {
            let offset = offset * DECRYPTION_KEY;
            let idx = mixed.iter().position(|&i| i == id).unwrap();
            mixed.remove(idx);
            let new_idx = (idx as isize + offset).rem_euclid(mixed.len() as isize) as usize;
            mixed.insert(new_idx, id);
        }
    }

    let zero_id = unmixed.iter().position(|&i| i == 0).unwrap();
    let zero_idx = mixed.iter().position(|&i| i == zero_id).unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|i| unmixed[mixed[(zero_idx + i) % mixed.len()]] * DECRYPTION_KEY)
        .sum::<isize>()
}
