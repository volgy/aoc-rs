use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, iter::repeat_n, num::ParseIntError, ops::Sub, str::FromStr};
use thiserror::Error;

aoc::parts!(1, 2);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos(i32, i32, i32);

#[derive(Error, Debug)]
enum ParsePosError {
    #[error("invalid coordinate")]
    Coordinate(#[from] ParseIntError),
    #[error("invalid number of coordinates")]
    Dimension,
}

impl FromStr for Pos {
    type Err = ParsePosError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<i32> = s.split(',').map(|v| v.parse()).collect::<Result<_, _>>()?;
        let (x, y, z) = coords
            .into_iter()
            .collect_tuple()
            .ok_or(ParsePosError::Dimension)?;
        Ok(Self(x, y, z))
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Pos {
    fn manhattan_distance(&self, other: &Pos) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs() + (self.2 - other.2).abs()
    }
}

struct Rotation([[i32; 3]; 3]);

impl Rotation {
    fn all() -> Vec<Self> {
        let mut rotations = Vec::new();
        for order in [0, 1, 2].iter().permutations(3) {
            for signs in repeat_n([-1, 1], 3).multi_cartesian_product() {
                let mut m = [[0; 3]; 3];
                for (i, &axis) in order.iter().enumerate() {
                    m[i][*axis] = signs[i];
                }

                // filter out reflections (determinant is +1)
                let det = m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
                    - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
                    + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]);

                if det == 1 {
                    rotations.push(Rotation(m));
                }
            }
        }
        rotations
    }

    fn apply(&self, p: Pos) -> Pos {
        let m = self.0;
        Pos(
            p.0 * m[0][0] + p.1 * m[0][1] + p.2 * m[0][2],
            p.0 * m[1][0] + p.1 * m[1][1] + p.2 * m[1][2],
            p.0 * m[2][0] + p.1 * m[2][1] + p.2 * m[2][2],
        )
    }
}

#[derive(Debug)]
struct Scanner {
    id: usize,
    beacons: Vec<Pos>,
    shift: Pos,
}

impl Scanner {
    fn parse_all(lines: &[&str]) -> Vec<Self> {
        lines
            .split(|line| line.is_empty())
            .map(Scanner::parse)
            .collect()
    }

    fn parse(lines: &[&str]) -> Self {
        let header = Regex::new(r"^--- scanner (\d+) ---$")
            .unwrap()
            .captures(lines[0])
            .expect("invalid scanner header");
        let id = header[1].parse().unwrap();
        let beacons = lines[1..]
            .iter()
            .map(|line| line.parse().unwrap())
            .collect();
        Self {
            id,
            beacons,
            shift: Pos(0, 0, 0),
        }
    }

    fn rotate(&self, rotation: &Rotation) -> Self {
        Self {
            id: self.id,
            beacons: self.beacons.iter().map(|&b| rotation.apply(b)).collect(),
            shift: self.shift,
        }
    }

    fn shift(&self, offset: Pos) -> Self {
        Self {
            id: self.id,
            beacons: self
                .beacons
                .iter()
                .map(|&b| Pos(b.0 + offset.0, b.1 + offset.1, b.2 + offset.2))
                .collect(),
            shift: Pos(
                self.shift.0 + offset.0,
                self.shift.1 + offset.1,
                self.shift.2 + offset.2,
            ),
        }
    }
}

fn align_scanners(scanners: Vec<Scanner>) -> Vec<Scanner> {
    let rotations = Rotation::all();

    let mut scanners = scanners.into_iter();
    let mut aligned = HashMap::new();
    let first_scanner = scanners.next().unwrap();
    aligned.insert(0, first_scanner);
    let mut unaligned: HashMap<usize, Scanner> = scanners.map(|s| (s.id, s)).collect();

    while !unaligned.is_empty() {
        let mut newly_aligned = Vec::new();

        for (&_aligned_id, aligned_scanner) in &aligned {
            let aligned_beacons: &Vec<Pos> = &aligned_scanner.beacons;

            let unaligned_ids: Vec<usize> = unaligned.keys().cloned().collect();
            for unaligned_id in unaligned_ids {
                let unaligned_scanner = unaligned.get(&unaligned_id).unwrap();

                'rotation: for rotation in &rotations {
                    let rotated_scanner = unaligned_scanner.rotate(rotation);

                    let mut offset_counts: HashMap<Pos, usize> = HashMap::new();
                    for &ab in aligned_beacons {
                        for &rb in &rotated_scanner.beacons {
                            let offset = ab - rb;
                            *offset_counts.entry(offset).or_default() += 1;
                        }
                    }

                    for (&offset, &count) in &offset_counts {
                        if count >= 12 {
                            // found alignment
                            newly_aligned.push((unaligned_id, rotated_scanner.shift(offset)));
                            break 'rotation;
                        }
                    }
                }
            }
        }

        for (id, scanner) in newly_aligned {
            unaligned.remove(&id);
            aligned.insert(id, scanner);
        }
    }
    aligned.into_values().collect()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let scanners = Scanner::parse_all(input.as_lines());

    let mut unique_beacons: HashMap<(i32, i32, i32), ()> = HashMap::new();
    for scanner in align_scanners(scanners) {
        for beacon in scanner.beacons {
            unique_beacons.insert((beacon.0, beacon.1, beacon.2), ());
        }
    }
    unique_beacons.len()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let scanners = Scanner::parse_all(input.as_lines());
    let aligned_scanners = align_scanners(scanners);

    aligned_scanners
        .iter()
        .tuple_combinations()
        .map(|(s1, s2)| s1.shift.manhattan_distance(&s2.shift))
        .max()
        .unwrap()
}
