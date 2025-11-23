use std::collections::HashMap;

aoc::parts!(1, 2);

#[derive(Debug)]
struct Line {
    start: (u32, u32),
    end: (u32, u32),
}

impl Line {
    fn from_str(line: &str) -> Self {
        fn parse_coord(s: &str) -> (u32, u32) {
            let (x, y) = s.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        }

        let (s_start, s_end) = line.split_once(" -> ").unwrap();

        Self {
            start: parse_coord(s_start),
            end: parse_coord(s_end),
        }
    }
}

#[derive(Debug)]
struct Diagram(HashMap<(u32, u32), u32>);

impl Diagram {
    fn from_lines(lines: &[Line], diagonals: bool) -> Self {
        let mut map = HashMap::new();
        for line in lines {
            let (x1, y1) = line.start;
            let (x2, y2) = line.end;

            if x1 == x2 {
                for y in y1.min(y2)..=y1.max(y2) {
                    *map.entry((x1, y)).or_insert(0) += 1;
                }
            } else if y1 == y2 {
                for x in x1.min(x2)..=x1.max(x2) {
                    *map.entry((x, y1)).or_insert(0) += 1;
                }
            } else if diagonals {
                let steps = x1.abs_diff(x2);
                assert_eq!(steps, y1.abs_diff(y2));

                let dx = if x2 > x1 { 1 } else { -1 };
                let dy = if y2 > y1 { 1 } else { -1 };

                for d in 0..=steps {
                    let x = (x1 as i32 + dx * d as i32) as u32;
                    let y = (y1 as i32 + dy * d as i32) as u32;
                    *map.entry((x, y)).or_insert(0) += 1;
                }
            }
        }
        Self(map)
    }

    fn dangerous_score(&self) -> usize {
        self.0.values().filter(|&&v| v > 1).count()
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let lines: Vec<_> = input.lines().map(Line::from_str).collect();
    Diagram::from_lines(&lines, false).dangerous_score()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let lines: Vec<_> = input.lines().map(Line::from_str).collect();
    Diagram::from_lines(&lines, true).dangerous_score()
}
