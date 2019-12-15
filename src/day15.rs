use crate::intcode::*;
use crate::util::{Vector2D, Point2D, BoundingBox2D};
use std::collections::{HashMap, VecDeque};
use std::cmp::min;

#[derive(Copy,Clone,Debug)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl From<Direction> for Word {
    fn from(d: Direction) -> Self {
        d as Word
    }
}

impl From<Direction> for Vector2D {
    fn from(d: Direction) -> Self {
        match d {
            Direction::North => vector!(0, -1),
            Direction::South => vector!(0, 1),
            Direction::West => vector!(-1, 0),
            Direction::East => vector!(1, 0),
        }
    }
}

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
enum Tile {
    Wall = 0,
    Floor = 1,
    Oxygen = 2,
    Empty = 255,
}

impl From<Word> for Tile {
    fn from(w: Word) -> Self {
        match w {
            0 => Tile::Wall,
            1 => Tile::Floor,
            2 => Tile::Oxygen,
            _ => panic!("unknown tile"),
        }
    }
}

#[derive(Clone,Debug)]
struct State {
    emulator: Emulator,
    position: Point2D,
    tile: Tile,
}

impl State {
    fn step(&self, d: Direction) -> State {
        let mut e = self.emulator.clone();
        e.write(From::from(d));
        e.run();
        let position = self.position + From::from(d);
        let tile: Tile = From::from(e.read().unwrap());
        return State {
            emulator: e,
            position,
            tile,
        }
    }
}

struct Droid {
    emulator: Emulator,
    map: HashMap<Point2D, Tile>,
    oxygen: Option<Point2D>,
}

impl Droid {
    fn from_data_file(filename: &str) -> Droid {
        Droid {
            emulator: Emulator::from_data_file(filename),
            map: HashMap::new(),
            oxygen: None,
        }
    }

    /// Use flood fill to discover the reachable contents of the map
    fn discover_map(&mut self) {
        let directions = [Direction::North, Direction::South, Direction::West, Direction::East];
        let mut queue: VecDeque<State> = VecDeque::new();
        // Record the starting position as floor
        self.map.insert(point!(0, 0), Tile::Floor);
        queue.push_back(State{emulator: self.emulator.clone(), position: point!(0, 0), tile: Tile::Floor});
        // Queue-based flood fill algorithm
        while let Some(state) = queue.pop_front() {
            for d in directions.iter().cloned() {
                let position = state.position + From::from(d);
                // Only try and fill empty tiles
                if self.map.get(&position).unwrap_or(&Tile::Empty) == &Tile::Empty {
                    let next_state = state.step(d);
                    self.map.insert(next_state.position, next_state.tile);
                    // If we found the oxygen system, record its position
                    if next_state.tile == Tile::Oxygen {
                        self.oxygen = Some(next_state.position.clone());
                    }
                    // Stop when walls are found
                    if next_state.tile != Tile::Wall {
                        queue.push_back(next_state);
                    }
                }
            }
        }
    }

    fn print_map(&self) {
        let mut top_left = point!(0, 0);
        let mut bottom_right = point!(0, 0);
        let mut bbox = BoundingBox2D::new(&point!(0, 0));
        for p in self.map.keys() {
            bbox.include(p);
        }
        for p in bbox.iter() {
            print!("{}", match self.map.get(&p).unwrap_or(&Tile::Empty) {
                Tile::Empty => ' ',
                Tile::Wall => '#',
                Tile::Floor => '.',
                Tile::Oxygen => 'O',
            });
            if p.x == bbox.max.x {
                println!();
            }
        }
        println!("Oxygen system @ {:?}", self.oxygen);
    }
}

pub fn part1() -> i32 {
    let mut droid = Droid::from_data_file("day15_input.txt");
    droid.discover_map();
    droid.print_map();
    0
}

pub fn part2() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), unimplemented!());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), unimplemented!());
    }
}
