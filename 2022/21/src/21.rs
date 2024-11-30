use std::collections::HashMap;

aoc::parts!(1, 2);

#[derive(Debug)]
enum Monkey<'a> {
    Number(i64),
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
}

impl<'a> Monkey<'a> {
    const HUMAN: &'static str = "humn";

    fn parse(text: &'a str) -> Self {
        let tokens: Vec<_> = text.split_whitespace().collect();
        match tokens[..] {
            [number] => Self::Number(number.parse().unwrap()),
            [left, "+", right] => Self::Add(left, right),
            [left, "-", right] => Self::Sub(left, right),
            [left, "*", right] => Self::Mul(left, right),
            [left, "/", right] => Self::Div(left, right),
            _ => unreachable!(),
        }
    }

    fn operands(&self) -> Option<(&'a str, &'a str)> {
        match self {
            Monkey::Number(_) => None,
            Monkey::Add(left, right)
            | Monkey::Sub(left, right)
            | Monkey::Mul(left, right)
            | Monkey::Div(left, right) => Some((left, right)),
        }
    }

    fn evaluate(&self, monkeys: &HashMap<&str, Self>) -> i64 {
        match self {
            Monkey::Number(n) => *n,
            Monkey::Add(left, right) => {
                monkeys[left].evaluate(monkeys) + monkeys[right].evaluate(monkeys)
            }
            Monkey::Sub(left, right) => {
                monkeys[left].evaluate(monkeys) - monkeys[right].evaluate(monkeys)
            }
            Monkey::Mul(left, right) => {
                monkeys[left].evaluate(monkeys) * monkeys[right].evaluate(monkeys)
            }
            Monkey::Div(left, right) => {
                monkeys[left].evaluate(monkeys) / monkeys[right].evaluate(monkeys)
            }
        }
    }

    fn depends_human(name: &str, monkeys: &HashMap<&str, Self>) -> bool {
        if name == Self::HUMAN {
            true
        } else {
            match monkeys[name].operands() {
                None => false,
                Some((left, right)) => {
                    Monkey::depends_human(left, monkeys) || Monkey::depends_human(right, monkeys)
                }
            }
        }
    }

    fn solve(&self, name: &str, value: i64, monkeys: &HashMap<&str, Self>) -> i64 {
        if name == Self::HUMAN {
            return value;
        }

        match self {
            Monkey::Add(left, right) => {
                if Monkey::depends_human(left, monkeys) {
                    monkeys[left].solve(left, value - monkeys[right].evaluate(monkeys), monkeys)
                } else {
                    monkeys[right].solve(right, value - monkeys[left].evaluate(monkeys), monkeys)
                }
            }
            Monkey::Sub(left, right) => {
                if Monkey::depends_human(left, monkeys) {
                    monkeys[left].solve(left, value + monkeys[right].evaluate(monkeys), monkeys)
                } else {
                    monkeys[right].solve(right, monkeys[left].evaluate(monkeys) - value, monkeys)
                }
            }
            Monkey::Mul(left, right) => {
                if Monkey::depends_human(left, monkeys) {
                    monkeys[left].solve(left, value / monkeys[right].evaluate(monkeys), monkeys)
                } else {
                    monkeys[right].solve(right, value / monkeys[left].evaluate(monkeys), monkeys)
                }
            }
            Monkey::Div(left, right) => {
                if Monkey::depends_human(left, monkeys) {
                    monkeys[left].solve(left, value * monkeys[right].evaluate(monkeys), monkeys)
                } else {
                    monkeys[right].solve(right, monkeys[left].evaluate(monkeys) / value, monkeys)
                }
            }
            _ => unreachable!(),
        }
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let monkeys: HashMap<_, _> = input
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(name, text)| (name, Monkey::parse(text)))
        .collect();
    monkeys["root"].evaluate(&monkeys)
}

fn part_2(input: aoc::Input) -> impl ToString {
    let monkeys: HashMap<_, _> = input
        .lines()
        .map(|l| l.split_once(": ").unwrap())
        .map(|(name, text)| (name, Monkey::parse(text)))
        .collect();

    let (left, right) = monkeys["root"].operands().unwrap();
    if Monkey::depends_human(left, &monkeys) {
        monkeys[left].solve(left, monkeys[right].evaluate(&monkeys), &monkeys)
    } else {
        monkeys[right].solve(right, monkeys[left].evaluate(&monkeys), &monkeys)
    }
}
