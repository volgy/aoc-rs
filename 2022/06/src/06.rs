use counter::Counter;

aoc::parts!(1, 2);

fn start_of_marker(line: &str, code_len: usize) -> usize {
    let (init, rest) = line.split_at(code_len);
    let mut last_four: Counter<_> = init.chars().collect();

    for (i, (ch_add, ch_remove)) in rest.chars().zip(line.chars()).enumerate() {
        if last_four.len() == code_len {
            return i + code_len;
        }
        last_four[&ch_add] += 1;
        last_four[&ch_remove] -= 1;
        last_four.retain(|_, &mut v| v > 0); // a bit hacky
    }
    panic!("unable to find marker");
}
fn part_1(input: aoc::Input) -> impl ToString {
    start_of_marker(input.raw(), 4)
}

// fn part_2(input: aoc::Input) -> impl ToString {
//     start_of_marker(input.raw(), 14)
// }
