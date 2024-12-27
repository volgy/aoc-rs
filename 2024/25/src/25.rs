use itertools::Itertools;
aoc::parts!(1);

#[derive(Debug)]
enum KeyLock {
    Key([usize; Self::N_PINS]),
    Lock([usize; Self::N_PINS]),
}

impl KeyLock {
    const N_PINS: usize = 5;
    const MAX_HEIGHT: usize = 5;

    fn parse(lines: Vec<&str>) -> Self {
        let is_lock = lines[0].chars().all(|c| c == '#');
        let mut code = [0; Self::N_PINS];
        for (i, line) in lines.iter().take(lines.len() - 1).enumerate() {
            for (j, ch) in line.chars().enumerate() {
                if ch == '#' {
                    code[j] = if is_lock {
                        i
                    } else {
                        code[j].max(Self::MAX_HEIGHT - i + 1)
                    };
                }
            }
        }

        if is_lock {
            Self::Lock(code)
        } else {
            Self::Key(code)
        }
    }

    fn fits(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Key(k), Self::Lock(l)) | (Self::Lock(l), Self::Key(k)) => k
                .iter()
                .zip(l)
                .all(|(h1, h2)| (h1 + h2) <= Self::MAX_HEIGHT),
            _ => false,
        }
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let (keys, locks): (Vec<_>, Vec<_>) = input
        .lines()
        .chunks(8)
        .into_iter()
        .map(|sch| KeyLock::parse(sch.collect_vec()))
        .partition(|kl| matches!(kl, KeyLock::Key(_)));

    keys.iter()
        .cartesian_product(locks.iter())
        .filter(|(key, lock)| key.fits(lock))
        .count()
}
