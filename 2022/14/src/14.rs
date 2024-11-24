use std::ops::{Index, IndexMut};

aoc::parts!(1, 2);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn parse(s: &str) -> Self {
        let mut parts = s.split(',');
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
struct BBox {
    min: Coord,
    max: Coord,
}

impl BBox {
    fn new(min: Coord, max: Coord) -> Self {
        Self { min, max }
    }

    fn extend(&mut self, coord: Coord) {
        self.min.x = self.min.x.min(coord.x);
        self.min.y = self.min.y.min(coord.y);
        self.max.x = self.max.x.max(coord.x);
        self.max.y = self.max.y.max(coord.y);
    }
}

#[derive(Debug)]
struct Path {
    steps: Vec<Coord>,
}

impl Path {
    fn parse(s: &str) -> Self {
        let steps = s.split(" -> ").map(Coord::parse).collect();
        Self { steps }
    }
    fn bbox(&self) -> BBox {
        let mut bbox = BBox::new(self.steps[0], self.steps[0]);
        for step in &self.steps[1..] {
            bbox.extend(*step);
        }
        bbox
    }
}

#[derive(Debug)]
struct Cave {
    grid: Vec<Vec<bool>>,
}

impl Cave {
    const SPRING: Coord = Coord { x: 500, y: 0 };

    fn parse(input: aoc::Input, has_floor: bool) -> Self {
        let paths: Vec<_> = input.lines().map(Path::parse).collect();
        let mut bbox = BBox::new(paths[0].steps[0], paths[0].steps[0]);
        for path in &paths {
            bbox.extend(path.bbox().min);
            bbox.extend(path.bbox().max);
        }

        let extent = if has_floor {
            (
                (Self::SPRING.x + bbox.max.y + 3) as usize,
                (bbox.max.y + 3) as usize,
            )
        } else {
            // Could optimize here by calculating the min. x offset
            // extra x space to let send drop at the bottom, only
            ((bbox.max.x + 2) as usize, (bbox.max.y + 1) as usize)
        };

        let mut grid = vec![vec![false; extent.0]; extent.1];
        for path in paths {
            let mut x = path.steps[0].x;
            let mut y = path.steps[0].y;
            grid[y as usize][x as usize] = true;
            for step in &path.steps[1..] {
                let dx = step.x - x;
                let dy = step.y - y;
                let abs_dx = dx.abs();
                let abs_dy = dy.abs();
                let steps = abs_dx.max(abs_dy);
                for i in 1..=steps {
                    let x = x + i * dx / steps;
                    let y = y + i * dy / steps;
                    grid[y as usize][x as usize] = true;
                }
                x = step.x;
                y = step.y;
            }
        }
        if has_floor {
            grid[extent.1 - 1] = vec![true; extent.0];
        }
        Self { grid }
    }

    #[allow(unused)]
    fn print(&self) {
        for row in &self.grid {
            for cell in row {
                print!("{}", if *cell { '#' } else { '.' });
            }
            println!();
        }
    }

    #[allow(unused)]
    #[inline]
    fn size_x(&self) -> i32 {
        self.grid[0].len() as i32
    }

    #[inline]
    fn size_y(&self) -> i32 {
        self.grid.len() as i32
    }

    fn pour(&mut self) -> i32 {
        let mut n_grains = 0;
        loop {
            let mut pos = Self::SPRING;
            if self[pos] {
                break;
            }
            while pos.y < self.size_y() - 1 {
                if !self[(pos.x, pos.y + 1)] {
                    pos.y += 1;
                } else if !self[(pos.x - 1, pos.y + 1)] {
                    pos.x -= 1;
                    pos.y += 1;
                } else if !self[(pos.x + 1, pos.y + 1)] {
                    pos.x += 1;
                    pos.y += 1;
                } else {
                    break;
                }
            }
            if pos.y >= self.size_y() - 1 {
                break;
            }
            self[pos] = true;
            n_grains += 1;
        }

        n_grains
    }
}

impl Index<Coord> for Cave {
    type Output = bool;

    fn index(&self, index: Coord) -> &Self::Output {
        &self.grid[index.y as usize][index.x as usize]
    }
}

impl IndexMut<Coord> for Cave {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self.grid[index.y as usize][index.x as usize]
    }
}

impl Index<(i32, i32)> for Cave {
    type Output = bool;

    fn index(&self, index: (i32, i32)) -> &Self::Output {
        self.index(Coord::new(index.0, index.1))
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut cave = Cave::parse(input, false);
    cave.pour()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut cave = Cave::parse(input, true);
    cave.pour()
}
