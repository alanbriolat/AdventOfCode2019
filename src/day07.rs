use std::cmp::max;
use permutohedron::Heap;
use crate::intcode::{Word, Emulator};

fn run_amps(base: &Emulator) -> Word {
    let mut phases: Vec<Word> = (0 .. 5).collect();
    let heap = Heap::new(&mut phases);
    let mut best: Word = 0;
    for permutation in heap {
        let mut signal: Word = 0;
        for phase in permutation {
            let mut amp = base.clone();
            amp.write(phase);
            amp.write(signal);
            amp.run();
            signal = *amp.read_all().last().unwrap();
        }
        best = max(best, signal);
    }
    return best;
}

pub fn part1() -> i32 {
    run_amps(&Emulator::from_data_file("day07_input.txt"))
}

pub fn part2() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_amps_1() {
        let base = Emulator::from_data_file("day07_example1.txt");
        let result = run_amps(&base);
        assert_eq!(result, 43210);
    }

    #[test]
    fn test_run_amps_2() {
        let base = Emulator::from_data_file("day07_example2.txt");
        let result = run_amps(&base);
        assert_eq!(result, 54321);
    }

    #[test]
    fn test_run_amps_3() {
        let base = Emulator::from_data_file("day07_example3.txt");
        let result = run_amps(&base);
        assert_eq!(result, 65210);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 46248);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), unimplemented!());
    }
}
