use std::collections::{BTreeMap, BTreeSet};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter, Error};
use std::iter::repeat_with;
use std::ops::{Deref, DerefMut};
use crate::intcode::*;
use crate::util::{Point2D, BoundingBox2D, Vector2D};

// Maximum number of robot subroutines
const MAX_ROUTINES: usize = 3;
// Maximum string length for each robot subroutine (excluding newline)
const MAX_ROUTINE_LENGTH: usize = 20;

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn rotate_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn to_vector(&self) -> Vector2D {
        match self {
            Direction::Up => vector!(0, -1),
            Direction::Right => vector!(1, 0),
            Direction::Down => vector!(0, 1),
            Direction::Left => vector!(-1, 0),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::Up),
            '>' => Ok(Direction::Right),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
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

#[derive(Copy,Clone,Debug,Eq,PartialEq)]
enum Command {
    Left,
    Right,
    Forward(i32),
}

impl Command {
    fn function_size(&self) -> usize {
        match self {
            Command::Left | Command::Right => 1,
            Command::Forward(n) => 1 + (*n as f32).log10().floor() as usize,
        }
    }

    fn to_string(&self) -> String {
        match self {
            Command::Left => "L".into(),
            Command::Right => "R".into(),
            Command::Forward(n) => format!("{}", n),
        }
    }
}

type SequenceDef = (usize, usize);
type DuplicateSequenceIndex = BTreeMap<SequenceDef, Vec<usize>>;
type AvailableSequenceIndex = BTreeMap<usize, Vec<SequenceDef>>;

type CompressedSequence = (Vec<char>, BTreeMap<char, SequenceDef>);

#[derive(Debug)]
struct Path(Vec<Command>);
deref!(Path, Vec<Command>);

impl From<&[Command]> for Path {
    fn from(commands: &[Command]) -> Self {
        Path(commands.to_vec())
    }
}

impl Path {
    fn new() -> Path {
        Path(Vec::new())
    }

    fn function_size(&self) -> usize {
        if self.len() == 0 {
            0
        } else {
            self.iter().map(Command::function_size).sum::<usize>() + self.len() - 1
        }
    }

    fn to_string(&self) -> String {
        self.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(",")
    }

    fn simplify(&self) -> Path {
        let mut new: Path = Path::new();
        let mut count = 0;
        for c in self.iter() {
            if let Command::Forward(n) = c {
                count += *n;
            } else {
                if count > 0 {
                    new.push(Command::Forward(count));
                }
                count = 0;
                new.push(*c);
            }
        }
        if count > 0 {
            new.push(Command::Forward(count));
        }
        return new;
    }

    fn find_duplicate_sequences(&self, min_commands: usize, max_length: usize) -> (DuplicateSequenceIndex, AvailableSequenceIndex) {
        assert!(min_commands > 0);
        // Mapping of (start, length) of a sequence to position of each repetition of that sequence
        let mut duplicates = DuplicateSequenceIndex::new();
        // Make sure each (start, length) sequence is recorded only against its first occurrence;
        // i.e. for a1,a2,a3, record all against a1, don't record a3 against a2
        let mut seen: BTreeSet<SequenceDef> = BTreeSet::new();
        // For each starting point that would generate at least one sequence of at least min_length
        for i in 0 ..= self.len() - min_commands {
            // For each potential repetition start
            for j in i + min_commands..= self.len() - min_commands {
                let mut sequence = Path::new();
                // For each pair of potentially matching elements
                for (offset, (a, b)) in self[i ..].iter().zip(self[j ..].iter()).enumerate() {
                    // Stop if sequence is no longer matching
                    if a != b {
                        break;
                    }
                    sequence.push(*a);
                    // Stop if the new command makes the sequence too long for a 20-char robot command
                    if sequence.function_size() > max_length {
                        break;
                    }
                    // Record the sequence if it's long enough
                    let base = (i, offset + 1);
                    let current = (j, offset + 1);
                    if sequence.len() >= min_commands && !seen.contains(&current) {
                        seen.insert(current);
                        duplicates.entry(base).or_insert(vec![i]).push(j);
                    }
                }
            }
        }
        // Invert the index, to map start positions to possible (start, length) sequences
        let mut sequences_from = AvailableSequenceIndex::new();
        for (sequence, starts) in duplicates.iter() {
            for start in starts {
                sequences_from.entry(*start).or_insert(Vec::new()).push(*sequence);
            }
        }
        return (duplicates, sequences_from);
    }

    fn compress(&self, sequences_from: &AvailableSequenceIndex, max_routines: usize, max_length: usize) -> Option<CompressedSequence> {
        assert!(max_routines > 0 && max_routines <= 26);

        let find_sequence = || -> Option<Vec<SequenceDef>> {
            // Depth-first search of possible compressions of the path using the index of available sequences
            let mut stack: Vec<(Vec<SequenceDef>, usize)> = Vec::new();
            // Start with empty main program
            stack.push((Vec::new(), 0));
            while !stack.is_empty() {
                let (main, len) = stack.pop().unwrap();
                // If this main program is going to be too long (where each index is 1 char, and separated by commas), skip it
                if main.len() > 0 && main.len() * 2 - 1 > max_length {
                    continue;
                }
                // If this main program covers the entire path, we're done
                if len == self.len() {
                    return Some(main);
                }
                // Otherwise, try next sequences that don't take us over the max_routines limit
                let set: BTreeSet<SequenceDef> = main.iter().cloned().collect();
                for c in sequences_from.get(&len).unwrap_or(&Vec::new()) {
                    if set.len() < max_routines || set.contains(c) {
                        let mut next = main.clone();
                        next.push(*c);
                        stack.push((next, len + c.1));
                    }
                }
            }
            return None;
        };

        let rewrite_compressed = |compressed: Vec<SequenceDef>| -> CompressedSequence {
            let mut new_indexes = (b'A' .. b'A' + max_routines as u8).map(char::from);
            let mut indexes: BTreeMap<SequenceDef, char> = BTreeMap::new();
            let rewritten = compressed
                .iter()
                .map(|s| {
                    *indexes
                        .entry(*s)
                        .or_insert_with(|| new_indexes.next().unwrap())
                })
                .collect();
            (rewritten, indexes.iter().map(|(k, v)| (*v, *k)).collect())
        };

        find_sequence().map(rewrite_compressed)
    }
}

#[derive(Clone,Debug,Eq,PartialEq)]
struct Robot {
    position: Point2D,
    direction: Direction,
}

impl Robot {
    fn rotate_left(&self) -> Robot {
        Robot {
            position: self.position,
            direction: self.direction.rotate_left(),
        }
    }

    fn rotate_right(&self) -> Robot {
        Robot {
            position: self.position,
            direction: self.direction.rotate_right(),
        }
    }

    fn go_forward(&self, n: i32) -> Robot {
        Robot {
            position: self.position + self.direction.to_vector() * n,
            direction: self.direction,
        }
    }

    fn apply_command(&self, command: Command) -> Robot {
        match command {
            Command::Left => self.rotate_left(),
            Command::Right => self.rotate_right(),
            Command::Forward(n) => self.go_forward(n),
        }
    }

    fn apply_commands(&self, commands: &[Command]) -> Robot {
        commands.iter().fold(self.clone(), |r, c| r.apply_command(*c))
    }
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
    fn new(data: &[String]) -> Map {
        let data: Vec<Vec<char>> = data
            .iter()
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

    fn find_path(&self) -> Path {
        let mut path = Path::new();
        let mut robot = self.robot.clone();
        loop {
            let commands = vec![
                vec![Command::Forward(1)],
                vec![Command::Left, Command::Forward(1)],
                vec![Command::Right, Command::Forward(1)],
            ];
            let robots: Vec<Robot> = commands.iter().map(|c| robot.apply_commands(c)).collect();
            if let Some((c, r)) = commands.iter().zip(robots.iter()).filter(|(_, r)| self.is_on_scaffold(r)).nth(0) {
                robot = r.clone();
                path.extend_from_slice(c);
            } else {
                break;
            }
        }
        return path;
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

    fn is_on_scaffold(&self, robot: &Robot) -> bool {
        match self.get(&robot.position) {
            Some('#') => true,
            _ => false,
        }
    }
}

pub fn part1() -> i32 {
    let mut emulator = Emulator::from_data_file("day17_input.txt");
    emulator.run();
    let initial_map_data: Vec<String> = repeat_with(|| emulator.read_line())
        .flatten()
        .take_while(|s| s.len() > 0)
        .collect();
    let map = Map::new(&initial_map_data);
//    map.print(Some(&map.robot));
    let intersections = map.find_intersections();
    intersections.iter().map(|p| p.x * p.y).sum()
}

pub fn part2() -> Word {
    let mut emulator = Emulator::from_data_file("day17_input.txt");
    // Wake the robot
    emulator.set(0, 2);
    // Run until the robot waits for input
    assert_eq!(emulator.run(), State::ReadWait);

    // Get the initial video frame & extract a scaffold map from it
    let initial_map_data: Vec<String> = repeat_with(|| emulator.read_line())
        .flatten()
        .take_while(|s| s.len() > 0)
        .collect();
    let initial_map = Map::new(&initial_map_data);
//    println!("Initial map:"); initial_map.print(Some(&initial_map.robot));
    // Find the path through the scaffold
    let path = initial_map.find_path();
    let simplified_path = path.simplify();
    // Compress the path
    let (_duplicates, sequences_from) = simplified_path.find_duplicate_sequences(3, MAX_ROUTINE_LENGTH);
    let compressed_path = simplified_path.compress(&sequences_from, MAX_ROUTINES, MAX_ROUTINE_LENGTH).unwrap();

    // Feed the input to the robot
    assert_eq!(emulator.read_line(), Some("Main:".to_string()));
    emulator.write_line(
        compressed_path.0
            .iter()
            .map(char::to_string)
            .collect::<Vec<String>>()
            .join(",")
            .as_str());
    assert_eq!(emulator.run(), State::ReadWait);
    assert_eq!(emulator.read_line(), Some("Function A:".to_string()));
    emulator.write_line(
        compressed_path.1
            .get(&'A')
            .map(|(start, len)| Path::from(&simplified_path[*start .. *start + *len]).to_string())
            .unwrap_or("".to_string())
            .as_str()
    );
    assert_eq!(emulator.run(), State::ReadWait);
    assert_eq!(emulator.read_line(), Some("Function B:".to_string()));
    emulator.write_line(
        compressed_path.1
            .get(&'B')
            .map(|(start, len)| Path::from(&simplified_path[*start .. *start + *len]).to_string())
            .unwrap_or("".to_string())
            .as_str()
    );
    assert_eq!(emulator.run(), State::ReadWait);
    assert_eq!(emulator.read_line(), Some("Function C:".to_string()));
    emulator.write_line(
        compressed_path.1
            .get(&'C')
            .map(|(start, len)| Path::from(&simplified_path[*start .. *start + *len]).to_string())
            .unwrap_or("".to_string())
            .as_str()
    );
    assert_eq!(emulator.run(), State::ReadWait);
    assert_eq!(emulator.read_line(), Some("Continuous video feed?".to_string()));
    emulator.write_line("n");
    // Run until the robot is finished
    assert_eq!(emulator.run(), State::Halt);
    assert_eq!(emulator.read_line(), Some("".to_string()));

    // Get the final video frame & extract a scaffold map from it
    let _final_map_data: Vec<String> = repeat_with(|| emulator.read_line())
        .flatten()
        .take_while(|s| s.len() > 0)
        .collect();
//    let final_map = Map::new(&_final_map_data);
//    println!("Final map:");final_map.print(Some(&final_map.robot));

    // Get the final output value
    let dust = emulator.read().unwrap();
    // (Make sure this is the end of the output)
    assert_eq!(emulator.read(), None);
    dust
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
        assert_eq!(part2(), 578918);
    }
}
