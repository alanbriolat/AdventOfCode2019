use std::time::Instant;

use advent_of_code_2019::*;

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
    run!(day04::part1());
    run!(day04::part2());
    run!(day05::part1());
    run!(day05::part2());
    run!(day06::part1());
    run!(day06::part2());
    run!(day07::part1());
    run!(day07::part2());
    run!(day08::part1());
    run!(day08::part2());
    run!(day09::part1());
    run!(day09::part2());
    run!(day10::part1());
    run!(day10::part2());
    run!(day11::part1());
    run!(day11::part2());
    run!(day12::part1());
    run!(day12::part2());
    run!(day13::part1());
    run!(day13::part2());
    run!(day14::part1());
    run!(day14::part2());
    run!(day15::part1());
    run!(day15::part2());
    run!(day16::part1());
    run!(day16::part2());
}
