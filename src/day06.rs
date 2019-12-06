use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::FromIterator;
use std::str::FromStr;

use crate::util;

#[derive(Debug)]
struct Orbit {
    parent: String,
    body: String,
}

impl FromStr for Orbit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let index = s.find(")").unwrap();
        Ok(Orbit {
            parent: s[0 .. index].to_string(),
            body: s[index+1 ..].to_string(),
        })
    }
}

#[derive(Debug)]
struct OrbitMap {
    orbit_data: HashMap<String, HashSet<String>>,
}

impl OrbitMap {
    fn new(orbits: &Vec<Orbit>) -> OrbitMap {
        let mut map = OrbitMap { orbit_data: HashMap::new() };
        for orbit in orbits {
            if !map.orbit_data.contains_key(orbit.parent.as_str()) {
                map.orbit_data.insert(orbit.parent.clone(), HashSet::new());
            }
            map.orbit_data.get_mut(orbit.parent.as_str()).unwrap().insert(orbit.body.clone());
        }
        map
    }
}

fn count_orbits(filename: &str) -> usize {
    let orbits: Vec<Orbit> = util::read_data(filename);
    let map =  OrbitMap::new(&orbits);
    let start: HashSet<String> = vec!["COM".to_string()].into_iter().collect();
    let mut queue: VecDeque<(usize, &HashSet<String>)> = VecDeque::new();
    queue.push_back((0, &start));
    let mut sum: usize = 0;
    loop {
        match queue.pop_front() {
            Some((depth, bodies)) => {
                sum += depth * bodies.len();
                for body in bodies {
                    match map.orbit_data.get(body) {
                        Some(next) => {
                            queue.push_back((depth + 1, next));
                        },
                        None => (),
                    }
                }
            },
            None => break,
        }
    };
    sum
}

pub fn part1() -> usize {
    count_orbits("day06_input.txt")
}

pub fn part2() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_orbits() {
        assert_eq!(count_orbits("day06_example1.txt"), 42);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 119831);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), unimplemented!());
    }
}
