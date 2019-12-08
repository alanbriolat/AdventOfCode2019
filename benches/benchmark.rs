use advent_of_code_2019::*;

use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    macro_rules! run {
        ($l:expr) => {
            c.bench_function(stringify!($l), |b| b.iter(|| $l));
        }
    }
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
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
