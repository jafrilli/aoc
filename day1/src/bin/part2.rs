use std::time::Instant;

fn main() {
    let input = include_str!("../../input1.txt");

    let start = Instant::now();
    let sum = day1::part2::exec(input);
    let time_elapsed = start.elapsed();
    println!("DAY 1.1 - sum={}, μs={}", sum, time_elapsed.as_micros());
}