use crate::intcode;
use crate::util;

pub fn part1() -> intcode::Word {
    let programs: Vec<intcode::Program> = util::read_data("day02_input.txt");
    let mut emulator = intcode::Emulator::new(&programs[0]);
    emulator.set(1, 12);
    emulator.set(2, 2);
    emulator.run();
    emulator.get(0)
}

pub fn part2() -> intcode::Word {
    let programs: Vec<intcode::Program> = util::read_data("day02_input.txt");
    let base = intcode::Emulator::new(&programs[0]);
    let target = 19690720 as intcode::Word;

    'outer: for x in 0..=99 {
        for y in 0..=99 {
            let mut emulator = base.clone();
            emulator.set(1, x);
            emulator.set(2, y);
            emulator.run();
            if emulator.get(0) == target {
                return 100 * x + y;
            }
        }
    }
    panic!("didn't find a solution");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 3562672);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 8250);
    }
}
