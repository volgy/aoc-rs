use std::collections::HashMap;

aoc::parts!(1, 2);

#[derive(Debug)]
struct Map(HashMap<String, Vec<String>>);

impl Map {
    fn from_lines<'a>(lines: impl Iterator<Item = &'a str>) -> Self {
        let mut adj: HashMap<String, Vec<String>> = HashMap::new();
        for line in lines {
            let (node1, node2) = line.split_once('-').unwrap();
            adj.entry(node1.into()).or_default().push(node2.into());
            adj.entry(node2.into()).or_default().push(node1.into());
        }
        Self(adj)
    }

    fn n_paths<'a>(&self, node: &'a str, mut visited: HashMap<&'a str, i8>, part2: bool) -> usize {
        if node == "end" {
            return 1;
        }
        let is_small = node.chars().all(|c| c.is_lowercase());
        let has_double = visited.values().any(|&v| v >= 2);
        let v_entry = visited.entry(node).or_default();

        if is_small {
            if *v_entry > 0 && (!part2 || node == "start" || has_double) {
                return 0;
            }
            *v_entry += 1;
        }

        self.0
            .get(node)
            .map(|neighbors| {
                neighbors
                    .iter()
                    .map(|next| self.n_paths(next, visited.clone(), part2))
                    .sum()
            })
            .unwrap_or(0)
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let map = Map::from_lines(input.lines());
    map.n_paths("start", HashMap::new(), false)
}

fn part_2(input: aoc::Input) -> impl ToString {
    let map = Map::from_lines(input.lines());
    map.n_paths("start", HashMap::new(), true)
}
