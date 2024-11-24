use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

use aoc::Parse;

aoc::parts!(1, 2);

#[derive(Debug)]
struct Valve<'a> {
    rate: u32,
    neighbors: Vec<&'a str>,
}

impl<'a> Valve<'a> {
    fn new(rate: u32, neighbors: Vec<&'a str>) -> Self {
        Self { rate, neighbors }
    }
}

#[derive(Debug, Clone)]
struct State<'a> {
    loc: &'a str,
    opened: HashSet<&'a str>,
    steps: u32,
    release: u32,
}

#[derive(Debug)]
struct Network<'a> {
    valves: HashMap<&'a str, Valve<'a>>,
}

impl<'a> Network<'a> {
    fn parse(input: aoc::Input<'a>) -> Self {
        let mut valves = HashMap::new();
        for line in input.lines() {
            let mut parser = line.as_parser();
            let name = parser.between("Valve ", " has");
            let rate = parser.between("rate=", ";").parse().unwrap();
            let neighbors = parser
                .after("to valve")
                .trim_start_matches("s")
                .split(",")
                .map(str::trim)
                .collect::<Vec<_>>();

            valves.insert(name, Valve::new(rate, neighbors));
        }
        Self { valves }
    }

    // Dijkstra
    fn distance(&self, a: &str, b: &str) -> u32 {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back((a, 0));
        visited.insert(a);

        while let Some((valve, distance)) = queue.pop_front() {
            if valve == b {
                return distance;
            }
            for neighbor in self.valves[valve].neighbors.iter() {
                if !visited.contains(neighbor) {
                    queue.push_back((neighbor, distance + 1));
                    visited.insert(neighbor);
                }
            }
        }
        unreachable!()
    }

    fn valid_paths(&self, end_time: u32, early_finish: bool) -> Vec<State> {
        let distances: HashMap<_, _> = self
            .valves
            .keys()
            .tuple_combinations()
            .flat_map(|(&a, &b)| {
                let d = self.distance(a, b);
                [((a, b), d), ((b, a), d)]
            })
            .collect();

        let mut states = vec![State {
            loc: "AA",
            opened: HashSet::new(),
            steps: 0,
            release: 0,
        }];
        let mut finished_states = vec![];
        while !states.is_empty() {
            let state = states.pop().unwrap();
            let mut progress = false;
            for (&next_loc, next_valve) in self.valves.iter() {
                if !state.opened.contains(next_loc) && next_valve.rate > 0 {
                    let new_steps = state.steps + distances[&(state.loc, next_loc)] + 1;
                    if new_steps < end_time {
                        let mut new_state = state.clone();
                        new_state.loc = next_loc;
                        new_state.opened.insert(next_loc);
                        new_state.steps = new_steps;
                        new_state.release += (end_time - new_steps) * next_valve.rate;
                        states.push(new_state);
                        progress = true;
                    }
                }
            }
            if early_finish || !progress {
                finished_states.push(state);
            }
        }
        finished_states
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    Network::parse(input)
        .valid_paths(30, false)
        .iter()
        .map(|s| s.release)
        .max()
        .unwrap()
}

fn part_2(input: aoc::Input) -> impl ToString {
    Network::parse(input)
        .valid_paths(26, true)
        .iter()
        .tuple_combinations()
        .filter_map(|(p1, p2)| {
            if p1.opened.is_disjoint(&p2.opened) {
                Some(p1.release + p2.release)
            } else {
                None
            }
        })
        .max()
        .unwrap()
}
