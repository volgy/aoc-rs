use std::collections::{HashMap, HashSet, VecDeque};

aoc::parts!(1, 2);

type Plot = (usize, usize);
type Garden = HashMap<Plot, char>;

fn parse_garden(input: aoc::Input) -> Garden {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, ch)| ((x, y), ch)))
        .collect()
}

fn find_regions(garden: &Garden) -> Vec<HashSet<Plot>> {
    let mut regions = vec![];
    let mut unvisited = garden.clone();
    let deltas = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    while let Some((&start, &plant)) = unvisited.iter().next() {
        let mut region = HashSet::new();
        let mut queue = VecDeque::from([start]);

        while let Some(pos) = queue.pop_front() {
            if unvisited.remove(&pos).is_none() {
                continue;
            }

            region.insert(pos);
            for (dx, dy) in deltas {
                let next = (pos.0.wrapping_add_signed(dx), pos.1.wrapping_add_signed(dy));
                if unvisited.get(&next) == Some(&plant) {
                    queue.push_back(next);
                }
            }
        }
        regions.push(region);
    }
    regions
}

fn perimeter(region: &HashSet<Plot>) -> usize {
    let mut shared_edges = 0;
    for &(x, y) in region {
        if region.contains(&(x + 1, y)) {
            shared_edges += 2;
        }
        if region.contains(&(x, y + 1)) {
            shared_edges += 2;
        }
    }
    4 * region.len() - shared_edges
}

fn sides(region: &HashSet<Plot>) -> usize {
    // insight: number of corners = number of sides
    let candidates: HashSet<_> = region // offset grid for corner points
        .iter()
        .flat_map(|&(x, y)| [(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)])
        .collect();

    candidates
        .into_iter()
        .map(|(x, y)| {
            // convolution (in disguise)
            [(1, (0, 0)), (-1, (-1, 0)), (1, (-1, -1)), (-1, (0, -1))]
                .into_iter()
                .map(|(w, (dx, dy))| {
                    w * region.contains(&(x.wrapping_add_signed(dx), y.wrapping_add_signed(dy)))
                        as isize
                })
                .sum::<isize>()
                .unsigned_abs()
        })
        .sum()
}

fn part_1(input: aoc::Input) -> impl ToString {
    find_regions(&parse_garden(input))
        .iter()
        .map(|region| region.len() * perimeter(region))
        .sum::<usize>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    find_regions(&parse_garden(input))
        .iter()
        .map(|region| region.len() * sides(region))
        .sum::<usize>()
}
