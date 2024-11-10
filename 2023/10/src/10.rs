aoc::parts!(1, 2);

use std::collections::HashMap;
use std::iter;

type Pos = (isize, isize);

struct Maze(HashMap<Pos, char>);

impl Maze {
    fn parse(input: aoc::Input) -> Maze {
        Self(
            input
                .lines()
                .enumerate()
                .flat_map(|(row, line)| iter::repeat(row).enumerate().zip(line.chars()))
                .map(|((col, row), c)| ((col as isize, row as isize), c))
                .collect(),
        )
    }

    fn start(&self) -> Pos {
        self.0
            .iter()
            .find_map(|(&pos, c)| match c {
                'S' => Some(pos),
                _ => None,
            })
            .unwrap()
    }

    fn longest_path(&self) -> (Vec<Pos>, char) {
        let start = self.start();
        let mut paths = Vec::new();

        for start_dir in "NESW".chars() {
            let mut dir = start_dir;
            let mut pos = start;
            let mut path = vec![pos];
            loop {
                let north_pos = (pos.0, pos.1 - 1);
                let east_pos = (pos.0 + 1, pos.1);
                let south_pos = (pos.0, pos.1 + 1);
                let west_pos = (pos.0 - 1, pos.1);

                let north = self.0.get(&north_pos);
                let east = self.0.get(&east_pos);
                let south = self.0.get(&south_pos);
                let west = self.0.get(&west_pos);

                // little hack: dir contains the actual start symbol at the end
                (dir, pos) = match (dir, north, east, south, west) {
                    ('N', Some('|'), ..) => ('N', north_pos),
                    ('N', Some('F'), ..) => ('E', north_pos),
                    ('N', Some('7'), ..) => ('W', north_pos),
                    ('N', Some('S'), ..) => (
                        match start_dir {
                            'E' => 'F',
                            'N' => '|',
                            'W' => '7',
                            _ => panic!(),
                        },
                        north_pos,
                    ),
                    ('E', _, Some('-'), ..) => ('E', east_pos),
                    ('E', _, Some('J'), ..) => ('N', east_pos),
                    ('E', _, Some('7'), ..) => ('S', east_pos),
                    ('E', _, Some('S'), ..) => (
                        match start_dir {
                            'N' => 'J',
                            'E' => '-',
                            'S' => '7',
                            _ => panic!(),
                        },
                        east_pos,
                    ),
                    ('S', .., Some('|'), _) => ('S', south_pos),
                    ('S', .., Some('L'), _) => ('E', south_pos),
                    ('S', .., Some('J'), _) => ('W', south_pos),
                    ('S', .., Some('S'), _) => (
                        match start_dir {
                            'E' => 'L',
                            'S' => '|',
                            'W' => 'J',
                            _ => panic!(),
                        },
                        south_pos,
                    ),
                    ('W', .., Some('-')) => ('W', west_pos),
                    ('W', .., Some('L')) => ('N', west_pos),
                    ('W', .., Some('F')) => ('S', west_pos),
                    ('W', .., Some('S')) => (
                        match start_dir {
                            'N' => 'L',
                            'W' => '-',
                            'S' => 'F',
                            _ => panic!(),
                        },
                        west_pos,
                    ),
                    _ => break,
                };

                path.push(pos);

                if self.0[&pos] == 'S' {
                    paths.push((path, dir));
                    break;
                }
            }
        }
        paths
            .into_iter()
            .reduce(|a, b| if a.0.len() >= b.0.len() { a } else { b })
            .unwrap()
    }

    fn enclosed_area(&self) -> usize {
        let (path, start_symbol) = self.longest_path();

        let (max_x, max_y) = self.0.keys().max().unwrap();

        let mut area = 0;
        for y in 0..=*max_y {
            let mut inside = false;
            let mut prev_elbow = None;
            for x in 0..=*max_x {
                let pos = (x, y);
                let symbol = self.0[&pos];
                let symbol = if symbol == 'S' { start_symbol } else { symbol };
                if path.contains(&pos) {
                    if symbol == '|' {
                        inside = !inside;
                    } else if "FL".contains(symbol) {
                        prev_elbow = Some(symbol);
                    } else if (symbol == '7' && prev_elbow.unwrap() == 'L')
                        || (symbol == 'J' && prev_elbow.unwrap() == 'F')
                    {
                        inside = !inside;
                    }
                } else if inside {
                    area += 1;
                }
            }
        }
        area
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let maze = Maze::parse(input);
    maze.longest_path().0.len() / 2
}

fn part_2(input: aoc::Input) -> impl ToString {
    let maze = Maze::parse(input);
    maze.enclosed_area()
}
