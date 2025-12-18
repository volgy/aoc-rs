use std::collections::VecDeque;
use z3::{ast::Int, Optimize, SatResult};

aoc::parts!(1, 2);

#[derive(Debug)]
struct Machine {
    lights: u64,
    wiring: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl Machine {
    fn from_line(line: &str) -> Self {
        let groups: Vec<_> = line.split_whitespace().collect();
        let lights = groups[0]
            .trim_matches(|c| "[]".contains(c))
            .chars()
            .rev()
            .fold(0, |acc, c| (acc << 1) + (c == '#') as u64);
        let wiring = groups[1..groups.len() - 1]
            .iter()
            .map(|g| {
                g.trim_matches(|c| "()".contains(c))
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect()
            })
            .collect();
        let joltage = groups
            .last()
            .unwrap()
            .trim_matches(|c| "{}".contains(c))
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();
        Self {
            lights,
            wiring,
            joltage,
        }
    }

    fn turn_on(&self) -> Option<usize> {
        let mut queue = VecDeque::new();
        let light_wiring: Vec<_> = self
            .wiring
            .iter()
            .map(|w| w.iter().fold(0, |acc, x| acc | 1 << x))
            .collect();
        queue.push_back((self.lights, 0));
        while let Some((lights, steps)) = queue.pop_front() {
            if lights == 0 {
                return Some(steps);
            }
            queue.extend(light_wiring.iter().map(|w| (lights ^ w, steps + 1)))
        }
        None
    }

    fn reach_joltage(&self) -> Option<usize> {
        // Do not want to implement Gauss elimination by hand...
        // so using Z3 SMT solver instead.

        let opt = Optimize::new();
        // x_j = number of presses for button j
        let xs: Vec<Int> = (0..self.wiring.len())
            .map(|j| Int::new_const(format!("x{j}")))
            .collect();

        // x_j >= 0
        for x in &xs {
            opt.assert(&x.ge(Int::from_i64(0)));
        }

        // For each counter i: sum_{j toggles i} x_j == target[i]
        for (i, &b_i) in self.joltage.iter().enumerate() {
            let mut terms: Vec<Int> = Vec::new();
            for (j, btn) in self.wiring.iter().enumerate() {
                if btn.contains(&i) {
                    terms.push(xs[j].clone());
                }
            }
            let sum = Int::add(&terms.iter().collect::<Vec<_>>());
            opt.assert(&sum.eq(Int::from_i64(b_i as i64)));
        }

        // minimize total presses: sum_j x_j
        let total = Int::add(&xs.iter().collect::<Vec<_>>());
        opt.minimize(&total);

        match opt.check(&[]) {
            SatResult::Sat => {
                let model = opt.get_model().unwrap();
                model
                    .eval(&total, true)
                    .and_then(|v| v.as_i64())
                    .map(|v| v as usize)
            }
            _ => None,
        }
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(Machine::from_line)
        .map(|m| m.turn_on().unwrap())
        .sum::<usize>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(Machine::from_line)
        .map(|m| m.reach_joltage().unwrap())
        .sum::<usize>()
}
