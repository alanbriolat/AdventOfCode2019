use std::cmp::{min, max};
use std::collections::HashMap;
use crate::intcode::{Emulator, Word, State};
use crate::util::{Point2D, Vector2D};

#[allow(dead_code)]
const BLACK: Word = 0;
const WHITE: Word = 1;
const CCW: Word = 0;
const CW: Word = 1;

const UP: u8 = 0;
const RIGHT: u8 = 1;
const DOWN: u8 = 2;
const LEFT: u8 = 3;

struct HullPainter {
    emulator: Emulator,
    position: Point2D,
    direction: u8,
    hull: HashMap<Point2D, Word>,
}

impl HullPainter {
    fn from_data_file(filename: &str) -> HullPainter {
        HullPainter {
            emulator: Emulator::from_data_file(filename),
            position: point!(0, 0),
            direction: UP,
            hull: HashMap::new(),
        }
    }

    fn rotate(&mut self, direction: Word) {
        self.direction = match direction {
            // Equivalent to -1 in modulo 4, but in Rust -1 % 4 == -1, not 3
            CCW => (self.direction + 3) % 4,
            CW => (self.direction + 1) % 4,
            _ => panic!(("unknown rotation", direction)),
        }
    }

    fn travel(&mut self) {
        self.position = self.position + match self.direction {
            UP => vector!(0, -1),
            RIGHT => vector!(1, 0),
            DOWN => vector!(0, 1),
            LEFT => vector!(-1, 0),
            _ => panic!(("unknown direction", self.direction)),
        }
    }

    fn run(&mut self) {
        loop {
            self.emulator.write(self.hull.get(&self.position).cloned().unwrap_or(0));
            let state = self.emulator.run();
            let output = self.emulator.read_all();
            if output.len() == 2 {
                self.hull.insert(self.position, output[0]);
                self.rotate(output[1]);
                self.travel();
            } else {
                panic!("expected emulator output");
            }
            if state == State::Halt {
                break;
            }
        }
    }

    fn count_painted(&self) -> usize {
        self.hull.len()
    }

    fn snapshot(&self) -> Vec<String> {
        // Get the bounding box
        let mut top_left = point!(0, 0);
        let mut bottom_right = point!(0, 0);
        for p in self.hull.keys() {
            top_left.x = min(top_left.x, p.x);
            top_left.y = min(top_left.y, p.y);
            bottom_right.x = max(bottom_right.x, p.x);
            bottom_right.y = max(bottom_right.y, p.y);
        }
        // Iterate over the bounding box to build up strings from the hull paint
        (top_left.y ..= bottom_right.y)
            .map(|y| {
                (top_left.x ..= bottom_right.x)
                    .map(|x| {
                        if let Some(&WHITE) = self.hull.get(&point!(x, y)) {
                            'X'
                        } else {
                            ' '
                        }
                    }).collect()
            }).collect()
    }
}

pub fn part1() -> usize {
    let mut robot = HullPainter::from_data_file("day11_input.txt");
    robot.run();
    robot.count_painted()
}

pub fn part2() -> String {
    let mut robot = HullPainter::from_data_file("day11_input.txt");
    robot.hull.insert(point!(0, 0), WHITE);
    robot.run();
    format!("\n{}\n", robot.snapshot().join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 2539);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), format!("\n{}\n", vec![
            " XXXX X    XXXX XXX  X  X   XX XXX   XX    ",
            "    X X    X    X  X X X     X X  X X  X   ",
            "   X  X    XXX  XXX  XX      X X  X X  X   ",
            "  X   X    X    X  X X X     X XXX  XXXX   ",
            " X    X    X    X  X X X  X  X X X  X  X   ",
            " XXXX XXXX XXXX XXX  X  X  XX  X  X X  X   ",
        ].join("\n")));
    }
}
