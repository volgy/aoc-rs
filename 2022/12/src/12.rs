use std::collections::VecDeque;

aoc::parts!(1, 2);

type Size = (usize, usize);
type Loc = (usize, usize);

#[derive(Debug)]
struct Terrain {
    start: Loc,
    end: Loc,
    size: Size,
    elev: Vec<Vec<i32>>,
    visited: Vec<Vec<bool>>,
}

impl Terrain {
    fn parse(input: aoc::Input) -> Self {
        let mut elev = vec![];
        let mut start = (0, 0);
        let mut end = (0, 0);

        for (y, line) in input.lines().enumerate() {
            let mut row = vec![];
            for (x, ch) in line.chars().enumerate() {
                let h = match ch {
                    'S' => {
                        start = (x, y);
                        'a' as i32
                    }
                    'E' => {
                        end = (x, y);
                        'z' as i32
                    }
                    _ => ch as i32,
                };
                row.push(h);
            }
            elev.push(row);
        }

        let size = (elev[0].len(), elev.len());
        let visited = vec![vec![false; size.0]; size.1];

        Self {
            start,
            end,
            size,
            elev,
            visited,
        }
    }

    fn traverse(&mut self) -> i32 {
        let mut queue = VecDeque::new();
        queue.push_back((self.start, 0));
        self.visited[self.start.1][self.start.0] = true;

        while let Some((loc, dist)) = queue.pop_front() {
            if loc == self.end {
                return dist;
            }

            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (x, y) = (loc.0 as isize + dx, loc.1 as isize + dy);
                if x < 0 || y < 0 || x >= self.size.0 as isize || y >= self.size.1 as isize {
                    continue;
                }
                let new_loc = (x as usize, y as usize);
                if !self.visited[new_loc.1][new_loc.0]
                    && self.elev[new_loc.1][new_loc.0] <= self.elev[loc.1][loc.0] + 1
                {
                    self.visited[new_loc.1][new_loc.0] = true;
                    queue.push_back((new_loc, dist + 1));
                }
            }
        }
        unreachable!()
    }

    fn backverse(&mut self) -> i32 {
        let mut queue = VecDeque::new();
        queue.push_back((self.end, 0));
        self.visited[self.end.1][self.end.0] = true;

        while let Some((loc, dist)) = queue.pop_front() {
            if self.elev[loc.1][loc.0] == 'a' as i32 {
                return dist;
            }

            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (x, y) = (loc.0 as isize + dx, loc.1 as isize + dy);
                if x < 0 || y < 0 || x >= self.size.0 as isize || y >= self.size.1 as isize {
                    continue;
                }
                let new_loc = (x as usize, y as usize);
                if !self.visited[new_loc.1][new_loc.0]
                    && self.elev[new_loc.1][new_loc.0] >= self.elev[loc.1][loc.0] - 1
                {
                    self.visited[new_loc.1][new_loc.0] = true;
                    queue.push_back((new_loc, dist + 1));
                }
            }
        }
        unreachable!()
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut terrain = Terrain::parse(input);
    terrain.traverse()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut terrain = Terrain::parse(input);
    terrain.backverse()
}
