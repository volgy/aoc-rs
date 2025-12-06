use regex::Regex;
use std::collections::HashMap;

aoc::parts!(1, 2);

#[derive(Debug, Clone, Default)]
struct LightBox<'a> {
    order: HashMap<&'a str, usize>,
    focal: HashMap<&'a str, u8>,
    last: usize,
}

impl<'a> LightBox<'a> {
    fn add(&mut self, label: &'a str, focal: u8) {
        if self.focal.insert(label, focal).is_none() {
            self.order.insert(label, self.last);
            self.last += 1;
        }
    }

    fn remove(&mut self, label: &'a str) {
        if let Some(removed_pos) = self.order.remove(label) {
            self.order.iter_mut().for_each(|(_, pos)| {
                if *pos > removed_pos {
                    *pos -= 1;
                }
            });
            self.last -= 1;
            self.focal.remove(label);
        }
    }

    fn power(&self) -> usize {
        self.order
            .iter()
            .map(|(label, pos)| self.focal[label] as usize * (pos + 1))
            .sum::<usize>()
    }
}

fn hash(s: &str) -> u32 {
    s.bytes().fold(0, |acc, b| ((acc + (b as u32)) * 17) & 0xFF)
}

fn part_1(input: aoc::Input) -> impl ToString {
    input.raw().split(',').map(hash).sum::<u32>()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut lightboxes = vec![LightBox::default(); 256];
    let re = Regex::new(r"^([[:alpha:]]+)([-=])(\d?)$").unwrap();
    for step in input.raw().split(',') {
        let (_, [label, op, focal]) = re.captures(step).unwrap().extract();
        let lightbox = &mut lightboxes[hash(label) as usize];
        match op {
            "-" => lightbox.remove(label),
            "=" => lightbox.add(label, focal.parse().unwrap()),
            _ => unreachable!(),
        }
    }
    lightboxes
        .into_iter()
        .enumerate()
        .map(|(i, b)| (i + 1) * b.power())
        .sum::<usize>()
}
