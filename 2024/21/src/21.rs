use std::collections::HashMap;
use std::iter::{repeat, repeat_n};

aoc::parts!(1);

type Pos = (usize, usize);

#[derive(Debug, Clone)]
struct KeyPad {
    buttons: HashMap<char, Pos>,
    current: char,
}

impl KeyPad {
    fn new(keys: &[&str]) -> Self {
        let buttons = keys
            .into_iter()
            .enumerate()
            .flat_map(|(y, row)| row.chars().enumerate().map(move |(x, ch)| (ch, (x, y))))
            .collect();
        Self {
            buttons,
            current: 'A',
        }
    }

    fn reset(&mut self) {
        self.current = 'A';
    }

    fn press(&mut self, button: char) -> Vec<String> {
        let src = self.buttons[&self.current];
        let dst = self.buttons[&button];
        let (dx, dy) = (
            dst.0 as isize - src.0 as isize,
            dst.1 as isize - src.1 as isize,
        );

        fn steps(delta: isize, pos: char, neg: char) -> impl Iterator<Item = char> {
            repeat_n(if delta > 0 { pos } else { neg }, delta.unsigned_abs())
        }

        let horizontal: String = steps(dx, '>', '<').collect();
        let vertical: String = steps(dy, 'v', '^').collect();

        let avoid_pos = self.buttons[&' '];

        self.current = button;

        if avoid_pos == (src.0, dst.1) || (dx * dy) == 0 {
            vec![horizontal + &vertical + "A"]
        } else if avoid_pos == (src.1, dst.0) {
            vec![vertical + &horizontal + "A"]
        } else {
            vec![
                horizontal.clone() + &vertical + "A",
                vertical + &horizontal + "A",
            ]
        }
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    const NUM_PAD: [&str; 4] = ["789", "456", "123", " 0A"];
    const DIR_PAD: [&str; 2] = [" ^A", "<v>"];
    let mut keypads = [
        KeyPad::new(&NUM_PAD),
        KeyPad::new(&DIR_PAD),
        KeyPad::new(&DIR_PAD),
    ];

    let mut keypad = KeyPad::new(&NUM_PAD);
    dbg!(keypad.press('0'));
    dbg!(keypad.press('2'));
    dbg!(keypad.press('9'));
    // let mut sum_complexity = 0;
    // for line in input.lines() {
    //     let numeric: usize = line
    //         .chars()
    //         .filter(|c| c.is_ascii_digit())
    //         .collect::<String>()
    //         .parse()
    //         .unwrap();

    //     let mut buttons = line.to_owned();
    //     for keypad in keypads.iter_mut() {
    //         keypad.reset();
    //         buttons = buttons.chars().map(|b| keypad.press(b)[0]).collect();
    //     }

    //     sum_complexity += numeric * buttons.len();
    // }

    //sum_complexity
    0
}
// fn part_2(input: aoc::Input) -> impl ToString {
//     0
// }
