use std::collections::{HashSet, VecDeque};

aoc::parts!(1, 2);

type Pos = (usize, usize);

struct Map {
    road: HashSet<Pos>,
    start: Pos,
    end: Pos,
}

impl Map {
    fn parse(input: aoc::Input) -> Self {
        let (mut road, mut start, mut end) = (HashSet::new(), None, None);
        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, ch)| {
                let pos = (x, y);
                match ch {
                    '.' => {
                        road.insert(pos);
                    }
                    'S' => {
                        road.insert(pos);
                        start = Some(pos);
                    }
                    'E' => {
                        road.insert(pos);
                        end = Some(pos);
                    }
                    _ => {}
                }
            });
        });
        Self {
            road,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }

    fn shortest_path(&self) -> Vec<Pos> {
        // BFS - simple Dijkstra
        let mut paths = VecDeque::new();
        let mut visited = HashSet::new();

        paths.push_back(vec![self.start]);
        while let Some(path) = paths.pop_front() {
            let pos = *path.last().unwrap();

            if pos == self.end {
                return path;
            }

            if !visited.insert(pos) {
                continue;
            }
            for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let next_pos = (pos.0.wrapping_add_signed(dx), pos.1.wrapping_add_signed(dy));
                if self.road.contains(&next_pos) {
                    let mut next_path = path.clone();
                    next_path.push(next_pos);
                    paths.push_back(next_path);
                }
            }
        }
        unreachable!()
    }

    fn cheats(&self, max_cheat_steps: usize) -> Vec<usize> {
        assert!(max_cheat_steps >= 2);
        let shortest_path = self.shortest_path();
        let mut cheats = vec![];

        for (idx, pos) in shortest_path.iter().enumerate() {
            for cheat_steps in 2..=max_cheat_steps {
                for abs_dx in 0..=cheat_steps {
                    let abs_dy = cheat_steps - abs_dx;

                    let mut dxdy = vec![
                        (abs_dx as isize, abs_dy as isize),
                        (-(abs_dx as isize), -(abs_dy as isize)),
                    ];
                    if abs_dx > 0 && abs_dy > 0 {
                        dxdy.push((-(abs_dx as isize), abs_dy as isize));
                        dxdy.push((abs_dx as isize, -(abs_dy as isize)));
                    }
                    for (dx, dy) in dxdy {
                        let cheat_pos =
                            (pos.0.wrapping_add_signed(dx), pos.1.wrapping_add_signed(dy));
                        if let Some(cheat_idx) = shortest_path.iter().position(|&p| p == cheat_pos)
                        {
                            if let Some(cheat) = cheat_idx.checked_sub(idx + cheat_steps) {
                                cheats.push(cheat);
                            }
                        }
                    }
                }
            }
        }
        cheats
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    Map::parse(input)
        .cheats(2)
        .into_iter()
        .filter(|&cheat| cheat >= 100)
        .count()
}

fn part_2(input: aoc::Input) -> impl ToString {
    Map::parse(input)
        .cheats(20)
        .into_iter()
        .filter(|&cheat| cheat >= 100)
        .count()
}
