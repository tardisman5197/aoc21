use day_3;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    let start = Instant::now();
    let part_1_result = day_3::part_1(&args[1]);
    let duration = start.elapsed();

    println!("part 1 = {} in {:?}", part_1_result, duration);

    let start = Instant::now();
    let part_2_result = day_3::part_2(&args[1]);
    let duration = start.elapsed();

    println!("part 2 = {} in {:?}", part_2_result, duration);
}
