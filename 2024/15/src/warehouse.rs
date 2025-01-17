use super::{Dir, Pos};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Object {
    Wall,
    Box,
}

#[derive(Debug)]
pub struct Warehouse {
    objects: HashMap<Pos, Object>,
    robot: Pos,
    sequence: Vec<Dir>,
}

impl Warehouse {
    pub fn parse(input: aoc::Input) -> Self {
        let mut lines = input.lines();
        let mut objects = HashMap::new();
        let mut robot = None;
        for (y, line) in lines.by_ref().take_while(|l| !l.is_empty()).enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = (x, y);
                match c {
                    '#' => {
                        objects.insert(pos, Object::Wall);
                    }
                    'O' => {
                        objects.insert(pos, Object::Box);
                    }
                    '.' => {}
                    '@' => {
                        robot = Some(pos);
                    }
                    _ => unreachable!(),
                }
            }
        }

        let sequence = lines
            .collect::<Vec<_>>()
            .concat()
            .chars()
            .map(|c| match c {
                '>' => Dir::Right,
                'v' => Dir::Down,
                '<' => Dir::Left,
                '^' => Dir::Up,
                _ => unreachable!(),
            })
            .collect();
        Self {
            objects,
            robot: robot.unwrap(),
            sequence,
        }
    }

    pub fn gps_sum(&self) -> usize {
        self.objects
            .iter()
            .filter_map(|(&pos, &object)| match object {
                Object::Box => Some(pos),
                _ => None,
            })
            .map(|(x, y)| x + 100 * y)
            .sum()
    }

    fn make_room(&mut self, pos: Pos, dir: Dir) -> bool {
        match self.objects.get(&pos) {
            Some(Object::Wall) => false,
            Some(Object::Box) => {
                let (dx, dy) = dir.dxdy();
                let new_pos = (pos.0.wrapping_add_signed(dx), pos.1.wrapping_add_signed(dy));
                if self.make_room(new_pos, dir) {
                    let object = self.objects.remove(&pos).unwrap();
                    self.objects.insert(new_pos, object);
                    true
                } else {
                    false
                }
            }
            None => true,
        }
    }

    fn move_robot(&mut self, dir: Dir) {
        let (dx, dy) = dir.dxdy();
        let new_pos = (
            self.robot.0.wrapping_add_signed(dx),
            self.robot.1.wrapping_add_signed(dy),
        );

        if self.make_room(new_pos, dir) {
            self.robot = new_pos;
        }
    }

    pub fn run(&mut self) {
        for dir in self.sequence.clone() {
            self.move_robot(dir);
            // debug trace
            //println!("{:?}\n{}", dir, self.render());
        }
    }

    #[allow(dead_code)]
    pub fn render(&self) -> String {
        let (max_x, max_y) = self.objects.keys().fold((0, 0), |(max_x, max_y), &(x, y)| {
            (max_x.max(x), max_y.max(y))
        });
        let mut buf = String::new();
        for y in 0..=max_y {
            for x in 0..=max_x {
                let pos = (x, y);
                let ch = match self.objects.get(&pos) {
                    Some(Object::Wall) => '#',
                    Some(Object::Box) => 'O',
                    None => '.',
                };
                if pos == self.robot {
                    buf.push('@');
                } else {
                    buf.push(ch);
                }
            }
            buf.push('\n');
        }
        buf
    }
}
