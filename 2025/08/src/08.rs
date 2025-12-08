use itertools::Itertools;
use std::{collections::HashMap, iter::repeat};

aoc::parts!(1, 2);

#[derive(Debug, Clone)]
struct Junction(i32, i32, i32);

impl Junction {
    fn from_line(line: &str) -> Self {
        let (x, y, z) = line
            .split(',')
            .map(|t| t.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Self(x, y, z)
    }

    fn distance(&self, other: &Junction) -> f64 {
        let dx = (self.0 - other.0) as f64;
        let dy = (self.1 - other.1) as f64;
        let dz = (self.2 - other.2) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt() // square root is not really necessary for comparison
    }
}

#[derive(Debug)]
struct Decoration {
    junctions: Vec<Junction>,
    junction_to_circuit: HashMap<usize, usize>,
    circuit_to_junctions: HashMap<usize, Vec<usize>>,
    links: Vec<(f64, (usize, usize))>,
}

impl Decoration {
    fn from_lines(lines: &[&str]) -> Self {
        let junctions: Vec<_> = lines
            .iter()
            .map(|&line| Junction::from_line(line))
            .collect();

        let mut junction_to_circuit: HashMap<usize, usize> = HashMap::new();
        let mut circuit_to_junctions: HashMap<usize, Vec<usize>> = HashMap::new();
        for i in 0..junctions.len() {
            junction_to_circuit.insert(i, i);
            circuit_to_junctions.insert(i, vec![i]);
        }

        // order: longest links first
        let links = (0..junctions.len())
            .tuple_combinations()
            .map(|(i1, i2)| (junctions[i1].distance(&junctions[i2]), (i1, i2)))
            .sorted_by(|(d1, ..), (d2, ..)| d2.partial_cmp(d1).unwrap())
            .collect();

        Self {
            junctions,
            junction_to_circuit,
            circuit_to_junctions,
            links,
        }
    }

    fn connect_next(&mut self) -> (Junction, Junction) {
        let (_dist, (i1, i2)) = self.links.pop().unwrap();
        let c1 = self.junction_to_circuit[&i1];
        let c2 = self.junction_to_circuit[&i2];
        if c1 != c2 {
            for i in self.circuit_to_junctions[&c2].iter() {
                self.junction_to_circuit.insert(*i, c1);
            }
            let c2_junctions = self.circuit_to_junctions.remove(&c2).unwrap();
            self.circuit_to_junctions
                .get_mut(&c1)
                .unwrap()
                .extend(c2_junctions);
        }
        (self.junctions[i1].clone(), self.junctions[i2].clone())
    }

    fn circuits(&self) -> usize {
        self.circuit_to_junctions.len()
    }

    fn top_circuits_sizes(&self, n: usize) -> Vec<usize> {
        self.circuit_to_junctions
            .values()
            .map(|js| js.len())
            .sorted()
            .rev()
            .take(n)
            .collect()
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut decoration = Decoration::from_lines(input.as_lines());

    // use 0..10 for the example
    for _ in 0..1000 {
        decoration.connect_next();
    }
    decoration.top_circuits_sizes(3).iter().product::<usize>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut decoration = Decoration::from_lines(input.as_lines());

    let (j1, j2) = repeat(())
        .find_map(|_| {
            let (j1, j2) = decoration.connect_next();
            if decoration.circuits() == 1 {
                Some((j1, j2))
            } else {
                None
            }
        })
        .unwrap();

    j1.0 * j2.0
}
