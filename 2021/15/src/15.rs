use std::{cmp::Reverse, collections::BinaryHeap};

aoc::parts!(1, 2);

type Pos = (usize, usize);

struct Cavern {
    size: usize,
    start: Pos,
    end: Pos,
    risk: Vec<Vec<u8>>,
}

impl Cavern {
    fn from_lines(lines: &[&str]) -> Self {
        let size = lines.len();
        let risk = lines
            .iter()
            .map(|l| l.bytes().map(|b| b - b'0').collect())
            .collect();
        let start = (0, 0);
        let end = (size - 1, size - 1);
        Self {
            size,
            start,
            end,
            risk,
        }
    }

    fn neighbors(&self, (x, y): Pos) -> impl Iterator<Item = Pos> {
        let size = self.size;
        [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter().filter_map(
            move |(dx, dy): (isize, isize)| {
                let nx = x.checked_add_signed(dx)?;
                let ny = y.checked_add_signed(dy)?;
                (nx < size && ny < size).then_some((nx, ny))
            },
        )
    }

    fn grow(&mut self, factor: usize) {
        let new_size = self.size * factor;
        let mut new_risk = vec![vec![0; new_size]; new_size];
        for tx in 0..factor {
            for ty in 0..factor {
                for x in 0..self.size {
                    for y in 0..self.size {
                        let mut r = self.risk[x][y] + (tx + ty) as u8;
                        if r > 9 {
                            r -= 9;
                        }
                        new_risk[tx * self.size + x][ty * self.size + y] = r;
                    }
                }
            }
        }
        self.size = new_size;
        self.risk = new_risk;
        self.end = (new_size - 1, new_size - 1);
    }

    fn lowest_total_risk(&self) -> i32 {
        // Dijkstra
        let mut distances = vec![vec![i32::MAX; self.size]; self.size];
        distances[self.start.0][self.start.1] = 0;
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, self.start)));

        while let Some(Reverse((total_risk, pos))) = heap.pop() {
            if pos == self.end {
                return total_risk;
            }
            if total_risk > distances[pos.0][pos.1] {
                continue;
            }
            for n_pos in self.neighbors(pos) {
                let n_risk = self.risk[n_pos.0][n_pos.1] as i32;
                let new_total_risk = total_risk + n_risk;
                if new_total_risk < distances[n_pos.0][n_pos.1] {
                    distances[n_pos.0][n_pos.1] = new_total_risk;
                    heap.push(Reverse((new_total_risk, n_pos)));
                }
            }
        }
        unreachable!()
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let cavern = Cavern::from_lines(input.as_lines());
    cavern.lowest_total_risk()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut cavern = Cavern::from_lines(input.as_lines());
    cavern.grow(5);
    cavern.lowest_total_risk()
}
