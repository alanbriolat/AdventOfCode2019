use std::cmp::Reverse;
use std::collections::{HashSet, HashMap};
use crate::util;
use crate::util::{Point2D, Vector2D};

fn read_asteroids(filename: &str) -> Vec<Point2D> {
    let lines = util::read_lines(filename);
    lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line
                .as_bytes()
                .iter()
                .enumerate()
                .filter_map(move |(x, byte)| {
                    if *byte == b'#' {
                        Some(Point2D{x: x as i32, y: y as i32})
                    } else {
                        None
                    }
                })
        })
        .collect()
}

/// Count asteroids visible from the one at index ``i``.
///
/// If an asteroid is "behind" another asteroid, that means the vectors between the station and each
/// asteroid are multiples of the same unit vector. In integer grid space, a "unit vector" should
/// still have integer co-ordinates, which can be determined by applying the greatest common divisor
/// of the 2 co-ordinates. Then, we can count the number of visible asteroids by counting the number
/// of unique unit vectors.
fn count_visible_from(i: usize, asteroids: &[Point2D]) -> usize {
    let station = asteroids[i];
    asteroids[..i].iter().chain(asteroids[i+1..].iter())
        .map(|&a| (a - station).to_unit_vector())
        .collect::<HashSet<Vector2D>>().len()
}

fn max_visible(asteroids: &[Point2D]) -> (usize, usize) {
    (0..asteroids.len())
        .map(|i| (i, count_visible_from(i, asteroids)))
        .max_by_key(|(_, n)| *n)
        .unwrap()
}

type InventoryItem = (Vector2D, Vec<Vector2D>);
type Inventory = Vec<InventoryItem>;

fn inventory(i: usize, asteroids: &[Point2D]) -> Inventory {
    let station = asteroids[i];
    let mut data: HashMap<Vector2D, Vec<Vector2D>> = HashMap::new();
    for &a in asteroids[..i].iter().chain(asteroids[i+1..].iter()) {
        let pos = a - station;
        let unit = pos.to_unit_vector();
        data.entry(unit).or_insert(Vec::new()).push(pos);
    }
    for direction in data.values_mut() {
        direction.sort_by_key(|a| Reverse(a.manhattan_length()));
    }
    data.into_iter().collect()
}

/// atan2 used in a way to agree with our cartesian/polar coordinate space
fn vector_angle(v: &Vector2D) -> f64 {
    -((v.x as f64).atan2(v.y as f64))
}

fn sort_inventory(data: &mut Inventory) {
    data.sort_by(|(a, _), (b, _)| vector_angle(a).partial_cmp(&vector_angle(b)).unwrap());
}

struct ShootingIterator<'a> {
    inventory: &'a mut Inventory,
    count: usize,
    index_iterator: Box<dyn Iterator<Item=usize>>,
}

impl<'a> ShootingIterator<'a> {
    fn new(inventory: &'a mut Inventory) -> ShootingIterator<'a> {
        let count = inventory.iter().map(|(_, asteroids)| asteroids.len()).sum();
        let index_iterator = Box::new((0..inventory.len()).cycle());
        ShootingIterator { inventory, count, index_iterator }
    }
}

impl<'a> Iterator for ShootingIterator<'a> {
    type Item = Vector2D;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 1 {
            None
        } else {
            loop {
                if let Some(a) = self.inventory[self.index_iterator.next().unwrap()].1.pop() {
                    self.count -= 1;
                    return Some(a);
                }
            }
        }
    }
}

pub fn part1() -> usize {
    let asteroids = read_asteroids("day10_input.txt");
    max_visible(asteroids.as_slice()).1
}

pub fn part2() -> i32 {
    let asteroids = read_asteroids("day10_input.txt");
    let (i, _) = max_visible(asteroids.as_slice());
    let mut inventory = inventory(i, asteroids.as_slice());
    sort_inventory(&mut inventory);
    let mut it = ShootingIterator::new(&mut inventory);
    let last_relative = it.nth(199).unwrap();
    let last = asteroids[i] + last_relative;
    last.x * 100 + last.y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_visible_example1() {
        let asteroids = read_asteroids("day10_example1.txt");
        assert_eq!(max_visible(asteroids.as_slice()).1, 33);
    }

    #[test]
    fn test_max_visible_example2() {
        let asteroids = read_asteroids("day10_example2.txt");
        assert_eq!(max_visible(asteroids.as_slice()).1, 35);
    }

    #[test]
    fn test_max_visible_example3() {
        let asteroids = read_asteroids("day10_example3.txt");
        assert_eq!(max_visible(asteroids.as_slice()).1, 41);
    }

    #[test]
    fn test_max_visible_example4() {
        let asteroids = read_asteroids("day10_example4.txt");
        assert_eq!(max_visible(asteroids.as_slice()).1, 210);
    }

    #[test]
    fn test_vector_angle() {
        let a0 = vector_angle(&vector!(0, -1));
        let a1 = vector_angle(&vector!(1, -1));
        let a2 = vector_angle(&vector!(1, 1));
        let a3 = vector_angle(&vector!(-1, 1));
        let a4 = vector_angle(&vector!(-1, -1));
        assert!(a0 < a1);
        assert!(a1 < a2);
        assert!(a2 < a3);
        assert!(a3 < a4);
    }

    #[test]
    fn test_shooting_iterator_example4() {
        let asteroids = read_asteroids("day10_example4.txt");
        let (station_index, _) = max_visible(asteroids.as_slice());
        let station = asteroids[station_index];
        let mut inventory = inventory(station_index, asteroids.as_slice());
        sort_inventory(&mut inventory);
        let results: Vec<_> = ShootingIterator::new(&mut inventory).collect();
        assert_eq!(results[0], point!(11, 12) - station);
        assert_eq!(results[1], point!(12, 1) - station);
        assert_eq!(results[2], point!(12, 2) - station);
        assert_eq!(results[9], point!(12, 8) - station);
        assert_eq!(results[19], point!(16, 0) - station);
        assert_eq!(results[49], point!(16, 9) - station);
        assert_eq!(results[99], point!(10, 16) - station);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 326);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1623);
    }
}
