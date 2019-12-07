use std::cmp::max;
use permutohedron::Heap;
use crate::intcode::{Program, Word, Emulator};
use crate::util;

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
    let programs: Vec<Program> = util::read_data("day07_input.txt");
    let base = Emulator::new(&programs[0]);
    run_amps(&base)
}

pub fn part2() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_amps_1() {
        let program = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0".parse::<Program>().unwrap();
        let base = Emulator::new(&program);
        let result = run_amps(&base);
        assert_eq!(result, 43210);
    }

    #[test]
    fn test_run_amps_2() {
        let program = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0".parse::<Program>().unwrap();
        let base = Emulator::new(&program);
        let result = run_amps(&base);
        assert_eq!(result, 54321);
    }

    #[test]
    fn test_run_amps_3() {
        let program = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0".parse::<Program>().unwrap();
        let base = Emulator::new(&program);
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
