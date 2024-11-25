use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

aoc::parts!(1, 2);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cube(i32, i32, i32);

impl Cube {
    fn neighbors(&self) -> Vec<Self> {
        vec![
            Self(self.0 + 1, self.1, self.2),
            Self(self.0 - 1, self.1, self.2),
            Self(self.0, self.1 + 1, self.2),
            Self(self.0, self.1 - 1, self.2),
            Self(self.0, self.1, self.2 + 1),
            Self(self.0, self.1, self.2 - 1),
        ]
    }
}

fn parse_cubes(input: aoc::Input) -> HashSet<Cube> {
    input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|c| c.parse().unwrap())
                .collect_tuple()
                .map(|(x, y, z)| Cube(x, y, z))
                .unwrap()
        })
        .collect()
}

fn all_faces(cubes: &HashSet<Cube>) -> u32 {
    let mut faces = 0;
    for cube in cubes.iter() {
        faces += 6;
        for neighbor in cube.neighbors() {
            if cubes.contains(&neighbor) {
                faces -= 1;
            }
        }
    }
    faces
}

fn part_1(input: aoc::Input) -> impl ToString {
    all_faces(&parse_cubes(input))
}

fn part_2(input: aoc::Input) -> impl ToString {
    let cubes = parse_cubes(input);
    let mut faces = all_faces(&cubes);

    let min_pos = cubes.iter().fold((i32::MAX, i32::MAX, i32::MAX), |a, c| {
        (a.0.min(c.0), a.1.min(c.1), a.2.min(c.2))
    });
    let max_pos = cubes.iter().fold((i32::MIN, i32::MIN, i32::MIN), |a, c| {
        (a.0.max(c.0), a.1.max(c.1), a.2.max(c.2))
    });

    let mut air_cubes = HashSet::new();
    for x in (min_pos.0 - 1)..=(max_pos.0 + 1) {
        for y in (min_pos.1 - 1)..=(max_pos.1 + 1) {
            for z in (min_pos.2 - 1)..=(max_pos.2 + 1) {
                let cube = Cube(x, y, z);
                if !cubes.contains(&cube) {
                    air_cubes.insert(cube);
                }
            }
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(Cube(min_pos.0 - 1, min_pos.1 - 1, min_pos.2 - 1));
    while !queue.is_empty() {
        let cube = queue.pop_front().unwrap();
        for neighbor in cube.neighbors() {
            if air_cubes.contains(&neighbor) {
                air_cubes.remove(&neighbor);
                queue.push_back(neighbor);
            }
        }
    }

    for air_cube in air_cubes {
        for neigbhbor in air_cube.neighbors() {
            if cubes.contains(&neigbhbor) {
                faces -= 1;
            }
        }
    }

    faces
}
