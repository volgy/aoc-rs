use std::collections::HashMap;

aoc::parts!(1, 2);

struct Manifold {
    start: usize,
    splitters: Vec<Vec<usize>>,
}

impl Manifold {
    fn from_lines(lines: &[&str]) -> Self {
        let start = lines[0].find('S').unwrap(); // ascii strings
        let splitters = lines[1..]
            .iter()
            .map(|l| {
                l.chars()
                    .enumerate()
                    .filter_map(|(i, c)| (c == '^').then_some(i))
                    .collect()
            })
            .collect();
        Self { start, splitters }
    }

    fn simulate(&self) -> (usize, usize) {
        let mut n_splits = 0;
        let mut beams = HashMap::new();
        beams.insert(self.start, 1);
        for layer in &self.splitters {
            let mut new_beams = HashMap::new();
            for (beam, count) in beams {
                if layer.binary_search(&beam).is_ok() {
                    // technically, we should check if within bounds, but the test cases
                    // allow to be sloppy here
                    *new_beams.entry(beam - 1).or_default() += count;
                    *new_beams.entry(beam + 1).or_default() += count;
                    n_splits += 1;
                } else {
                    *new_beams.entry(beam).or_default() += count;
                }
            }
            beams = new_beams;
        }
        let n_paths = beams.values().sum();
        (n_splits, n_paths)
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    Manifold::from_lines(input.as_lines()).simulate().0
}

fn part_2(input: aoc::Input) -> impl ToString {
    Manifold::from_lines(input.as_lines()).simulate().1
}
