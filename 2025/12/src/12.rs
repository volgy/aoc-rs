use aoc::Parse;

aoc::parts!(1);

#[derive(Debug)]
struct Shape(Vec<Vec<bool>>);

impl Shape {
    fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Self {
        Shape(
            lines
                .take_while(|l| !l.is_empty())
                .map(|l| l.chars().map(|c| c == '#').collect())
                .collect(),
        )
    }

    fn min_area(&self) -> usize {
        self.0
            .iter()
            .map(|r| r.iter().filter(|x| **x).count())
            .sum()
    }

    fn max_area(&self) -> usize {
        self.0.iter().map(|r| r.len()).sum()
    }
}

#[derive(Debug)]
struct Region {
    size: (usize, usize),
    quantities: Vec<usize>,
}

impl Region {
    fn parse(line: &str) -> Self {
        let (size_s, quantities_s) = line.split_once(':').unwrap();
        let (size_x_s, size_y_s) = size_s.split_once('x').unwrap();
        let size = (size_x_s.parse_uw(), size_y_s.parse_uw());
        let quantities = quantities_s
            .split_whitespace()
            .map(|s| s.parse_uw())
            .collect();
        Self { size, quantities }
    }

    fn fit(&self, shapes: &[Shape], area_fn: fn(&Shape) -> usize) -> bool {
        let available = self.size.0 * self.size.1;
        let required = self
            .quantities
            .iter()
            .enumerate()
            .map(|(i, q)| q * area_fn(&shapes[i]))
            .sum();
        available >= required
    }

    fn min_fit(&self, shapes: &[Shape]) -> bool {
        self.fit(shapes, Shape::min_area)
    }

    fn max_fit(&self, shapes: &[Shape]) -> bool {
        self.fit(shapes, Shape::max_area)
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut shapes = vec![];
    let mut regions = vec![];
    let lines = input.as_lines();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        if line.contains('x') {
            regions.push(Region::parse(line));
            i += 1;
        } else {
            shapes.push(Shape::parse(lines[(i + 1)..].iter().copied()));
            i += lines[i..].iter().take_while(|l| !l.is_empty()).count();
            if i < lines.len() && lines[i].is_empty() {
                i += 1;
            }
        }
    }

    // sad simplification - does not work for the simple example
    let min_fit = regions.iter().filter(|r| r.min_fit(&shapes)).count();
    let max_fit = regions.iter().filter(|r| r.max_fit(&shapes)).count();
    if min_fit != max_fit {
        panic!("heuristic fails to work")
    }
    min_fit
}

// fn part_2(input: aoc::Input) -> impl ToString {
//     0
// }
