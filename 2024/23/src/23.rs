use itertools::Itertools;
use std::collections::{HashMap, HashSet};

aoc::parts!(1, 2);

struct Graph {
    adjacency: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn parse(input: aoc::Input) -> Self {
        let mut adjacency = HashMap::new();
        input
            .lines()
            .map(|l| l.split_once("-").unwrap())
            .for_each(|(lhs, rhs)| {
                for (node1, node2) in [(lhs, rhs), (rhs, lhs)] {
                    adjacency
                        .entry(node1.to_owned())
                        .or_insert(HashSet::new())
                        .insert(node2.to_owned());
                }
            });
        Self { adjacency }
    }

    fn nodes(&self) -> HashSet<String> {
        self.adjacency.keys().cloned().collect()
    }

    fn triplets(&self) -> HashSet<[&str; 3]> {
        let mut triplets = HashSet::new();

        for (node, neighbors) in self.adjacency.iter() {
            for neighbor in neighbors {
                for neighbor_of_neighbor in self.adjacency[neighbor].iter() {
                    if neighbor_of_neighbor != node
                        && self.adjacency[neighbor_of_neighbor].contains(node)
                    {
                        let mut triplet =
                            vec![node as &str, neighbor as &str, neighbor_of_neighbor as &str];
                        triplet.sort();
                        triplets.insert(triplet.try_into().unwrap());
                    }
                }
            }
        }

        triplets
    }

    /// Bron–Kerbosch
    fn cliques(
        &self,
        current: HashSet<String>,
        mut candidates: HashSet<String>,
        mut excluded: HashSet<String>,
        cliques: &mut Vec<HashSet<String>>,
    ) {
        if candidates.is_empty() && excluded.is_empty() {
            cliques.push(current);
            return;
        }

        for node in candidates.clone().iter() {
            let mut new_current = current.clone();
            new_current.insert(node.clone());

            let adjacents: HashSet<_> = self.adjacency[node].iter().cloned().collect();
            let new_candidates: HashSet<_> = candidates.intersection(&adjacents).cloned().collect();
            let new_excluded: HashSet<_> = excluded.intersection(&adjacents).cloned().collect();

            self.cliques(new_current, new_candidates, new_excluded, cliques);

            candidates.remove(node);
            excluded.insert(node.clone());
        }
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    Graph::parse(input)
        .triplets()
        .into_iter()
        .filter(|triplet| triplet.iter().any(|n| n.starts_with("t")))
        .count()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let graph = Graph::parse(input);
    let mut cliques = Vec::new();
    let nodes = graph.nodes();

    graph.cliques(HashSet::new(), nodes, HashSet::new(), &mut cliques);
    cliques
        .into_iter()
        .max_by_key(|c| c.len())
        .unwrap()
        .iter()
        .sorted()
        .join(",")
}
