use std::collections::HashSet;
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

fn max_visible(asteroids: &[Point2D]) -> usize {
    (0..asteroids.len())
        .map(|i| count_visible_from(i, asteroids))
        .max()
        .unwrap()
}

pub fn part1() -> usize {
    let asteroids = read_asteroids("day10_input.txt");
    max_visible(asteroids.as_slice())
}

pub fn part2() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_visible_example1() {
        let asteroids = read_asteroids("day10_example1.txt");
        assert_eq!(max_visible(asteroids.as_slice()), 33);
    }

    #[test]
    fn test_max_visible_example2() {
        let asteroids = read_asteroids("day10_example2.txt");
        assert_eq!(max_visible(asteroids.as_slice()), 35);
    }

    #[test]
    fn test_max_visible_example3() {
        let asteroids = read_asteroids("day10_example3.txt");
        assert_eq!(max_visible(asteroids.as_slice()), 41);
    }

    #[test]
    fn test_max_visible_example4() {
        let asteroids = read_asteroids("day10_example4.txt");
        assert_eq!(max_visible(asteroids.as_slice()), 210);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 326);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), unimplemented!());
    }
}
