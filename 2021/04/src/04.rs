use aoc::IterUnwrap;
use itertools::Itertools;
use std::collections::HashSet;

aoc::parts!(1, 2);

const N: usize = 5;

#[derive(Debug)]
struct Board {
    grid: [[u32; N]; N],
    marked: [[bool; N]; N],
}

impl Board {
    fn from_lines(lines: &[&str]) -> Self {
        let grid = lines
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|token| token.parse().unwrap())
                    .collect_n()
            })
            .collect_n();

        let marked = [[false; N]; N];
        Self { grid, marked }
    }

    fn mark(&mut self, number: u32) {
        for (i, j) in (0..N).cartesian_product(0..N) {
            if self.grid[i][j] == number {
                self.marked[i][j] = true;
            }
        }
    }

    fn bingo(&self) -> bool {
        (0..N).any(|i| (0..N).all(|j| self.marked[i][j]))
            || (0..N).any(|j| (0..N).all(|i| self.marked[i][j]))
    }

    fn sum_unmarked(&self) -> u32 {
        (0..N)
            .cartesian_product(0..N)
            .filter(|&(i, j)| !self.marked[i][j])
            .map(|(i, j)| self.grid[i][j])
            .sum()
    }
}

fn setup(input: aoc::Input) -> (Vec<u32>, Vec<Board>) {
    let mut lines = input.lines();
    let draws = lines
        .next_uw()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect_vec();

    let boards = lines
        .skip(1)
        .chunks(N + 1)
        .into_iter()
        .map(|chunk| Board::from_lines(&chunk.take(N).collect_vec()))
        .collect_vec();

    (draws, boards)
}

fn part_1(input: aoc::Input) -> impl ToString {
    let (draws, mut boards) = setup(input);
    draws
        .into_iter()
        .find_map(|number| {
            boards.iter_mut().find_map(|board| {
                board.mark(number);
                board.bingo().then(|| board.sum_unmarked() * number)
            })
        })
        .unwrap()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let (draws, mut boards) = setup(input);
    let mut remaining: HashSet<usize> = (0..boards.len()).collect();

    draws
        .into_iter()
        .find_map(|number| {
            for (idx, board) in boards.iter_mut().enumerate() {
                if !remaining.contains(&idx) {
                    continue;
                }
                board.mark(number);
                if board.bingo() {
                    remaining.remove(&idx);
                    if remaining.is_empty() {
                        return Some(board.sum_unmarked() * number);
                    }
                }
            }
            None
        })
        .unwrap()
}
