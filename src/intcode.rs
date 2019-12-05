use std::str::FromStr;
use std::num::ParseIntError;

pub type Word = i32;

pub struct Program(Vec<Word>);

impl FromStr for Program {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code = s.split(",").map(|x| x.parse::<Word>().unwrap()).collect();
        Ok(Program(code))
    }
}

#[derive(Debug,Eq,PartialEq)]
enum Param {
    Immediate(Word),
    Position(usize),
}

impl Param {
    pub fn new(word: Word, mode: u8) -> Param {
        use Param::*;
        match mode {
            0 => Position(word as usize),
            1 => Immediate(word),
            _ => panic!("unknown parameter mode"),
        }
    }
}

#[derive(Debug,Eq,PartialEq)]
enum Op {
    Add(Param, Param, Param),
    Mul(Param, Param, Param),
    Read(Param),
    Write(Param),
    Halt,
}

impl Op {
    fn size(&self) -> usize {
        use Op::*;
        match self {
            Add(_, _, _) => 4,
            Mul(_, _, _) => 4,
            Read(_) => 2,
            Write(_) => 2,
            Halt => 1,
        }
    }
}

#[derive(Clone,Debug)]
pub struct Emulator {
    memory: Vec<Word>,
    input: Vec<Word>,
    input_pointer: usize,
    output: Vec<Word>,
    output_pointer: usize,
    ip: usize,
}

impl Emulator {
    pub fn new(program: &Program) -> Emulator {
        Emulator {
            memory: program.0.clone(),
            input: Vec::new(),
            input_pointer: 0,
            output: Vec::new(),
            output_pointer: 0,
            ip: 0,
        }
    }

    pub fn set(&mut self, pos: usize, v: Word) -> Word {
        let prev = self.memory[pos];
        self.memory[pos] = v;
        return prev
    }

    pub fn get(&mut self, pos: usize) -> Word {
        return self.memory[pos]
    }

    /// Write input value to emulator
    pub fn write(&mut self, v: Word) {
        self.input.push(v);
    }

    /// Read output value from emulator
    pub fn read(&mut self) -> Option<Word> {
        if self.output_pointer >= self.output.len() {
            None
        } else {
            let out = self.output[self.output_pointer];
            self.output_pointer += 1;
            Some(out)
        }
    }

    fn fetch(&self, pos: usize) -> Op {
        use Param::*;
        use Op::*;
        match self.memory[pos] {
            1 => Add(Position(self.memory[pos + 1] as usize),
                     Position(self.memory[pos + 2] as usize),
                     Position(self.memory[pos + 3] as usize)),
            101 => Add(Immediate(self.memory[pos + 1]),
                       Position(self.memory[pos + 2] as usize),
                       Position(self.memory[pos + 3] as usize)),
            1001 => Add(Position(self.memory[pos + 1] as usize),
                        Immediate(self.memory[pos + 2]),
                        Position(self.memory[pos + 3] as usize)),
            1101 => Add(Immediate(self.memory[pos + 1]),
                        Immediate(self.memory[pos + 2]),
                        Position(self.memory[pos + 3] as usize)),
            2 => Mul(Position(self.memory[pos + 1] as usize),
                     Position(self.memory[pos + 2] as usize),
                     Position(self.memory[pos + 3] as usize)),
            102 => Mul(Immediate(self.memory[pos + 1]),
                       Position(self.memory[pos + 2] as usize),
                       Position(self.memory[pos + 3] as usize)),
            1002 => Mul(Position(self.memory[pos + 1] as usize),
                        Immediate(self.memory[pos + 2]),
                        Position(self.memory[pos + 3] as usize)),
            1102 => Mul(Immediate(self.memory[pos + 1]),
                        Immediate(self.memory[pos + 2]),
                        Position(self.memory[pos + 3] as usize)),
            3 => Read(Position(self.memory[pos + 1] as usize)),
            4 => Write(Position(self.memory[pos + 1] as usize)),
            104 => Write(Immediate(self.memory[pos + 1])),
            99 => Halt,
            _ => panic!("unknown opcode"),
        }
    }

    fn value(&self, param: &Param) -> Word {
        use Param::*;
        match param {
            Immediate(v) => *v,
            Position(p) => self.memory[*p],
        }
    }

    pub fn step(&mut self) -> bool {
        use Op::*;
        use Param::*;
        let op = self.fetch(self.ip);
        match &op {
            Add(a, b, Position(c)) => {
                self.memory[*c] = self.value(a) + self.value(b);
            },
            Mul(a, b, Position(c)) => {
                self.memory[*c] = self.value(a) * self.value(b);
            },
            Read(Position(a)) => {
                self.memory[*a] = self.input[self.input_pointer];
                self.input_pointer += 1;
            },
            Write(a) => {
                self.output.push(self.value(a));
            },
            Halt => return false,   // Don't increment instruction pointer
            _ => panic!("unknown op"),
        };
        self.ip += op.size();
        return true;
    }

    pub fn run(&mut self) {
        while self.step() {};
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_match {
        ($a:expr, $b:pat) => {
            match $a { $b => true, _ => false }
        }
    }

    #[test]
    fn test_fetch() {
        use Op::*;
        use Param::*;
        let e = Emulator::new(&Program(vec![
            99,
            1002, 4, 3, 4,
        ]));
        assert_match!(e.fetch(0), Halt);
        assert_match!(e.fetch(1), Add(Position(4), Immediate(3), Position(4)));
    }

    #[test]
    fn test_value() {
        use Param::*;
        let e = Emulator::new(&Program(vec![
            99,
            1002, 4, 3, 4,
        ]));
        assert_eq!(e.value(&Immediate(3)), 3);
        assert_eq!(e.value(&Position(2)), 4);
    }

    #[test]
    fn test_program_day02_1() {
        let mut e = Emulator::new(&"1,9,10,3,2,3,11,0,99,30,40,50".parse::<Program>().unwrap());
        assert!(e.step());
        assert_eq!(e.memory, vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
        assert!(e.step());
        assert_eq!(e.memory, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }

    #[test]
    fn test_program_day02_2() {
        let mut e = Emulator::new(&"1,0,0,0,99".parse::<Program>().unwrap());
        e.run();
        assert_eq!(e.memory, vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_program_day02_3() {
        let mut e = Emulator::new(&"2,3,0,3,99".parse::<Program>().unwrap());
        e.run();
        assert_eq!(e.memory, vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_program_day02_4() {
        let mut e = Emulator::new(&"2,4,4,5,99,0".parse::<Program>().unwrap());
        e.run();
        assert_eq!(e.memory, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_program_day02_5() {
        let mut e = Emulator::new(&"1,1,1,4,99,5,6,0,99".parse::<Program>().unwrap());
        e.run();
        assert_eq!(e.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
