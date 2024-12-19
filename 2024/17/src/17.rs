use std::collections::HashSet;

use aoc::Parse;

aoc::parts!(1, 2);

#[derive(Debug, Clone)]
struct Computer {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,

    ip: usize,
    mem: Vec<u64>,
}

impl Computer {
    fn parse(input: aoc::Input) -> Self {
        let mut lines = input.lines();
        let parse_register =
            |line: &str, prefix: &str| -> u64 { line.as_parser().after(prefix).parse().unwrap() };

        let reg_a = parse_register(lines.next().unwrap(), "A: ");
        let reg_b = parse_register(lines.next().unwrap(), "B: ");
        let reg_c = parse_register(lines.next().unwrap(), "C: ");
        assert!(lines.next().unwrap().trim().is_empty());

        let mem = lines
            .next()
            .unwrap()
            .as_parser()
            .after("Program: ")
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();

        Self {
            reg_a,
            reg_b,
            reg_c,
            ip: 0,
            mem,
        }
    }

    fn fetch(&mut self) -> Option<(u64, u64)> {
        if self.ip + 1 < self.mem.len() {
            let pair = (self.mem[self.ip], self.mem[self.ip + 1]);
            self.ip += 2;
            Some(pair)
        } else {
            None
        }
    }

    fn decode_combo(&self, operand: u64) -> u64 {
        match operand {
            0..=3 => operand,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => unreachable!(),
        }
    }

    fn run(&mut self) -> Vec<u64> {
        let mut output = vec![];
        while let Some((inst, operand)) = self.fetch() {
            match inst {
                0 => {
                    // adv
                    self.reg_a >>= self.decode_combo(operand);
                }
                1 => {
                    // bxl
                    self.reg_b ^= operand;
                }
                2 => {
                    // bst
                    self.reg_b = self.decode_combo(operand) & 0x7;
                }
                3 => {
                    // jnz
                    if self.reg_a != 0 {
                        self.ip = operand as usize;
                    }
                }
                4 => {
                    // bxc
                    self.reg_b ^= self.reg_c;
                }
                5 => {
                    // out
                    output.push(self.decode_combo(operand) & 0x7);
                }
                6 => {
                    // bdv
                    self.reg_b = self.reg_a >> self.decode_combo(operand);
                }
                7 => {
                    // cdv
                    self.reg_c = self.reg_a >> self.decode_combo(operand);
                }
                _ => unreachable!(),
            }
        }
        output
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut computer = Computer::parse(input);
    computer
        .run()
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn part_2(input: aoc::Input) -> impl ToString {
    // I do not like this - point solution for my actual test input
    //
    // Program: 2,4, 1,5, 7,5, 1,6, 4,2, 5,5, 0,3, 3,0
    // B = A & 0x7
    // B ^= 5
    // C = A >> B
    // B ^= 6
    // B ^= C
    // OUT B & 0x7
    // A >>= 3
    // IF A != 0 RESET
    //
    // Collapsed Program:
    // OUT  ((((A & 0x7) ^ 0x5) ^ 0x6) ^ (A >> ((A & 0x7) ^ 0x5))) 0x7
    // A >>=3 (until not 0)
    //
    // Simplified Program:
    // (((A & 0x7) ^ 0x3) ^ (A >> ((A & 0x7) ^ 0x5))) & 0x7
    // A >>=3 (until not 0)
    //
    // Conclusions:
    // - the program will print out as many values as many 3-bit non-zero bit groups in initial A
    // - for a single output only the lower 6 bits are relevant in A (might be not important)
    // - we should work backwards

    let computer = Computer::parse(input);
    let target_out = computer.mem.clone();
    let mut candidates: HashSet<u64> = HashSet::new();
    candidates.insert(0);

    for target_digit in target_out.into_iter().rev() {
        let mut new_candidates = HashSet::new();
        for candidate in &candidates {
            let base_candidate = candidate << 3;
            for octal in 0..8 {
                let reg_a = base_candidate + octal;
                let mut computer = computer.clone();
                computer.reg_a = reg_a;
                if computer.run()[0] == target_digit {
                    new_candidates.insert(reg_a);
                }
            }
        }
        candidates = new_candidates;
    }
    candidates.into_iter().min().unwrap()
}
