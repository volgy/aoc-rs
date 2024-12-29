mod double_warehouse;
mod warehouse;

aoc::parts!(1, 2);

type Pos = (usize, usize);

#[derive(Debug, Clone, Copy)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}

impl Dir {
    fn dxdy(&self) -> (isize, isize) {
        match self {
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Up => (0, -1),
        }
    }

    fn is_horizontal(&self) -> bool {
        match self {
            Self::Right | Self::Left => true,
            Self::Down | Self::Up => false,
        }
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut warehouse = warehouse::Warehouse::parse(input);
    warehouse.run();
    warehouse.gps_sum()
}

fn part_2(input: aoc::Input) -> impl ToString {
    let mut warehouse = double_warehouse::Warehouse::parse(input);
    warehouse.run();
    warehouse.gps_sum()
}
