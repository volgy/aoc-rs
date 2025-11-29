use core::str;

use regex::Regex;

aoc::parts!(1, 2);

struct TargetArea {
    x_range: (i32, i32),
    y_range: (i32, i32),
}

impl TargetArea {
    fn parse(input: &str) -> Self {
        let caps = Regex::new(r"x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)")
            .unwrap()
            .captures(input)
            .unwrap();
        let x_range: (i32, i32) = (caps[1].parse().unwrap(), caps[2].parse().unwrap());
        let y_range: (i32, i32) = (caps[3].parse().unwrap(), caps[4].parse().unwrap());
        TargetArea { x_range, y_range }
    }

    fn contains(&self, x: i32, y: i32) -> bool {
        (self.x_range.0..=self.x_range.1).contains(&x)
            && (self.y_range.0..=self.y_range.1).contains(&y)
    }

    fn missed(&self, x: i32, y: i32) -> bool {
        x > self.x_range.1 || y < self.y_range.0
    }
}

#[derive(Debug)]
struct Shot(i32, i32);

impl Shot {
    fn valid_shots(target: &TargetArea) -> Vec<Shot> {
        let mut valids = vec![];

        let vx_min = 1; // naive
        let vx_max = target.x_range.1;
        let vy_min = target.y_range.0;
        let vy_max = -target.y_range.0; // assuming downward trajectory

        for vx in vx_min..=vx_max {
            for vy in vy_min..=vy_max {
                let (mut x, mut y) = (0, 0);
                let (mut cvx, mut cvy) = (vx, vy);
                for _ in 1.. {
                    x += cvx;
                    y += cvy;
                    cvx = (cvx - 1).max(0);
                    cvy -= 1;
                    if target.contains(x, y) {
                        valids.push(Shot(vx, vy));
                        break;
                    }
                    if target.missed(x, y) {
                        break;
                    }

                    if cvx == 0 && x < target.x_range.0 {
                        break;
                    }
                }
            }
        }

        valids
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let target = TargetArea::parse(input.as_lines()[0]);
    let valids = Shot::valid_shots(&target);
    let vy_max = valids.into_iter().map(|Shot(_x, y)| y).max().unwrap();
    (vy_max + 1) * vy_max / 2
}

fn part_2(input: aoc::Input) -> impl ToString {
    let target = TargetArea::parse(input.as_lines()[0]);
    let valids = Shot::valid_shots(&target);
    println!("{:?}", valids);
    valids.len()
}
