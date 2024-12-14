use std::iter::repeat_n;

aoc::parts!(1, 2);

#[derive(Debug)]
struct Disk(Vec<Option<usize>>);

impl Disk {
    fn parse(input: aoc::Input) -> Self {
        Self(
            input
                .raw()
                .chars()
                .map(|ch| ch.to_digit(10).unwrap() as usize)
                .enumerate()
                .flat_map(|(i, count)| {
                    if i % 2 == 0 {
                        repeat_n(Some(i / 2), count)
                    } else {
                        repeat_n(None, count)
                    }
                })
                .collect(),
        )
    }

    fn checksum(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(pos, id)| id.and_then(|id| Some(id * pos)))
            .sum()
    }

    fn defrag_by_blocks(&mut self) -> &Self {
        let mut src = 0;
        let mut dst = self.0.len() - 1;
        loop {
            while self.0[src].is_some() {
                src += 1;
            }
            while self.0[dst].is_none() {
                dst -= 1;
            }
            if src >= dst {
                break;
            }
            self.0.swap(src, dst);
        }
        self
    }

    fn defrag_by_files(&mut self) -> &Self {
        fn find_last_file(region: &[Option<usize>]) -> Option<(usize, usize)> {
            let mut end = region.len();
            while end > 0 && region[end - 1].is_none() {
                end -= 1;
            }
            if end == 0 {
                return None;
            }

            let mut start = end - 1;
            while start > 0 && region[start - 1] == region[start] {
                start -= 1;
            }

            Some((start, end))
        }

        fn find_first_free(region: &[Option<usize>], size: usize) -> Option<usize> {
            let mut start = 0;
            while start < region.len() {
                let mut end = start;
                while end < region.len() && region[end].is_none() {
                    end += 1;
                    if end - start >= size {
                        return Some(start);
                    }
                }
                start = end + 1;
            }
            None
        }

        let mut tail = self.0.len();
        while tail > 0 {
            if let Some((file_start, file_end)) = find_last_file(&self.0[0..tail]) {
                let file_len = file_end - file_start;
                if let Some(free_start) = find_first_free(&self.0, file_len) {
                    if free_start < file_start {
                        for i in 0..file_len {
                            self.0.swap(file_start + i, free_start + i);
                        }
                    }
                }
                tail = file_start;
            } else {
                break;
            }
        }
        self
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    Disk::parse(input).defrag_by_blocks().checksum()
}

fn part_2(input: aoc::Input) -> impl ToString {
    Disk::parse(input).defrag_by_files().checksum()
}
