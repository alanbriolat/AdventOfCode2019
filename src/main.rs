use std::time::Instant;

mod util;
mod day01;

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
}
