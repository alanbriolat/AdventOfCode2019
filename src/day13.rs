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

impl From<Tile> for char {
    fn from(t: Tile) -> Self {
        match t {
            Tile::Empty => ' ',
            Tile::Wall => '#',
            Tile::Block => 'X',
            Tile::Paddle => '=',
            Tile::Ball => 'o',
        }
    }
}

struct Display {
    data: HashMap<Point2D, Tile>,
    top_left: Point2D,
    bottom_right: Point2D,
    paddle: Option<Point2D>,
    ball: Option<Point2D>,
}

impl Display {
    fn new() -> Display {
        Display {
            data: HashMap::new(),
            top_left: point!(0, 0),
            bottom_right: point!(0, 0),
            paddle: None,
            ball: None,
        }
    }

    fn draw(&mut self, x: i32, y: i32, tile: Tile) {
        self.top_left.x = min(self.top_left.x, x);
        self.top_left.y = min(self.top_left.y, y);
        self.bottom_right.x = max(self.bottom_right.x, x);
        self.bottom_right.y = max(self.bottom_right.y, y);
        if tile == Tile::Empty {
            self.data.remove(&point!(x, y));
        } else {
            if tile == Tile::Paddle {
                self.paddle = Some(point!(x, y));
            } else if tile == Tile::Ball {
                self.ball = Some(point!(x, y));
            }
            self.data.insert(point!(x, y), tile);
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in self.top_left.y ..= self.bottom_right.y {
            for x in self.top_left.x ..= self.bottom_right.x {
                print!("{}", match self.data.get(&point!(x, y)).unwrap_or(&Tile::Empty) {
                    Tile::Empty => ' ',
                    Tile::Wall => '#',
                    Tile::Block => 'X',
                    Tile::Paddle => '=',
                    Tile::Ball => 'o',
                });
            }
            println!();
        }
    }
}

struct ArcadeMachine {
    emulator: Emulator,
    display: Display,
    score: Word,
}

impl ArcadeMachine {
    fn from_data_file(filename: &str) -> ArcadeMachine {
        ArcadeMachine {
            emulator: Emulator::from_data_file(filename),
            display: Display::new(),
            score: 0,
        }
    }

    fn insert_coin(&mut self) {
        self.emulator.set(0, 2);
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("Score: {}", self.score);
        self.display.print();
    }

    fn step(&mut self, strategy: fn(&ArcadeMachine) -> Word) -> bool {
        let state = self.emulator.run();
        for chunk in self.emulator.read_all().chunks(3) {
            if (chunk[0], chunk[1]) == (-1, 0) {
                self.score = chunk[2];
            } else {
                self.display.draw(chunk[0] as i32, chunk[1] as i32, From::from(chunk[2]));
            }
        }
        match state {
            State::Continue => {},
            State::ReadWait => {
                self.emulator.write(strategy(&self));
            },
            State::Halt => {
                return false;
            }
        }
        return true;
    }

    fn run(&mut self, strategy: fn(&ArcadeMachine) -> Word) {
        while self.step(strategy) {}
    }
}

pub fn part1() -> usize {
    let mut emulator = Emulator::from_data_file("day13_input.txt");
    let mut screen = Display::new();
    emulator.run();
    for chunk in emulator.read_all().chunks(3) {
        screen.draw(chunk[0] as i32, chunk[1] as i32, From::from(chunk[2]));
    }
    screen.data.values().map(|t| *t == Tile::Block).filter(|x| *x).count()
}

/// Always move the paddle towards the X coordinate of the ball
fn match_ball(arcade: &ArcadeMachine) -> Word {
    if let (Some(ball), Some(paddle)) = (arcade.display.ball, arcade.display.paddle) {
        (ball.x - paddle.x).signum() as Word
    } else {
        0
    }
}

pub fn part2() -> Word {
    let mut arcade = ArcadeMachine::from_data_file("day13_input.txt");
    arcade.insert_coin();
    arcade.run(match_ball);
//    arcade.print();
    arcade.score
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
        assert_eq!(part2(), 15328);
    }
}
