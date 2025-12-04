use itertools::Itertools;

aoc::parts!(1, 2);

type Loc = (usize, usize);
struct Grid {
    size: usize,
    occupancy: Vec<Vec<bool>>,
}

impl Grid {
    fn from_lines(lines: &[&str]) -> Self {
        let size = lines.len();
        assert!(size > 0 && size == lines[0].len());
        let occupancy = lines
            .iter()
            .map(|l| l.chars().map(|c| c == '@').collect())
            .collect();
        Self { size, occupancy }
    }

    fn neighbors(&self, loc: Loc) -> impl Iterator<Item = Loc> {
        let bounds = 0..self.size as isize;
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter_map(move |(di, dj)| {
                if di == 0 && dj == 0 {
                    None
                } else {
                    let ni = loc.0 as isize + di;
                    let nj = loc.1 as isize + dj;
                    if bounds.contains(&ni) && bounds.contains(&nj) {
                        Some((ni as usize, nj as usize))
                    } else {
                        None
                    }
                }
            })
    }

    fn accessible(&self) -> Vec<Loc> {
        (0..self.size)
            .cartesian_product(0..self.size)
            .filter(|&(i, j)| {
                self.occupancy[i][j]
                    && self
                        .neighbors((i, j))
                        .filter(|&(ni, nj)| self.occupancy[ni][nj])
                        .count()
                        < 4
            })
            .collect()
    }

    fn remove_accessible(&mut self) -> usize {
        let n = self
            .accessible()
            .into_iter()
            .inspect(|&(i, j)| self.occupancy[i][j] = false)
            .count();
        if n > 0 {
            n + self.remove_accessible()
        } else {
            0
        }
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    Grid::from_lines(input.as_lines()).accessible().len()
}

fn part_2(input: aoc::Input) -> impl ToString {
    Grid::from_lines(input.as_lines()).remove_accessible()
}
