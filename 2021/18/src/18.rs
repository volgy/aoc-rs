use std::{
    fmt::{Display, Formatter},
    iter::Sum,
    ops::Add,
};

use aoc::IterUnwrap;
use itertools::Itertools;

aoc::parts!(1, 2);

#[derive(Clone)]
enum Element {
    Primitive(u8),
    Compound(Pair),
}

impl Element {
    fn parse<I: Iterator<Item = char>>(chars: &mut std::iter::Peekable<I>) -> Self {
        if chars.peek() == Some(&'[') {
            Element::Compound(Pair::parse(chars))
        } else {
            let mut n: u8 = 0;
            while let Some(&d) = chars.peek() {
                if d.is_ascii_digit() {
                    n = n * 10 + (chars.next_uw() as u8 - b'0');
                } else {
                    break;
                }
            }
            Element::Primitive(n)
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Primitive(n) => write!(f, "{n}"),
            Element::Compound(p) => write!(f, "{p}"),
        }
    }
}

#[derive(Clone)]
struct Pair {
    lhs: Box<Element>,
    rhs: Box<Element>,
}

impl Pair {
    fn parse<I: Iterator<Item = char>>(chars: &mut std::iter::Peekable<I>) -> Self {
        assert_eq!(chars.next_uw(), '[');
        let lhs = Box::new(Element::parse(chars));
        assert_eq!(chars.next_uw(), ',');
        let rhs = Box::new(Element::parse(chars));
        assert_eq!(chars.next_uw(), ']');
        Self { lhs, rhs }
    }

    fn magnitude(&self) -> u32 {
        let lhs_mag = match self.lhs.as_ref() {
            Element::Primitive(n) => *n as u32,
            Element::Compound(p) => p.magnitude(),
        };
        let rhs_mag = match self.rhs.as_ref() {
            Element::Primitive(n) => *n as u32,
            Element::Compound(p) => p.magnitude(),
        };
        3 * lhs_mag + 2 * rhs_mag
    }

    fn reduce(&mut self) {
        while self.try_explode(0).is_some() || self.try_split() {}
    }

    fn try_explode(&mut self, level: usize) -> Option<(Option<u8>, Option<u8>)> {
        if level == 4 {
            // At level 4, we need to explode this pair
            let left_val = match self.lhs.as_ref() {
                Element::Primitive(n) => *n,
                _ => panic!("Expected primitive at level 4"),
            };
            let right_val = match self.rhs.as_ref() {
                Element::Primitive(n) => *n,
                _ => panic!("Expected primitive at level 4"),
            };
            return Some((Some(left_val), Some(right_val)));
        }

        // Try to explode left side first
        if let Element::Compound(ref mut pair) = self.lhs.as_mut() {
            if let Some((left, right)) = pair.try_explode(level + 1) {
                if level == 3 {
                    // Replace the exploded pair with 0
                    self.lhs = Box::new(Element::Primitive(0));
                }
                // Add right value to the leftmost of right side
                if let Some(r) = right {
                    Self::add_leftmost(&mut self.rhs, r);
                }
                return Some((left, None));
            }
        }

        // Try to explode right side
        if let Element::Compound(ref mut pair) = self.rhs.as_mut() {
            if let Some((left, right)) = pair.try_explode(level + 1) {
                if level == 3 {
                    // Replace the exploded pair with 0
                    self.rhs = Box::new(Element::Primitive(0));
                }
                // Add left value to the rightmost of left side
                if let Some(l) = left {
                    Self::add_rightmost(&mut self.lhs, l);
                }
                return Some((None, right));
            }
        }

        None
    }

    fn add_leftmost(elem: &mut Box<Element>, val: u8) {
        match elem.as_mut() {
            Element::Primitive(n) => *n += val,
            Element::Compound(pair) => Self::add_leftmost(&mut pair.lhs, val),
        }
    }

    fn add_rightmost(elem: &mut Box<Element>, val: u8) {
        match elem.as_mut() {
            Element::Primitive(n) => *n += val,
            Element::Compound(pair) => Self::add_rightmost(&mut pair.rhs, val),
        }
    }

    fn try_split(&mut self) -> bool {
        // Try to split left side first
        match self.lhs.as_mut() {
            Element::Primitive(n) if *n >= 10 => {
                let left_val = *n / 2;
                let right_val = *n - left_val;
                self.lhs = Box::new(Element::Compound(Pair {
                    lhs: Box::new(Element::Primitive(left_val)),
                    rhs: Box::new(Element::Primitive(right_val)),
                }));
                return true;
            }
            Element::Compound(pair) => {
                if pair.try_split() {
                    return true;
                }
            }
            _ => {}
        }

        // Try to split right side
        match self.rhs.as_mut() {
            Element::Primitive(n) if *n >= 10 => {
                let left_val = *n / 2;
                let right_val = *n - left_val;
                self.rhs = Box::new(Element::Compound(Pair {
                    lhs: Box::new(Element::Primitive(left_val)),
                    rhs: Box::new(Element::Primitive(right_val)),
                }));
                return true;
            }
            Element::Compound(pair) => {
                if pair.try_split() {
                    return true;
                }
            }
            _ => {}
        }

        false
    }
}

impl<T: AsRef<str>> From<T> for Pair {
    fn from(line: T) -> Self {
        Self::parse(&mut line.as_ref().chars().peekable())
    }
}

impl Display for Pair {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.lhs, self.rhs)
    }
}

impl Add for Pair {
    type Output = Pair;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Pair {
            lhs: Box::new(Element::Compound(self)),
            rhs: Box::new(Element::Compound(rhs)),
        };
        result.reduce();
        result
    }
}

impl Sum for Pair {
    fn sum<I: Iterator<Item = Self>>(it: I) -> Self {
        it.reduce(|a, b| a + b).unwrap()
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    input.lines().map(Pair::from).sum::<Pair>().magnitude()
}

fn part_2(input: aoc::Input) -> impl ToString {
    input
        .lines()
        .map(Pair::from)
        .permutations(2)
        .map(|pairs| pairs.into_iter().sum::<Pair>().magnitude())
        .max()
        .unwrap()
}
