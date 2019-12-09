use crate::intcode::*;

pub fn part1() -> Word {
    let mut emulator = Emulator::from_data_file("day09_input.txt");
    emulator.write(1);
    emulator.run();
    let output = emulator.read_all().to_vec();
    assert_eq!(output.len(), 1);
    output[0]
}

pub fn part2() -> Word {
    let mut emulator = Emulator::from_data_file("day09_input.txt");
    emulator.write(2);
    emulator.run();
    let output = emulator.read_all().to_vec();
    assert_eq!(output.len(), 1);
    output[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let program: Program = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99".parse().unwrap();
        let mut emulator = Emulator::new(&program);
        emulator.run();
        assert_eq!(emulator.read_all().to_vec(), vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);
    }

    #[test]
    fn test_example_2() {
        let program: Program = "1102,34915192,34915192,7,4,7,99,0".parse().unwrap();
        let mut emulator = Emulator::new(&program);
        emulator.run();
        assert_eq!(emulator.read().unwrap(), 1219070632396864);
    }

    #[test]
    fn test_example_3() {
        let program: Program = "104,1125899906842624,99".parse().unwrap();
        let mut emulator = Emulator::new(&program);
        emulator.run();
        assert_eq!(emulator.read().unwrap(), 1125899906842624);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 3989758265);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 76791);
    }
}
