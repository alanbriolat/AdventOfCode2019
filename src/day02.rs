use std::str::FromStr;
use std::num::ParseIntError;

use crate::util;

#[derive(Clone,Debug)]
struct Program {
    data: Vec<usize>,
}

impl FromStr for Program {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code = s.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
        Ok(Program{data: code})
    }
}

type State = Program;

#[derive(Clone,Debug)]
struct Machine {
    state: State,
    program_counter: usize,
}

impl Machine {
    fn new(program: &Program) -> Machine {
        Machine {
            state: program.clone(),
            program_counter: 0,
        }
    }

    fn step(self: &mut Self) -> bool {
        match self.state.data[self.program_counter] {
            1 => {
                let (a, b, c) = (self.state.data[self.program_counter + 1], self.state.data[self.program_counter + 2], self.state.data[self.program_counter + 3]);
                self.state.data[c] = self.state.data[a] + self.state.data[b];
                self.program_counter += 4;
                true
            },
            2 => {
                let (a, b, c) = (self.state.data[self.program_counter + 1], self.state.data[self.program_counter + 2], self.state.data[self.program_counter + 3]);
                self.state.data[c] = self.state.data[a] * self.state.data[b];
                self.program_counter += 4;
                true
            },
            99 => false,
            _ => panic!("unhandled opcode"),
        }
    }

    fn run(self: &mut Self) {
        while self.step() {};
    }
}

pub fn part1() -> usize {
    let programs: &mut Vec<Program> = &mut util::read_data("day02_input.txt");
    let mut machine = Machine::new(&programs[0]);
    machine.state.data[1] = 12;
    machine.state.data[2] = 2;
    machine.run();
    machine.state.data[0]
}

pub fn part2() -> usize {
    let programs: &mut Vec<Program> = &mut util::read_data("day02_input.txt");
    let base = Machine::new(&programs[0]);
    let target = 19690720_usize;

    'outer: for x in 0_usize..=99_usize {
        for y in 0_usize..=99_usize {
            let mut machine = base.clone();
            machine.state.data[1] = x;
            machine.state.data[2] = y;
            machine.run();
            if machine.state.data[0] == target {
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
    fn test_machine_step() {
        let mut machine = Machine::new(&"1,9,10,3,2,3,11,0,99,30,40,50".parse::<Program>().unwrap());
        machine.step();
        assert_eq!(machine.state.data, vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
        machine.step();
        assert_eq!(machine.state.data, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn test_machine_run_1() {
        let mut machine = Machine::new(&"1,0,0,0,99".parse::<Program>().unwrap());
        machine.run();
        assert_eq!(machine.state.data, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_machine_run_2() {
        let mut machine = Machine::new(&"2,3,0,3,99".parse::<Program>().unwrap());
        machine.run();
        assert_eq!(machine.state.data, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_machine_run_3() {
        let mut machine = Machine::new(&"2,4,4,5,99,0".parse::<Program>().unwrap());
        machine.run();
        assert_eq!(machine.state.data, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_machine_run_4() {
        let mut machine = Machine::new(&"1,1,1,4,99,5,6,0,99".parse::<Program>().unwrap());
        machine.run();
        assert_eq!(machine.state.data, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 3562672);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 8250);
    }
}
