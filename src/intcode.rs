use std::str::FromStr;
use std::num::ParseIntError;
use std::collections::VecDeque;
use crate::util;

pub type Word = i64;

pub struct Program(Vec<Word>);

impl FromStr for Program {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let code = s.split(",").map(|x| x.parse::<Word>().unwrap()).collect();
        Ok(Program(code))
    }
}

const MODE_POSITION: Word = 0;
const MODE_IMMEDIATE: Word = 1;
const MODE_RELATIVE: Word = 2;

#[derive(Debug,Eq,PartialEq)]
enum Param {
    Position(Word),
    Immediate(Word),
    Relative(Word),
}

impl Param {
    fn new(mode: Word, value: Word) -> Param {
        use Param::*;
        match mode {
            MODE_POSITION => Position(value),
            MODE_IMMEDIATE => Immediate(value),
            MODE_RELATIVE => Relative(value),
            _ => panic!(("unrecognised mode", mode)),
        }
    }
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
    AdjustBase(Param),
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
            AdjustBase(_) => 2,
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
    sp: Word,
    input_buffer: VecDeque<Word>,
    output_buffer: VecDeque<Word>,
}

impl Emulator {
    pub fn new(program: &Program) -> Emulator {
        Emulator {
            memory: program.0.clone(),
            ip: 0,
            sp: 0,
            input_buffer: VecDeque::new(),
            output_buffer: VecDeque::new(),
        }
    }

    pub fn from_data_file(filename: &str) -> Emulator {
        let programs: Vec<Program> = util::read_data(filename);
        Emulator::new(&programs[0])
    }

    fn make_pointer(&mut self, pos: usize) -> &mut Word {
        if pos >= self.memory.len() {
            self.memory.resize(pos + 1, 0);
        }
        &mut self.memory[pos]
    }

    pub fn len(&self) -> usize { self.memory.len() }

    pub fn resize(&mut self, new_len: usize) { self.memory.resize(new_len, 0) }

    pub fn set(&mut self, pos: Word, v: Word) {
        *self.make_pointer(pos as usize) = v;
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
        let op = self.get(pos);
        let (modes, opcode) = (op / 100, op % 100);

        // Get the mode for 1-indexed parameter
        macro_rules! mode {
            ($i:literal) => ( modes / ((10 as Word).pow($i - 1)) % 10 );
        }

        // Get 1-indexed parameter
        macro_rules! p {
            ($i:literal) => ( Param::new(mode!($i), self.get(pos + $i)) );
        }

        // Get an Op with specified arity
        macro_rules! op {
            ($t:path, 0) => ( $t );
            ($t:path, 1) => ( $t(p!(1)) );
            ($t:path, 2) => ( $t(p!(1), p!(2)) );
            ($t:path, 3) => ( $t(p!(1), p!(2), p!(3)) );
        }

        use Op::*;

        match opcode {
            1 => op!(Add, 3),
            2 => op!(Mul, 3),
            3 => op!(Read, 1),
            4 => op!(Write, 1),
            5 => op!(JumpIfTrue, 2),
            6 => op!(JumpIfFalse, 2),
            7 => op!(LessThan, 3),
            8 => op!(Equal, 3),
            9 => op!(AdjustBase, 1),
            99 => Halt,
            _ => panic!(("unknown opcode", opcode)),
        }
    }

    fn value(&self, param: &Param) -> Word {
        use Param::*;
        match param {
            Position(p) => self.get(*p),
            Immediate(v) => *v,
            Relative(r) => self.get(self.sp + *r),
        }
    }

    fn pointer(&mut self, param: &Param) -> &mut Word {
        use Param::*;
        match param {
            Position(p) => self.make_pointer(*p as usize),
            Relative(r) => self.make_pointer((self.sp + *r) as usize),
            _ => panic!("invalid parameter for pointer"),
        }
    }

    pub fn step(&mut self) -> State {
        use Op::*;
        let op = self.fetch(self.ip);
        match &op {
            Add(a, b, c) => {
                *self.pointer(c) = self.value(a) + self.value(b);
            },
            Mul(a, b, c) => {
                *self.pointer(c) = self.value(a) * self.value(b);
            },
            Read(a) => {
                match self.input_buffer.pop_front() {
                    Some(v) => {
                        *self.pointer(a) = v;
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
            LessThan(a, b, c) => {
                *self.pointer(c) = if self.value(a) < self.value(b) { 1 } else { 0 };
            },
            Equal(a, b, c) => {
                *self.pointer(c) = if self.value(a) == self.value(b) { 1 } else { 0 };
            },
            AdjustBase(a) => {
                self.sp += self.value(a);
            },
            Halt => {
                // Don't increment instruction pointer, will remain in halted state
                return State::Halt
            },
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
