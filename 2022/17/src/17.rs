aoc::parts!(1);

fn part_1(input: aoc::Input) -> impl ToString {
    let wind = input.raw().trim().as_bytes();
    let mut wind2 = wind.to_owned();
    wind2.extend_from_slice(wind);
    for i in 1..wind.len() {
        if wind2.chunks_exact(i).all(|c| c == &wind[0..i]) {
            println!("{i}");
        }
    }
    0
}

// fn part_2(input: aoc::Input) -> impl ToString {
//     0
// }
