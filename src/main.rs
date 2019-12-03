use std::time::Instant;

#[macro_use] extern crate itertools;

mod util;
mod day01;
mod day02;
mod day03;

macro_rules! run {
    ($l:expr) => {
        let start = Instant::now();
        let result = $l;
        let elapsed = Instant::now().duration_since(start);
        println!("{}: {} ({:?})", stringify!($l), result, elapsed);
    }
}

fn main() {
    run!(day01::part1());
    run!(day01::part2());
    run!(day02::part1());
    run!(day02::part2());
    run!(day03::part1());
    run!(day03::part2());
}
