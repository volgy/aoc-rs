// The changes seem to be significant enough to warrant a copy & paste approach
use super::{Dir, Pos};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Object {
    Wall,
    LeftBox,
    RightBox,
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
                let left_pos = (2 * x, y);
                let right_pos = (2 * x + 1, y);
                match c {
                    '#' => {
                        objects.insert(left_pos, Object::Wall);
                        objects.insert(right_pos, Object::Wall);
                    }
                    'O' => {
                        objects.insert(left_pos, Object::LeftBox);
                        objects.insert(right_pos, Object::RightBox);
                    }
                    '.' => {}
                    '@' => {
                        robot = Some(left_pos);
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
                Object::LeftBox => Some(pos),
                _ => None,
            })
            .map(|(x, y)| x + 100 * y)
            .sum()
    }

    fn make_room(&mut self, pos: Pos, dir: Dir) -> bool {
        // this approach is highly inefficient (for transaction-like behavior), but it works
        let mut temp_objects = self.objects.clone();
        if Self::try_make_room(&mut temp_objects, pos, dir) {
            self.objects = temp_objects;
            true
        } else {
            false
        }
    }

    fn try_make_room(temp_objects: &mut HashMap<Pos, Object>, pos: Pos, dir: Dir) -> bool {
        match temp_objects.get(&pos) {
            Some(Object::Wall) => false,
            Some(boxtype @ (Object::LeftBox | Object::RightBox)) => {
                let (dx, dy) = dir.dxdy();
                let new_pos = (pos.0.wrapping_add_signed(dx), pos.1.wrapping_add_signed(dy));
                if dir.is_horizontal() {
                    if Self::try_make_room(temp_objects, new_pos, dir) {
                        let object = temp_objects.remove(&pos).unwrap();
                        temp_objects.insert(new_pos, object);
                        true
                    } else {
                        false
                    }
                } else {
                    // vertical
                    let peer_pos = match boxtype {
                        Object::LeftBox => (pos.0 + 1, pos.1),
                        Object::RightBox => (pos.0 - 1, pos.1),
                        _ => unreachable!(),
                    };
                    let new_peer_pos = (
                        peer_pos.0.wrapping_add_signed(dx),
                        peer_pos.1.wrapping_add_signed(dy),
                    );
                    if Self::try_make_room(temp_objects, new_pos, dir)
                        && Self::try_make_room(temp_objects, new_peer_pos, dir)
                    {
                        let object = temp_objects.remove(&pos).unwrap();
                        temp_objects.insert(new_pos, object);
                        let peer_object = temp_objects.remove(&peer_pos).unwrap();
                        temp_objects.insert(new_peer_pos, peer_object);
                        true
                    } else {
                        false
                    }
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
            // print!("\x1B[1;1H");
            // println!("{:?}\n{}", dir, self.render());
            // std::thread::sleep(std::time::Duration::from_millis(100));
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
                    Some(Object::LeftBox) => '[',
                    Some(Object::RightBox) => ']',
                    _ => '.',
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
