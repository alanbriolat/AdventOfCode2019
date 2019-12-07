use crate::intcode;
use crate::util;

pub fn part1() -> intcode::Word {
    let programs: Vec<intcode::Program> = util::read_data("day05_input.txt");
    let mut emulator = intcode::Emulator::new(&programs[0]);
    emulator.write(1);
    emulator.run();
    *emulator.read_all().last().unwrap()
}

pub fn part2() -> i32 {
    let programs: Vec<intcode::Program> = util::read_data("day05_input.txt");
    let mut emulator = intcode::Emulator::new(&programs[0]);
    emulator.write(5);
    emulator.run();
    *emulator.read_all().last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 15508323);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 9006327);
    }
}
