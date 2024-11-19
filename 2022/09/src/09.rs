use std::{collections::HashSet, iter::repeat};

aoc::parts!(1, 2);

#[derive(Debug, Clone, Default)]
struct Knot {
    x: i32,
    y: i32,
    next: Option<Box<Knot>>,
}

impl Knot {
    fn lengthen(&mut self) {
        let mut tail = self;
        while let Some(ref mut next) = tail.next {
            tail = next
        }
        tail.next = Some(Box::new(Knot {
            x: tail.x,
            y: tail.y,
            next: None,
        }));
    }

    fn tail(&self) -> &Self {
        let mut tail = self;
        while let Some(ref next) = tail.next {
            tail = next;
        }
        tail
    }

    fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn pull(&mut self, dx: i32, dy: i32) {
        assert!(dx.abs() < 2 && dy.abs() < 2);

        self.x += dx;
        self.y += dy;

        if let Some(ref mut next) = self.next {
            let pull_x = self.x - next.x;
            let pull_y = self.y - next.y;

            if pull_x.abs() > 1 || pull_y.abs() > 1 {
                next.pull(pull_x.signum(), pull_y.signum());
            }
        }
    }
}

fn simulate(head: &mut Knot, input: aoc::Input) -> usize {
    let mut unique_positons = HashSet::new();
    unique_positons.insert(head.tail().position());
    for dir in input
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .flat_map(|(d, c)| repeat(d).take(c.parse().unwrap()))
    {
        //eprintln!("tail = {:?}, {}", tail, dir);
        match dir {
            "U" => head.pull(0, 1),
            "D" => head.pull(0, -1),
            "R" => head.pull(1, 0),
            "L" => head.pull(-1, 0),
            _ => unreachable!(),
        }
        unique_positons.insert(head.tail().position());
    }

    unique_positons.len()
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut head = Knot::default();
    head.lengthen();
    simulate(&mut head, input)
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut head = Knot::default();
    for _ in 0..9 {
        head.lengthen()
    }
    simulate(&mut head, input)
}
