mod warehouse;

aoc::parts!(1);

fn part_1(input: aoc::Input) -> impl ToString {
    let mut warehouse = warehouse::Warehouse::parse(input);
    warehouse.run();
    warehouse.gps_sum()
}

// fn part_2(input: aoc::Input) -> impl ToString {
//     0
// }
