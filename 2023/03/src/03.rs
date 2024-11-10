aoc::parts!(1, 2);

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    Number {
        value: u32,
        row: usize,
        col_start: usize,
        col_end: usize,
    },
    Symbol {
        value: char,
        row: usize,
        col: usize,
    },
}

#[derive(Debug)]
struct Schematic {
    tokens: Vec<Token>,
    lut: HashMap<(isize, isize), usize>, // (row, col) -> index
}

impl Schematic {
    fn parse(input: &str) -> Self {
        let mut tokens = Vec::new();
        let mut lut = HashMap::new();

        let mut num_str = String::new();

        // Instead of "lines.()" we leave the "\n" in the result
        // for uniform handling of end of numbers.
        for (row, line) in input.split_inclusive('\n').enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch.is_ascii_digit() {
                    num_str.push(ch);
                } else {
                    if !num_str.is_empty() {
                        let value = num_str.parse().unwrap();
                        let col_end = col - 1;
                        let col_start = col - num_str.len();
                        tokens.push(Token::Number {
                            value,
                            row,
                            col_start,
                            col_end,
                        });
                        for j in col_start..=col_end {
                            lut.insert((row as isize, j as isize), tokens.len() - 1);
                        }
                    }
                    num_str.clear();

                    if ch != '.' && ch != '\n' {
                        tokens.push(Token::Symbol {
                            value: ch,
                            row,
                            col,
                        });
                        lut.insert((row as isize, col as isize), tokens.len() - 1);
                    }
                }
            }
        }

        Self { tokens, lut }
    }

    fn find_partnos(&self) -> Vec<&Token> {
        let mut partnos = Vec::new();
        for token in &self.tokens {
            if let &Token::Number {
                row,
                col_start,
                col_end,
                ..
            } = token
            {
                'search: for i in ((row as isize) - 1)..=((row as isize) + 1) {
                    for j in ((col_start as isize) - 1)..=((col_end as isize) + 1) {
                        if let Some(&idx) = self.lut.get(&(i, j)) {
                            if let Token::Symbol { .. } = self.tokens[idx] {
                                partnos.push(token);
                                break 'search;
                            }
                        }
                    }
                }
            }
        }
        partnos
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let schematic = Schematic::parse(input.raw());
    schematic
        .find_partnos()
        .iter()
        .filter_map(|t| match t {
            Token::Number { value, .. } => Some(*value),
            _ => None,
        })
        .sum::<u32>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let schematic = Schematic::parse(input.raw());

    let mut sum = 0;
    let partnos = schematic.find_partnos();
    for token in &schematic.tokens {
        if let &Token::Symbol { value, row, col } = token {
            if value == '*' {
                let mut near_partnos_idxs = HashSet::new();
                for i in ((row as isize) - 1)..=((row as isize) + 1) {
                    for j in ((col as isize) - 1)..=((col as isize) + 1) {
                        if let Some(&idx) = schematic.lut.get(&(i, j)) {
                            if partnos.contains(&&schematic.tokens[idx]) {
                                near_partnos_idxs.insert(idx);
                            }
                        }
                    }
                }
                if near_partnos_idxs.len() == 2 {
                    let mut ratio = 1;
                    for idx in near_partnos_idxs {
                        if let Token::Number { value, .. } = schematic.tokens[idx] {
                            ratio *= value;
                        }
                    }
                    sum += ratio;
                }
            }
        }
    }
    sum
}
