use std::collections::{HashMap, HashSet, VecDeque};
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
    adjacent: HashMap<String, HashSet<String>>,
}

impl OrbitMap {
    fn new(orbits: &Vec<Orbit>) -> OrbitMap {
        let mut map = OrbitMap { adjacent: HashMap::new() };
        for orbit in orbits {
            map.add_adjacent(orbit.parent.as_str(), orbit.body.as_str());
        }
        map
    }

    fn add_adjacent(&mut self, a: &str, b: &str) {
        self.get_adjacent_mut(a).insert(b.to_string());
        self.get_adjacent_mut(b).insert(a.to_string());
    }

    fn get_adjacent_mut(&mut self, k: &str) -> &mut HashSet<String> {
        if !self.adjacent.contains_key(k) {
            self.adjacent.insert(k.to_string(), HashSet::new());
        }
        self.adjacent.get_mut(k).unwrap()
    }

    fn get_adjacent(&self, k: &str) -> Option<&HashSet<String>> {
        self.adjacent.get(k)
    }

    fn get_distances_from(&self, k: &str) -> HashMap<String, usize> {
        let mut distances: HashMap<String, usize> = HashMap::new();
        let mut queue: VecDeque<(&HashSet<String>, usize)> = VecDeque::new();
        let start: HashSet<String> = vec![k.to_string()].into_iter().collect();
        queue.push_back((&start, 0));
        loop {
            match queue.pop_front() {
                Some((bodies, depth)) => {
                    for body in bodies {
                        if !distances.contains_key(body.as_str()) {
                            distances.insert(body.clone(), depth);
                            match self.get_adjacent(body.as_str()) {
                                Some(next) => {
                                    queue.push_back((next, depth + 1));
                                },
                                None => (),
                            }
                        }
                    }
                },
                None => break,
            }
        }
        distances
    }
}

fn count_orbits(filename: &str) -> usize {
    let orbits: Vec<Orbit> = util::read_data(filename);
    let map =  OrbitMap::new(&orbits);
    let distances = map.get_distances_from("COM");
    distances.values().sum()
}

fn get_orbital_transfers(filename: &str, a: &str, b: &str) -> usize {
    let orbits: Vec<Orbit> = util::read_data(filename);
    let map =  OrbitMap::new(&orbits);
    let distances = map.get_distances_from(a);
    *distances.get(b).unwrap() - 2
}

pub fn part1() -> usize {
    count_orbits("day06_input.txt")
}

pub fn part2() -> usize {
    get_orbital_transfers("day06_input.txt", "YOU", "SAN")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_orbits() {
        assert_eq!(count_orbits("day06_example1.txt"), 42);
    }

    #[test]
    fn test_get_orbital_transfers() {
        assert_eq!(get_orbital_transfers("day06_example2.txt", "YOU", "SAN"), 4);
        assert_eq!(get_orbital_transfers("day06_input.txt", "YOU", "SAN"),
                   get_orbital_transfers("day06_input.txt", "SAN", "YOU"));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 119831);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 322);
    }
}
