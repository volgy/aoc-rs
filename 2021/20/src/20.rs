use std::ops::{Index, IndexMut};

aoc::parts!(1, 2);

struct Image {
    size: usize,
    pixels: Vec<Vec<bool>>,
    boundary: bool,
}

impl Image {
    fn new(size: usize, boundary: bool) -> Self {
        Self {
            size,
            pixels: vec![vec![false; size]; size],
            boundary,
        }
    }

    fn parse(lines: &[&str]) -> Self {
        let size = lines.len();
        let pixels = lines
            .iter()
            .map(|l| l.chars().map(|c| c == '#').collect())
            .collect();
        Self {
            size,
            pixels,
            boundary: false,
        }
    }

    fn n_lit(&self) -> usize {
        self.pixels.iter().flatten().filter(|p| **p).count()
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in &self.pixels {
            for &p in row {
                print!("{}", if p { '#' } else { '.' });
            }
            println!();
        }
    }
}

impl Index<(isize, isize)> for Image {
    type Output = bool;

    fn index(&self, index: (isize, isize)) -> &Self::Output {
        let extent = 0..self.size as isize;
        if !extent.contains(&index.0) || !extent.contains(&index.1) {
            &self.boundary
        } else {
            &self.pixels[index.0 as usize][index.1 as usize]
        }
    }
}

impl IndexMut<(isize, isize)> for Image {
    fn index_mut(&mut self, index: (isize, isize)) -> &mut Self::Output {
        let extent = 0..self.size as isize;
        if !extent.contains(&index.0) || !extent.contains(&index.1) {
            panic!("Index out of bounds");
        }
        &mut self.pixels[index.0 as usize][index.1 as usize]
    }
}

struct Convolution(Vec<bool>);

impl Convolution {
    fn parse(line: &str) -> Self {
        Self(line.chars().map(|c| c == '#').collect())
    }

    fn next_boundary(&self, current: bool) -> bool {
        if current {
            self.0[self.0.len() - 1]
        } else {
            self.0[0]
        }
    }

    fn apply(&self, input: &Image) -> Image {
        let mut output = Image::new(input.size + 2, self.next_boundary(input.boundary));

        for i in 0..output.size as isize {
            for j in 0..output.size as isize {
                let mut idx = 0;
                for di in [-1, 0, 1] {
                    for dj in [-1, 0, 1] {
                        idx = (idx << 1) + input[(i + di - 1, j + dj - 1)] as usize;
                    }
                }
                output[(i, j)] = self.0[idx];
            }
        }
        output
    }
}

fn solve(input: aoc::Input, n_iterations: usize) -> usize {
    let lines = input.as_lines();
    let conv = Convolution::parse(lines[0]);
    let image = Image::parse(&lines[2..]);
    (0..n_iterations)
        .fold(image, |img, _| conv.apply(&img))
        .n_lit()
}

fn part_1(input: aoc::Input) -> impl ToString {
    solve(input, 2)
}

fn part_2(input: aoc::Input) -> impl ToString {
    solve(input, 50)
}
