use std::collections::{HashMap, VecDeque};
use crate::intcode::*;
use crate::util::{Vector2D, Point2D, BoundingBox2D};

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
    distance: usize,
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
            distance: self.distance + 1,
            tile,
        }
    }
}

struct Droid {
    emulator: Emulator,
    map: HashMap<Point2D, State>,
    oxygen: Option<(Point2D, usize)>,
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
        let initial = State{emulator: self.emulator.clone(), position: point!(0, 0), distance: 0, tile: Tile::Floor};
        self.map.insert(point!(0, 0), initial.clone());
        queue.push_back(initial);
        // Queue-based flood fill algorithm
        while let Some(state) = queue.pop_front() {
            for d in directions.iter().cloned() {
                let next_position = state.position + From::from(d);
                // Only process tiles that are empty
                if let Some(prev_state) = self.map.get(&next_position) {
                    if prev_state.tile != Tile::Empty {
                        continue;
                    }
                }
                // Find out what's in this direction
                let next_state = state.step(d);
                // If we found the oxygen system, record its position
                if next_state.tile == Tile::Oxygen {
                    self.oxygen = Some((next_state.position.clone(), next_state.distance));
                }
                // If it's not a wall, continue flood fill from that point
                if next_state.tile != Tile::Wall {
                    queue.push_back(next_state.clone());
                }
                // Record what's at this new position
                self.map.insert(next_state.position, next_state);
            }
        }
    }

    #[allow(dead_code)]
    fn print_map(&self) {
        let mut bbox = BoundingBox2D::new(&point!(0, 0));
        for p in self.map.keys() {
            bbox.include(p);
        }
        for p in bbox.iter() {
            let tile = self.map.get(&p).map(|state| state.tile).unwrap_or(Tile::Empty);
            print!("{}", match tile {
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

pub fn part1() -> usize {
    let mut droid = Droid::from_data_file("day15_input.txt");
    droid.discover_map();
//    droid.print_map();
    droid.oxygen.unwrap().1
}

pub fn part2() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 282);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), unimplemented!());
    }
}
