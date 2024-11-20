use aoc::Parse;
use gcd::Gcd;
use itertools::Itertools;
use std::collections::VecDeque;

aoc::parts!(1, 2);

#[derive(Debug)]
enum Operation {
    Add(i64),
    Multiply(i64),
    Square,
}

enum WorryDecrease {
    Divide(i64),
    Modulus(i64),
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    operation: Operation,
    divide_by: i64,
    next_true_idx: usize,
    next_false_idx: usize,
    inspected: usize,
}

impl Monkey {
    fn parse(lines: &[&str]) -> Self {
        let mut items = VecDeque::new();

        assert!(lines[0].starts_with("Monkey"));

        lines[1]
            .as_parser()
            .after("Starting items:")
            .split(",")
            .map(|x| x.trim().parse().unwrap())
            .for_each(|x| items.push_back(x));

        let (operator, operand) = lines[2]
            .as_parser()
            .after("Operation: new = old ")
            .split_once(" ")
            .unwrap();

        let operation = match (operator, operand) {
            ("*", "old") => Operation::Square,
            ("*", operand) => Operation::Multiply(operand.parse().unwrap()),
            ("+", operand) => Operation::Add(operand.parse().unwrap()),
            _ => unreachable!(),
        };

        let divide_by = lines[3]
            .as_parser()
            .after("Test: divisible by ")
            .parse()
            .unwrap();

        let next_true_idx = lines[4]
            .as_parser()
            .after("If true: throw to monkey ")
            .parse()
            .unwrap();

        let next_false_idx = lines[5]
            .as_parser()
            .after("If false: throw to monkey ")
            .parse()
            .unwrap();

        Self {
            items,
            operation,
            divide_by,
            next_true_idx,
            next_false_idx,
            inspected: 0,
        }
    }

    fn turn(&mut self, worry_decrease: WorryDecrease) -> Vec<(usize, i64)> {
        let mut throws = vec![];
        for _ in 0..self.items.len() {
            let item = self.items.pop_front().unwrap();
            self.inspected += 1;

            let mut new_item = match &self.operation {
                Operation::Add(operand) => item + operand,
                Operation::Multiply(operand) => item * operand,
                Operation::Square => item * item,
            };

            new_item = match worry_decrease {
                WorryDecrease::Divide(divisor) => new_item / divisor,
                WorryDecrease::Modulus(divisor) => new_item % divisor,
            };

            if new_item % self.divide_by == 0 {
                throws.push((self.next_true_idx, new_item));
            } else {
                throws.push((self.next_false_idx, new_item));
            }
        }
        throws
    }
}

fn make_monkeys(input: aoc::Input) -> Vec<Monkey> {
    let mut monkeys = vec![];

    let mut monkey_lines = vec![];
    for line in input.lines().chain(std::iter::once("")) {
        if line.is_empty() {
            monkeys.push(Monkey::parse(&monkey_lines));
            monkey_lines.clear();
        } else {
            monkey_lines.push(line);
        }
    }
    monkeys
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut monkeys = make_monkeys(input);

    for _ in 0..20 {
        for monkey_idx in 0..monkeys.len() {
            let throws = monkeys[monkey_idx].turn(WorryDecrease::Divide(3));
            for (dest_idx, item) in throws {
                monkeys[dest_idx].items.push_back(item);
            }
        }
    }
    // eprintln!("monkeys = {:#?}", monkeys);
    monkeys
        .iter()
        .map(|m| m.inspected)
        .sorted()
        .rev()
        .take(2)
        .product::<usize>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut monkeys = make_monkeys(input);

    let gcd = monkeys
        .iter()
        .map(|m| m.divide_by as u64)
        .fold(1, |acc, x| acc.gcd(x));

    let lcm = monkeys.iter().map(|m| m.divide_by as u64).product::<u64>() / gcd;

    for _ in 0..10_000 {
        for monkey_idx in 0..monkeys.len() {
            let throws = monkeys[monkey_idx].turn(WorryDecrease::Modulus(lcm as i64));
            for (dest_idx, item) in throws {
                monkeys[dest_idx].items.push_back(item);
            }
        }
    }
    //eprintln!("monkeys = {:#?}", monkeys);
    monkeys
        .iter()
        .map(|m| m.inspected)
        .sorted()
        .rev()
        .take(2)
        .product::<usize>()
}
