use itertools::Itertools;

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    let xmas = b"XMAS";
    let puzzle: Vec<_> = input.lines().map(str::as_bytes).collect();
    let (n_rows, n_cols) = (puzzle.len(), puzzle[0].len());

    let mut n_found = 0;

    for i in 0..n_rows {
        for j in 0..n_cols {
            for (di, dj) in (-1isize..=1).cartesian_product(-1isize..=1) {
                if (0..xmas.len()).all(|n| {
                    let (ni, nj) = (i as isize + n as isize * di, j as isize + n as isize * dj);
                    ni >= 0
                        && nj >= 0
                        && (ni as usize) < n_rows
                        && (nj as usize) < n_cols
                        && puzzle[ni as usize][nj as usize] == xmas[n]
                }) {
                    n_found += 1;
                }
            }
        }
    }
    n_found
}

fn part_2(input: aoc::Input) -> impl ToString {
    let puzzle: Vec<_> = input.lines().map(str::as_bytes).collect();
    let (n_rows, n_cols) = (puzzle.len(), puzzle[0].len());

    (1..n_rows - 1)
        .cartesian_product(1..n_cols - 1)
        .filter(|&(i, j)| {
            puzzle[i][j] == b'A'
                && matches!(
                    (
                        puzzle[i - 1][j - 1],
                        puzzle[i - 1][j + 1],
                        puzzle[i + 1][j - 1],
                        puzzle[i + 1][j + 1]
                    ),
                    (b'M', b'M', b'S', b'S')
                        | (b'S', b'M', b'S', b'M')
                        | (b'S', b'S', b'M', b'M')
                        | (b'M', b'S', b'M', b'S')
                )
        })
        .count()
}
