use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Error};
use crate::intcode::*;
use crate::util::{Point2D, BoundingBox2D};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::Up),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            _ => Err("not a valid direction (^, v, <, or >)"),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        })
    }
}

#[derive(Debug)]
struct Robot {
    position: Point2D,
    direction: Direction,
}

#[derive(Debug)]
struct Map {
    data: Vec<Vec<char>>,
    width: usize,
    height: usize,
    bbox: BoundingBox2D,
    robot: Robot,
}

impl Map {
    fn new(data: String) -> Map {
        let data: Vec<Vec<char>> = data
            // Split on newline
            .lines()
            // Remove empty line(s)
            .filter(|x| x.len() > 0)
            // Turn each line into Vec<char>
            .map(|x| x.chars().collect())
            // Collect into Vec<Vec<char>>
            .collect();
        let height = data.len();
        let width = data[0].len();
        let mut bbox = BoundingBox2D::new(&point!(0, 0));
        bbox.include(&point!((width - 1) as i32, (height - 1) as i32));
        let mut map = Map {
            data,
            width,
            height,
            bbox,
            robot: Robot { position: point!(-1, -1), direction: Direction::Up },
        };
        for p in map.bbox.clone().iter() {
            if let Some(Ok(d)) = map.get(&p).map(|c| Direction::try_from(*c)) {
                map.robot = Robot { position: p, direction: d };
                *map.get_mut(&p).unwrap() = '#';
                break;
            }
        }
        return map;
    }

    #[allow(dead_code)]
    fn print(&self, robot: Option<&Robot>) {
        for p in self.bbox.iter() {
            match robot {
                Some(Robot{position, direction}) if *position == p => {
                    print!("{}", direction);
                },
                _ => {
                    print!{"{}", self.get(&p).unwrap()}
                },
            }
            if p.x == self.bbox.max.x {
                println!();
            }
        }
    }

    fn find_intersections(&self) -> Vec<Point2D> {
        let mut intersections: Vec<Point2D> = Vec::new();
        for p in self.bbox.iter() {
            if let Some('#') = self.get(&p) {
                let up = self.get(&(p +  point!(0, -1)));
                let down = self.get(&(p +  point!(0, 1)));
                let left = self.get(&(p +  point!(-1, 0)));
                let right = self.get(&(p +  point!(1, 0)));
                if let (Some('#'), Some('#'), Some('#'), Some('#')) = (up, down, left, right) {
                    intersections.push(p);
                }
            }
        }
        return intersections;
    }

    fn get(&self, p: &Point2D) -> Option<&char> {
        if self.bbox.contains(p) {
            Some(&self.data[p.y as usize][p.x as usize])
        } else {
            None
        }
    }

    fn get_mut(&mut self, p: &Point2D) -> Option<&mut char> {
        if self.bbox.contains(p) {
            Some(&mut self.data[p.y as usize][p.x as usize])
        } else {
            None
        }
    }
}

pub fn part1() -> i32 {
    let mut emulator = Emulator::from_data_file("day17_input.txt");
    emulator.run();
    let map = Map::new(emulator.read_all().into_iter().map(|c| c as u8 as char).collect());
//    map.print(Some(&map.robot));
//    map.print(None);
    let intersections = map.find_intersections();
    intersections.iter().map(|p| p.x * p.y).sum()
}

pub fn part2() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 4112);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), unimplemented!());
    }
}
