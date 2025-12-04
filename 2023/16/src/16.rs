use std::collections::HashSet;

aoc::parts!(1, 2);

type Loc = (isize, isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    N,
    E,
    S,
    W,
}

#[derive(Debug, Clone)]
struct Beam {
    path: Vec<Loc>,
    dir: Dir,
}

impl Beam {
    fn new(start: Loc, dir: Dir) -> Self {
        Self {
            path: vec![start],
            dir,
        }
    }

    fn next_loc(&self) -> Loc {
        let (x, y) = self.path.last().copied().unwrap();
        match self.dir {
            Dir::N => (x - 1, y),
            Dir::E => (x, y + 1),
            Dir::S => (x + 1, y),
            Dir::W => (x, y - 1),
        }
    }
}

struct Grid {
    size: usize,
    cells: Vec<Vec<char>>,
}

impl Grid {
    fn from_lines(lines: &[&str]) -> Self {
        let size = lines.len();
        assert!(size > 0 && lines[0].len() == size);
        let cells = lines.iter().map(|l| l.chars().collect()).collect();
        Self { size, cells }
    }

    fn beams(&self, start: Loc, dir: Dir) -> Vec<Beam> {
        let mut active = vec![Beam::new(start, dir)];
        let mut finished = Vec::new();
        let mut visited = HashSet::new();
        let bounds = 0..self.size as isize;

        while let Some(mut beam) = active.pop() {
            let loc = beam.next_loc();

            if !bounds.contains(&loc.0) || !bounds.contains(&loc.1) {
                finished.push(beam);
                continue;
            }

            if visited.contains(&(loc, beam.dir)) {
                finished.push(beam);
                continue;
            }

            visited.insert((loc, beam.dir));
            beam.path.push(loc);

            let new_dirs = match self.cells[loc.0 as usize][loc.1 as usize] {
                '-' => {
                    if matches!(beam.dir, Dir::N | Dir::S) {
                        vec![Dir::W, Dir::E]
                    } else {
                        vec![beam.dir]
                    }
                }

                '|' => {
                    if matches!(beam.dir, Dir::E | Dir::W) {
                        vec![Dir::N, Dir::S]
                    } else {
                        vec![beam.dir]
                    }
                }

                '\\' => vec![match beam.dir {
                    Dir::N => Dir::W,
                    Dir::W => Dir::N,
                    Dir::S => Dir::E,
                    Dir::E => Dir::S,
                }],

                '/' => vec![match beam.dir {
                    Dir::N => Dir::E,
                    Dir::E => Dir::N,
                    Dir::S => Dir::W,
                    Dir::W => Dir::S,
                }],

                _ => vec![beam.dir],
            };
            for dir in new_dirs {
                let mut new_beam = beam.clone();
                new_beam.dir = dir;
                active.push(new_beam);
            }
        }

        finished
    }

    fn energize_top_left(&self) -> usize {
        self.beams((0, 0), Dir::E)
            .into_iter()
            .flat_map(|b| b.path)
            .collect::<HashSet<_>>()
            .len()
    }

    fn energize_max_from_edge(&self) -> usize {
        let last = (self.size - 1) as isize;

        (0..self.size as isize)
            .flat_map(|i| {
                vec![
                    self.beams((0, i), Dir::S),
                    self.beams((last, i), Dir::N),
                    self.beams((i, 0), Dir::E),
                    self.beams((i, last), Dir::W),
                ]
            })
            .map(|beams| {
                beams
                    .into_iter()
                    .flat_map(|b| b.path)
                    .collect::<HashSet<_>>()
                    .len()
            })
            .max()
            .unwrap()
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    Grid::from_lines(input.as_lines()).energize_top_left()
}

fn part_2(input: aoc::Input) -> impl ToString {
    Grid::from_lines(input.as_lines()).energize_max_from_edge()
}
