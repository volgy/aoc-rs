use std::collections::HashMap;

aoc::parts!(1, 2);

struct Network(HashMap<String, Vec<String>>);

impl Network {
    fn from_input(input: &aoc::Input) -> Self {
        let map: HashMap<_, _> = input
            .lines()
            .map(|l| {
                let (src, dsts) = l.split_once(':').unwrap();
                (
                    src.to_owned(),
                    dsts.split_whitespace().map(str::to_owned).collect(),
                )
            })
            .collect();
        Self(map)
    }

    fn visit(
        &self,
        current: &str,
        required: Vec<String>,
        cache: &mut HashMap<(String, Vec<String>), usize>,
    ) -> usize {
        let key = (current.to_owned(), required.clone());
        if let Some(&count) = cache.get(&key) {
            return count;
        }
        let count = if current == "out" {
            required.is_empty() as usize
        } else {
            self.0.get(current).map_or(0, |dsts| {
                dsts.iter()
                    .map(|d| {
                        let mut new_required = required.clone();
                        if let Some(pos) = new_required.iter().position(|r| r == current) {
                            new_required.remove(pos);
                        }
                        self.visit(d, new_required, cache)
                    })
                    .sum()
            })
        };
        cache.insert(key, count);
        count
    }

    fn n_exit_paths(&self, src: &str, required: &[&str]) -> usize {
        let mut cache = HashMap::new();
        let required_vec = required.iter().map(|s| s.to_string()).collect();
        self.visit(src, required_vec, &mut cache)
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    Network::from_input(&input).n_exit_paths("you", &[])
}

fn part_2(input: aoc::Input) -> impl ToString {
    Network::from_input(&input).n_exit_paths("svr", &["dac", "fft"])
}
