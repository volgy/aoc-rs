aoc::parts!(1);

fn to_snafu(val: i64) -> String {
    if val == 0 {
        return "0".to_owned();
    }

    let mut val = val;
    let mut snafu = String::new();

    while val > 0 {
        let (mut d, m) = (val / 5, val % 5);
        snafu.push(match m {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => {
                d += 1;
                '='
            }
            4 => {
                d += 1;
                '-'
            }
            _ => unreachable!(),
        });
        val = d;
    }
    snafu.chars().rev().collect()
}

fn from_snafu(snafu: &str) -> i64 {
    let mut val = 0;

    for (p, v) in snafu.chars().rev().enumerate() {
        let m = 5_i64.pow(p as u32);
        val += match v {
            '0' => 0,
            '1' => m,
            '2' => 2 * m,
            '=' => -2 * m,
            '-' => -m,
            _ => unreachable!(),
        }
    }

    val
}

fn part_1(input: aoc::Input) -> impl ToString {
    to_snafu(input.lines().map(from_snafu).sum())
}
