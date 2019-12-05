use crate::intcode;
use crate::util;
use std::iter::repeat_with;

pub fn part1() -> intcode::Word {
    let programs: Vec<intcode::Program> = util::read_data("day05_input.txt");
    let mut emulator = intcode::Emulator::new(&programs[0]);
    emulator.write(1);
    emulator.run();
    repeat_with(|| emulator.read())
        .take_while(|x| x.is_some())
        .last()
        .unwrap()
        .unwrap()
}

pub fn part2() -> i32 {
    0
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
        unimplemented!();
    }
}
