use itertools::Itertools;

aoc::parts!(1, 2);

type Pos = (isize, isize);

fn parse_tiles(input: aoc::Input) -> Vec<Pos> {
    input
        .lines()
        .map(|l| {
            l.split(',')
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

fn rect_area(p1: Pos, p2: Pos) -> isize {
    ((p1.0 - p2.0).abs() + 1) * ((p1.1 - p2.1).abs() + 1)
}

fn line_intersects_rect(rect: (Pos, Pos), line: (Pos, Pos)) -> bool {
    let ((rx1, ry1), (rx2, ry2)) = rect;
    let ((lx1, ly1), (lx2, ly2)) = line;

    // inside area (excluding edges)
    let (r_left, r_right) = (rx1.min(rx2) + 1, rx1.max(rx2) - 1);
    let (r_top, r_bottom) = (ry1.min(ry2) + 1, ry1.max(ry2) - 1);

    if r_left > r_right || r_top > r_bottom {
        return false;
    }

    let overlaps = |a: (isize, isize), b: (isize, isize)| a.0.max(b.0) <= a.1.min(b.1);

    match (lx1 == lx2, ly1 == ly2) {
        (true, _) => {
            (r_left..=r_right).contains(&lx1)
                && overlaps((ly1.min(ly2), ly1.max(ly2)), (r_top, r_bottom))
        }
        (_, true) => {
            (r_top..=r_bottom).contains(&ly1)
                && overlaps((lx1.min(lx2), lx1.max(lx2)), (r_left, r_right))
        }
        _ => panic!("only horizontal and vertical lines are supported"),
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let tiles = parse_tiles(input);
    tiles
        .iter()
        .tuple_combinations()
        .map(|(&t1, &t2)| rect_area(t1, t2))
        .max()
        .unwrap()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let tiles = parse_tiles(input);

    tiles
        .iter()
        .tuple_combinations()
        .filter_map(|(&t1, &t2)| {
            tiles
                .iter()
                .circular_tuple_windows()
                .all(|(&l1, &l2)| !line_intersects_rect((t1, t2), (l1, l2)))
                .then_some(rect_area(t1, t2))
        })
        .max()
        .unwrap()
}
