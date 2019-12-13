use crate::intcode::*;
use std::collections::HashMap;
use crate::util::Point2D;
use std::cmp::{min, max};

#[derive(Debug,Eq,PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl From<Word> for Tile {
    fn from(v: i64) -> Self {
        match v {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!(format!("unrecognised tile {}", v)),
        }
    }
}

struct Screen {
    data: HashMap<Point2D, Tile>,
    top_left: Point2D,
    bottom_right: Point2D,
}

impl Screen {
    fn new() -> Screen {
        Screen {
            data: HashMap::new(),
            top_left: point!(0, 0),
            bottom_right: point!(0, 0),
        }
    }

    fn draw(&mut self, x: i32, y: i32, tile: Tile) {
        self.top_left.x = min(self.top_left.x, x);
        self.top_left.y = min(self.top_left.y, y);
        self.bottom_right.x = max(self.bottom_right.x, x);
        self.bottom_right.y = max(self.bottom_right.y, y);
        self.data.insert(point!(x, y), tile);
    }
}

pub fn part1() -> usize {
    let mut emulator = Emulator::from_data_file("day13_input.txt");
    let mut screen = Screen::new();
    emulator.run();
    for chunk in emulator.read_all().chunks(3) {
        screen.draw(chunk[0] as i32, chunk[1] as i32, From::from(chunk[2]));
    }
    screen.data.values().map(|t| *t == Tile::Block).filter(|x| *x).count()
}

pub fn part2() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 306);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), unimplemented!());
    }
}
