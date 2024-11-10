aoc::parts!(1, 2);

use std::{collections::HashMap, fmt};

#[derive(Debug)]
struct Platform {
    grid: Vec<char>,
    rows: usize,
    cols: usize,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in (0..self.rows).map(|i| {
            self.grid[(i * self.cols)..((i + 1) * self.cols)]
                .iter()
                .collect::<String>()
        }) {
            writeln!(f, "{line}")?;
        }
        Ok(())
    }
}

impl Platform {
    fn parse(input: aoc::Input) -> Self {
        let nested: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
        let (rows, cols) = (nested.len(), nested[0].len());
        let grid = nested.into_iter().flatten().collect();
        Self { grid, rows, cols }
    }

    fn tilt_north(&mut self) {
        for j in 0..self.cols {
            let mut slide_pos = 0;
            for i in 0..self.rows {
                let cidx = i * self.cols + j;
                let sidx = slide_pos * self.cols + j;
                match self.grid[cidx] {
                    'O' => {
                        self.grid.swap(cidx, sidx);
                        slide_pos += 1;
                    }
                    '#' => {
                        slide_pos = i + 1;
                    }
                    _ => {}
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for i in 0..self.rows {
            let mut slide_pos = 0;
            for j in 0..self.cols {
                let cidx = i * self.cols + j;
                let sidx = i * self.cols + slide_pos;
                match self.grid[cidx] {
                    'O' => {
                        self.grid.swap(cidx, sidx);
                        slide_pos += 1;
                    }
                    '#' => {
                        slide_pos = j + 1;
                    }
                    _ => {}
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for j in 0..self.cols {
            let mut slide_pos = self.rows - 1;
            for i in (0..self.rows).rev() {
                let cidx = i * self.cols + j;
                let sidx = slide_pos * self.cols + j;
                match self.grid[cidx] {
                    'O' => {
                        self.grid.swap(cidx, sidx);
                        slide_pos = slide_pos.saturating_sub(1);
                    }
                    '#' if i > 0 => {
                        slide_pos = i - 1;
                    }
                    _ => {}
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for i in 0..self.rows {
            let mut slide_pos = self.cols - 1;
            for j in (0..self.cols).rev() {
                let cidx = i * self.cols + j;
                let sidx = i * self.cols + slide_pos;
                match self.grid[cidx] {
                    'O' => {
                        self.grid.swap(cidx, sidx);
                        slide_pos = slide_pos.saturating_sub(1);
                    }
                    '#' if j > 0 => {
                        slide_pos = j - 1;
                    }
                    _ => {}
                }
            }
        }
    }

    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn north_load(&self) -> usize {
        let mut load = 0;
        for j in 0..self.cols {
            for i in 0..self.rows {
                load += if self.grid[i * self.cols + j] == 'O' {
                    self.rows - i
                } else {
                    0
                };
            }
        }
        load
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut platform = Platform::parse(input);
    platform.tilt_north();
    platform.north_load()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut platform = Platform::parse(input);
    let mut memory = HashMap::new();
    let mut i = 0;
    loop {
        memory.insert(platform.grid.clone(), i);
        platform.cycle();
        i += 1;
        if let Some(j) = memory.get(&platform.grid) {
            let k = (1_000_000_000 - j) % (i - j);
            for _ in 0..k {
                platform.cycle();
            }
            break;
        }
    }

    platform.north_load()
}
