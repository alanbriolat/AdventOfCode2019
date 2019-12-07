use crate::intcode;

pub fn part1() -> intcode::Word {
    let mut emulator = intcode::Emulator::from_data_file("day05_input.txt");
    emulator.write(1);
    emulator.run();
    *emulator.read_all().last().unwrap()
}

pub fn part2() -> i32 {
    let mut emulator = intcode::Emulator::from_data_file("day05_input.txt");
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
