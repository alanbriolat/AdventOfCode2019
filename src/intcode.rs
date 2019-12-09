use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::VecDeque;
use crate::util;

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
    Position(Word),
    Immediate(Word),
}

#[derive(Debug,Eq,PartialEq)]
enum Op {
    Add(Param, Param, Param),
    Mul(Param, Param, Param),
    Read(Param),
    Write(Param),
    JumpIfTrue(Param, Param),
    JumpIfFalse(Param, Param),
    LessThan(Param, Param, Param),
    Equal(Param, Param, Param),
    Halt,
}

impl Op {
    fn size(&self) -> Word {
        use Op::*;
        match self {
            Add(_, _, _) => 4,
            Mul(_, _, _) => 4,
            Read(_) => 2,
            Write(_) => 2,
            JumpIfTrue(_, _) => 3,
            JumpIfFalse(_, _) => 3,
            LessThan(_, _, _) => 4,
            Equal(_, _, _) => 4,
            Halt => 1,
        }
    }
}

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub enum State {
    Continue,
    Halt,
    ReadWait,
}

#[derive(Clone,Debug)]
pub struct Emulator {
    memory: Vec<Word>,
    ip: Word,
    input_buffer: VecDeque<Word>,
    output_buffer: VecDeque<Word>,
}

impl Emulator {
    pub fn new(program: &Program) -> Emulator {
        Emulator {
            memory: program.0.clone(),
            ip: 0,
            input_buffer: VecDeque::new(),
            output_buffer: VecDeque::new(),
        }
    }

    pub fn from_data_file(filename: &str) -> Emulator {
        let programs: Vec<Program> = util::read_data(filename);
        Emulator::new(&programs[0])
    }

    pub fn set(&mut self, pos: Word, v: Word) {
        let pos = pos as usize;
        if pos >= self.memory.len() {
            self.memory.reserve(pos - self.memory.len() + 1);
        }
        self.memory[pos] = v;
    }

    pub fn get(&self, pos: Word) -> Word {
        let pos = pos as usize;
        self.memory.get(pos).cloned().unwrap_or(0)
    }

    /// Write input value to emulator
    pub fn write(&mut self, v: Word) {
        self.input_buffer.push_back(v);
    }

    /// Read output value from emulator
    pub fn read(&mut self) -> Option<Word> {
        self.output_buffer.pop_front()
    }

    /// Read all unread output from emulator
    pub fn read_all(&mut self) -> Vec<Word> {
        self.output_buffer.drain(..).collect()
    }

    fn fetch(&self, pos: Word) -> Op {
        macro_rules! p {
            (0, $offset:literal) => ( Position(self.get(pos + $offset)) );
            (1, $offset:literal) => ( Immediate(self.get(pos + $offset)) );
        }

        macro_rules! op {
            ($t:path, $a:tt, $b:tt, $c:tt) => ( $t(p!($a, 1), p!($b, 2), p!($c, 3)) );
            ($t:path, $a:tt, $b:tt) => ( $t(p!($a, 1), p!($b, 2)) );
            ($t:path, $a:tt) => ( $t(p!($a, 1)) );
            ($t:path) => ( $t );
        }

        use Param::*;
        use Op::*;
        match self.get(pos) {
            00001 => op!(Add, 0, 0, 0),
            00101 => op!(Add, 1, 0, 0),
            01001 => op!(Add, 0, 1, 0),
            01101 => op!(Add, 1, 1, 0),
            00002 => op!(Mul, 0, 0, 0),
            00102 => op!(Mul, 1, 0, 0),
            01002 => op!(Mul, 0, 1, 0),
            01102 => op!(Mul, 1, 1, 0),
            00003 => op!(Read, 0),
            00004 => op!(Write, 0),
            00104 => op!(Write, 1),
            00005 => op!(JumpIfTrue, 0, 0),
            00105 => op!(JumpIfTrue, 1, 0),
            01005 => op!(JumpIfTrue, 0, 1),
            01105 => op!(JumpIfTrue, 1, 1),
            00006 => op!(JumpIfFalse, 0, 0),
            00106 => op!(JumpIfFalse, 1, 0),
            01006 => op!(JumpIfFalse, 0, 1),
            01106 => op!(JumpIfFalse, 1, 1),
            00007 => op!(LessThan, 0, 0, 0),
            00107 => op!(LessThan, 1, 0, 0),
            01007 => op!(LessThan, 0, 1, 0),
            01107 => op!(LessThan, 1, 1, 0),
            00008 => op!(Equal, 0, 0, 0),
            00108 => op!(Equal, 1, 0, 0),
            01008 => op!(Equal, 0, 1, 0),
            01108 => op!(Equal, 1, 1, 0),
            00099 => op!(Halt),
            _ => panic!("unknown opcode"),
        }
    }

    fn value(&self, param: &Param) -> Word {
        use Param::*;
        match param {
            Position(p) => self.get(*p),
            Immediate(v) => *v,
        }
    }

    pub fn step(&mut self) -> State {
        use Op::*;
        use Param::*;
        let op = self.fetch(self.ip);
        match &op {
            Add(a, b, Position(c)) => {
                self.set(*c, self.value(a) + self.value(b));
            },
            Mul(a, b, Position(c)) => {
                self.set(*c, self.value(a) * self.value(b));
            },
            Read(Position(a)) => {
                match self.input_buffer.pop_front() {
                    Some(v) => {
                        self.set(*a, v);
                    },
                    None => {
                        // Don't increment instruction pointer, will re-try on next step()/run()
                        return State::ReadWait
                    },
                }
            },
            Write(a) => {
                self.output_buffer.push_back(self.value(a));
            },
            JumpIfTrue(test, dest) => {
                if self.value(test) != 0 {
                    self.ip = self.value(dest);
                    return State::Continue;     // Don't increment instruction pointer after jump
                }
            },
            JumpIfFalse(test, dest) => {
                if self.value(test) == 0 {
                    self.ip = self.value(dest);
                    return State::Continue;     // Don't increment instruction pointer after jump
                }
            },
            LessThan(a, b, Position(c)) => {
                self.set(*c, if self.value(a) < self.value(b) { 1 } else { 0 });
            },
            Equal(a, b, Position(c)) => {
                self.set(*c, if self.value(a) == self.value(b) { 1 } else { 0 });
            },
            Halt => {
                // Don't increment instruction pointer, will remain in halted state
                return State::Halt
            },
            _ => panic!("unknown op"),
        };
        self.ip += op.size();
        return State::Continue;
    }

    pub fn run(&mut self) -> State {
        loop {
            match self.step() {
                State::Continue => (),
                state => return state,
            }
        }
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
        assert_eq!(e.step(), State::Continue);
        assert_eq!(e.memory, vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
        assert_eq!(e.step(), State::Continue);
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
