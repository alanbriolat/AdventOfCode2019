use std::cmp::{max, min};
use std::collections::HashSet;
use std::num::ParseIntError;
use std::ops;
use std::str::FromStr;

use crate::util;

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
struct Vec2D(i32, i32);

impl Vec2D {
    fn manhattan_length(&self) -> i32 {
        (self.0).abs() + (self.1).abs()
    }
}

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
struct Point2D(i32, i32);

impl ops::Add<Vec2D> for Point2D {
    type Output = Point2D;

    fn add(self, rhs: Vec2D) -> Point2D {
        Point2D(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::Sub<Point2D> for Point2D {
    type Output = Vec2D;

    fn sub(self, rhs: Point2D) -> Vec2D {
        Vec2D(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
struct Line2D(Point2D, Point2D);

#[derive(Debug,Eq,PartialEq)]
enum Axis {
    Horizontal,
    Vertical,
}

impl Line2D {
    fn axis(&self) -> Option<Axis> {
        // Aligned with axis 0 means axis 1 values are the same
        let aligned_0 = (self.0).1 == (self.1).1;
        // Aligned with axis 1 means axis 0 values are the same
        let aligned_1 = (self.0).0 == (self.1).0;
        match (aligned_0, aligned_1) {
            (true, false) => Some(Axis::Horizontal),
            (false, true) => Some(Axis::Vertical),
            _ => None,
        }
    }

    fn bounding_box(&self) -> (Point2D, Point2D) {
        (
            Point2D(min((self.0).0, (self.1).0), min((self.0).1, (self.1).1)),
            Point2D(max((self.0).0, (self.1).0), max((self.0).1, (self.1).1)),
        )
    }
}

#[derive(Debug,Eq,PartialEq)]
struct Wire {
    vectors: Vec<Vec2D>,
    points: Vec<Point2D>,
    lines: Vec<Line2D>,
}

impl FromStr for Wire {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vectors: Vec<Vec2D> = s
            .split(",")
            .map(|x| x.split_at(1))
            .map(|(d, v)| match (d, v.parse::<i32>().unwrap()) {
                ("L", v) => Vec2D(-v, 0),
                ("R", v) => Vec2D(v, 0),
                ("U", v) => Vec2D(0, v),
                ("D", v) => Vec2D(0, -v),
                _ => panic!("unrecognised direction"),
            })
            .collect();
        let mut points: Vec<Point2D> = vec![Point2D(0, 0)];
        points.extend(
            vectors
            .iter()
            .scan(Point2D(0, 0), |state, x| -> Option<Point2D> {
                *state = *state + *x;
                Some(*state)
            })
        );
        let lines: Vec<Line2D> = points.as_slice().windows(2)
            .map(|s| {
                match s {
                    [p1, p2] => Line2D(*p1, *p2),
                    _ => panic!("literally impossible"),
                }
            })
            .collect();
        Ok(Wire { vectors, points, lines })
    }
}

/// Find the intersection of two lines
///
/// Assuming that `l1` and `l2` are both axis-aligned, returns `Some(Point2D)` if the lines
/// intersect or `None` if they do not.
fn axis_aligned_line_intersection(l1: &Line2D, l2: &Line2D) -> Option<Point2D> {
    let a1 = l1.axis().unwrap();
    let a2 = l2.axis().unwrap();
    if a1 == a2 {
        // Parallel lines never intersect!
        return None;
    };
    // Ensure the "first" line is always horizontal and the "second" line is always vertical
    let (l1, l2) = if a1 == Axis::Horizontal { (l1, l2) } else { (l2, l1) };
    // Get bounding boxes to make sure we're dealing with ordered values for range checks
    let (l1_min, l1_max) = l1.bounding_box();
    let (l2_min, l2_max) = l2.bounding_box();
    // Vertical line's X coordinate within horizontal line's range
    let horizontal_overlap = (l1_min.0 ..= l1_max.0).contains(&l2_max.0);
    // Horizontal line's Y coordinate within vertical line's range
    let vertical_overlap = (l2_min.1 ..= l2_max.1).contains(&l1_min.1);
    if horizontal_overlap && vertical_overlap {
        // Intersection must be at X coordinate of vertical line and Y coordinate of horizontal line
        return Some(Point2D(l2_min.0, l1_min.1));
    } else {
        return None;
    }
}

fn find_intersections(a: &[Line2D], b: &[Line2D], f: impl Fn(&Line2D, &Line2D) -> Option<Point2D>) -> HashSet<Point2D> {
    iproduct!(a, b)
        .filter_map(|(a, b)| f(a, b))
        .collect()
}

pub fn part1() -> i32 {
    let wires: Vec<Wire> = util::read_data("day03_input.txt");
    let wire1 = &wires[0];
    let wire2 = &wires[1];
    let intersections = find_intersections(&wire1.lines, &wire2.lines, axis_aligned_line_intersection);
    let mut distances: Vec<i32> =
        intersections
        .iter()
        .map(|&x| (x - Point2D(0, 0)).manhattan_length())
        .filter(|&x| x > 0)
        .collect();
    distances.sort();
    *distances.first().unwrap()
}

pub fn part2() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wire_parser() {
        let wire = "R8,U5,L5,D3".parse::<Wire>().unwrap();
        assert_eq!(wire, Wire {
            vectors: vec![Vec2D(8, 0), Vec2D(0, 5), Vec2D(-5, 0), Vec2D(0, -3)],
            points: vec![Point2D(0, 0), Point2D(8, 0), Point2D(8, 5), Point2D(3, 5), Point2D(3, 2)],
            lines: vec![
                Line2D(Point2D(0, 0), Point2D(8, 0)),
                Line2D(Point2D(8, 0), Point2D(8, 5)),
                Line2D(Point2D(8, 5), Point2D(3, 5)),
                Line2D(Point2D(3, 5), Point2D(3, 2)),
            ],
        });
    }

    #[test]
    fn test_axis() {
        assert_eq!(Some(Axis::Horizontal), Line2D(Point2D(0, 5), Point2D(5, 5)).axis());
        assert_eq!(Some(Axis::Vertical), Line2D(Point2D(5, 0), Point2D(5, 5)).axis());
        assert_eq!(None, Line2D(Point2D(5, 5), Point2D(5, 5)).axis());
        assert_eq!(None, Line2D(Point2D(0, 0), Point2D(5, 5)).axis());
    }

    #[test]
    fn test_line_intersection() {
        // Nice simple horizontal + vertical lines that have the same end
        assert_eq!(Some(Point2D(5, 5)), axis_aligned_line_intersection(
            &Line2D(Point2D(0, 5), Point2D(5, 5)),
            &Line2D(Point2D(5, 0), Point2D(5, 5)),
        ));
        // Arguments reversed
        assert_eq!(Some(Point2D(5, 5)), axis_aligned_line_intersection(
            &Line2D(Point2D(5, 0), Point2D(5, 5)),
            &Line2D(Point2D(0, 5), Point2D(5, 5)),
        ));
        // Line "directions" reversed
        assert_eq!(Some(Point2D(5, 5)), axis_aligned_line_intersection(
            &Line2D(Point2D(5, 5), Point2D(5, 0)),
            &Line2D(Point2D(5, 5), Point2D(0, 5)),
        ));
        // Lines with same start
        assert_eq!(Some(Point2D(0, 0)), axis_aligned_line_intersection(
            &Line2D(Point2D(0, 0), Point2D(5, 0)),
            &Line2D(Point2D(0, 0), Point2D(0, 5)),
        ));
        // First line ends on second line
        assert_eq!(Some(Point2D(5, 5)), axis_aligned_line_intersection(
            &Line2D(Point2D(0, 5), Point2D(5, 5)),
            &Line2D(Point2D(5, 0), Point2D(5, 10)),
        ));
        // Second line ends on first line
        assert_eq!(Some(Point2D(5, 5)), axis_aligned_line_intersection(
            &Line2D(Point2D(5, 0), Point2D(5, 10)),
            &Line2D(Point2D(0, 5), Point2D(5, 5)),
        ));
        // Lines intersect somewhere that's not a line end
        assert_eq!(Some(Point2D(5, 3)), axis_aligned_line_intersection(
            &Line2D(Point2D(5, 0), Point2D(5, 10)),
            &Line2D(Point2D(3, 3), Point2D(20, 3)),
        ));
        // Lines don't intersect at all
        assert_eq!(None, axis_aligned_line_intersection(
            &Line2D(Point2D(5, 0), Point2D(5, 10)),
            &Line2D(Point2D(0, 5), Point2D(3, 5)),
        ));
    }

    #[test]
    fn test_line_intersections() {
        let wire1 = "R8,U5,L5,D3".parse::<Wire>().unwrap();
        let wire2 = "U7,R6,D4,L4".parse::<Wire>().unwrap();
        let intersections = find_intersections(&wire1.lines, &wire2.lines, axis_aligned_line_intersection);
        let expected = [Point2D(0, 0), Point2D(3, 3), Point2D(6, 5)].iter().cloned().collect();
        assert_eq!(intersections, expected);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 860);
    }
}
