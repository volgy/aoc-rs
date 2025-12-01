aoc::parts!(1);

type Pos = (usize, usize);

struct SeaFloor {
    rows: usize,
    cols: usize,
    cucumbers_east: Vec<Pos>,
    cucumbers_south: Vec<Pos>,
}

impl SeaFloor {
    fn parse(lines: &[&str]) -> Self {
        let rows = lines.len();
        let cols = lines[0].len();
        let mut cucumbers_east = vec![];
        let mut cucumbers_south = vec![];

        for (i, line) in lines.iter().enumerate() {
            for (j, ch) in line.chars().enumerate() {
                let pos = (i, j);
                match ch {
                    '>' => cucumbers_east.push(pos),
                    'v' => cucumbers_south.push(pos),
                    _ => {}
                }
            }
        }
        Self {
            rows,
            cols,
            cucumbers_east,
            cucumbers_south,
        }
    }

    fn occupancy(&self) -> Vec<Vec<bool>> {
        let mut occupied = vec![vec![false; self.cols]; self.rows];
        for cucumber in self
            .cucumbers_east
            .iter()
            .chain(self.cucumbers_south.iter())
        {
            occupied[cucumber.0][cucumber.1] = true;
        }
        occupied
    }

    fn step(&mut self) -> bool {
        let mut change = false;
        let rows = self.rows;
        let cols = self.cols;

        let mut try_move =
            |cucumbers: &mut Vec<Pos>, (di, dj): (usize, usize), occupied: &Vec<Vec<bool>>| {
                for c in cucumbers.iter_mut() {
                    let ni = (c.0 + di) % rows;
                    let nj = (c.1 + dj) % cols;
                    if !occupied[ni][nj] {
                        change = true;
                        *c = (ni, nj);
                    }
                }
            };

        // east-facing cucumbers move first (snapshot before east moves)
        let occupied = self.occupancy();
        try_move(&mut self.cucumbers_east, (0, 1), &occupied);

        // south-facing cucumbers move next (snapshot after east moves)
        let occupied = self.occupancy();
        try_move(&mut self.cucumbers_south, (1, 0), &occupied);

        change
    }
}

fn part_1(input: aoc::Input) -> impl ToString {
    let mut seafloor = SeaFloor::parse(input.as_lines());
    (1..).find(|_| !seafloor.step()).unwrap()
}
