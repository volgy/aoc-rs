use itertools::Itertools;
use std::collections::HashMap;

aoc::parts!(1, 2);

#[derive(Debug, Clone)]
enum Net {
    Value(bool),
    And(String, String),
    Or(String, String),
    Xor(String, String),
}

#[derive(Debug, Clone)]
struct Circuit {
    nets: HashMap<String, Net>,
}

impl Circuit {
    fn parse(input: aoc::Input) -> Self {
        let mut lines = input.lines();

        let mut nets: HashMap<_, _> = lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| line.split_once(": ").unwrap())
            .map(|(name, value)| (name.to_owned(), Net::Value(value == "1")))
            .collect();

        nets.extend(lines.map(|line| {
            let tokens = line.split_whitespace().collect_vec();
            match tokens[..] {
                [lhs, "AND", rhs, "->", net] => {
                    (net.to_owned(), Net::And(lhs.to_owned(), rhs.to_owned()))
                }
                [lhs, "OR", rhs, "->", net] => {
                    (net.to_owned(), Net::Or(lhs.to_owned(), rhs.to_owned()))
                }
                [lhs, "XOR", rhs, "->", net] => {
                    (net.to_owned(), Net::Xor(lhs.to_owned(), rhs.to_owned()))
                }
                _ => unreachable!(),
            }
        }));

        Self { nets }
    }

    fn evaluate(&self, net: &str) -> bool {
        // inefficient (no memoization)
        match &self.nets[net] {
            Net::Value(value) => *value,
            Net::And(lhs, rhs) => self.evaluate(lhs) && self.evaluate(rhs),
            Net::Or(lhs, rhs) => self.evaluate(lhs) || self.evaluate(rhs),
            Net::Xor(lhs, rhs) => self.evaluate(lhs) ^ self.evaluate(rhs),
        }
    }
}

fn bools_to_num<T>(bools: T) -> u64
where
    T: IntoIterator<Item = bool>,
{
    bools
        .into_iter()
        .fold(0, |acc, bit| (acc << 1) + bit as u64)
}

fn part_1(input: aoc::Input) -> impl ToString {
    let circuit = Circuit::parse(input);
    bools_to_num(
        circuit
            .nets
            .keys()
            .filter(|n| n.starts_with("z"))
            .sorted()
            .rev()
            .map(|net| circuit.evaluate(net)),
    )
}

///            ╭───╮        ╭───╮              
///   Xi────╮─►│   │ XYi╭──►│   │              
///         │  │ ^ │───┐╯   │ ^ │──────────►Zi
///   Yi──╮─│─►│   │   │╭──►│   │              
///       │ │  ╰───╯   ││   ╰───╯              
/// Ci-1──│─│────────┐─│╯   ╭───╮              
///       │ │        │ └───►│   │XCi           
///       │ │        │      │ & │──╮           
///       │ │        └─────►│   │  │ ╭───╮     
///       │ │               ╰───╯  ╰►│   │     
///       │ │               ╭───╮    │ | │─►Ci
///       │ ╰──────────────►│   │  ╭►│   │     
///       │                 │ & │──╯ ╰───╯     
///       ╰────────────────►│   │YCi           
///                         ╰───╯              
///
fn part_2(input: aoc::Input) -> impl ToString {
    let circuit = Circuit::parse(input);
    let xs = circuit
        .nets
        .keys()
        .filter(|n| n.starts_with("x"))
        .collect_vec();
    let ys = circuit
        .nets
        .keys()
        .filter(|n| n.starts_with("y"))
        .collect_vec();
    let zs = circuit
        .nets
        .keys()
        .filter(|n| n.starts_with("z"))
        .collect_vec();
    let n_bits = xs.len();
    assert_eq!(ys.len(), n_bits);
    assert_eq!(zs.len(), n_bits + 1); // carry

    for bit in 0..n_bits {
        let mut circuit = circuit.clone();
    }
    0
}
