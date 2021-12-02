use day_2;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    let start = Instant::now();
    let part_1_result = match day_2::parse_input(&args[1]) {
        Ok(input) => day_2::part_1(input),
        Err(e) => panic!("{:?}", e),
    };
    let duration = start.elapsed();

    println!("part 1 = {} in {:?}", part_1_result, duration);

    let start = Instant::now();
    let part_2_result = match day_2::parse_input_pt2(&args[1]) {
        Ok(input) => day_2::part_2(input),
        Err(e) => panic!("{:?}", e),
    };
    let duration = start.elapsed();

    println!("part 2 = {} in {:?}", part_2_result, duration);
}
