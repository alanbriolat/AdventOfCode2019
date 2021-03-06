use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

use crate::util;
use crate::util::*;

#[derive(Debug)]
struct PathSegment {
    next: Line2D,
    cost: i32,
}

#[derive(Debug,Eq,PartialEq)]
struct Wire {
    vectors: Vec<Vector2D>,
    points: Vec<Point2D>,
    lines: Vec<Line2D>,
}

impl FromStr for Wire {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vectors: Vec<Vector2D> = s
            .split(",")
            .map(|x| x.split_at(1))
            .map(|(d, v)| match (d, v.parse::<i32>().unwrap()) {
                ("L", v) => vector!(-v, 0),
                ("R", v) => vector!(v, 0),
                ("U", v) => vector!(0, v),
                ("D", v) => vector!(0, -v),
                _ => panic!("unrecognised direction"),
            })
            .collect();
        let mut points: Vec<Point2D> = vec![point!(0, 0)];
        points.extend(
            vectors
            .iter()
            .scan(point!(0, 0), |state, x| -> Option<Point2D> {
                *state = *state + *x;
                Some(*state)
            })
        );
        let lines: Vec<Line2D> = points.as_slice().windows(2)
            .map(|s| {
                match s {
                    [p1, p2] => Line2D{start: *p1, end: *p2},
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
    let horizontal_overlap = (l1_min.x ..= l1_max.x).contains(&l2_max.x);
    // Horizontal line's Y coordinate within vertical line's range
    let vertical_overlap = (l2_min.y ..= l2_max.y).contains(&l1_min.y);
    if horizontal_overlap && vertical_overlap {
        // Intersection must be at X coordinate of vertical line and Y coordinate of horizontal line
        return Some(point!(l2_min.x, l1_min.y));
    } else {
        return None;
    }
}

fn find_intersections(a: &[Line2D], b: &[Line2D], f: impl Fn(&Line2D, &Line2D) -> Option<Point2D>) -> HashSet<Point2D> {
    iproduct!(a, b)
        .filter_map(|(a, b)| f(a, b))
        .collect()
}

fn find_intersections_with_costs(a: &[Line2D], b: &[Line2D]) -> Vec<Intersection> {
    let a_with_costs: Vec<(&Line2D, i32)> =
        a.iter()
            .scan(0, |state, l| {
                let old_state = *state;
                *state += l.manhattan_length();
                Some((l, old_state))
            })
            .collect();
    let b_with_costs: Vec<(&Line2D, i32)> =
        b.iter()
            .scan(0, |state, l| {
                let old_state = *state;
                *state += l.manhattan_length();
                Some((l, old_state))
            })
            .collect();
    iproduct!(a_with_costs, b_with_costs)
        .filter_map(|((a_line, a_base), (b_line, b_base))| {
            a_line.intersection_with(b_line).and_then(|Intersection(p, a_cost, b_cost)| {
                Some(Intersection(p, a_base + a_cost, b_base + b_cost))
            })
        })
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
        .map(|&x| (x - point!(0, 0)).manhattan_length())
        .filter(|&x| x > 0)
        .collect();
    distances.sort();
    *distances.first().unwrap()
}

pub fn part2() -> i32 {
    let wires: Vec<Wire> = util::read_data("day03_input.txt");
    let wire1 = &wires[0];
    let wire2 = &wires[1];
    let intersections = find_intersections_with_costs(&wire1.lines, &wire2.lines);
    let mut distances: Vec<i32> =
        intersections
        .iter()
        .map(|Intersection(_, a_cost, b_cost)| a_cost + b_cost)
        .filter(|&x| x > 0)
        .collect();
    distances.sort();
    *distances.first().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wire_parser() {
        let wire = "R8,U5,L5,D3".parse::<Wire>().unwrap();
        assert_eq!(wire, Wire {
            vectors: vec![vector!(8, 0), vector!(0, 5), vector!(-5, 0), vector!(0, -3)],
            points: vec![point!(0, 0), point!(8, 0), point!(8, 5), point!(3, 5), point!(3, 2)],
            lines: vec![
                Line2D{start: point!(0, 0), end: point!(8, 0)},
                Line2D{start: point!(8, 0), end: point!(8, 5)},
                Line2D{start: point!(8, 5), end: point!(3, 5)},
                Line2D{start: point!(3, 5), end: point!(3, 2)},
            ],
        });
    }

    #[test]
    fn test_axis() {
        assert_eq!(Some(Axis::Horizontal), Line2D{start: point!(0, 5), end: point!(5, 5)}.axis());
        assert_eq!(Some(Axis::Vertical), Line2D{start: point!(5, 0), end: point!(5, 5)}.axis());
        assert_eq!(None, Line2D{start: point!(5, 5), end: point!(5, 5)}.axis());
        assert_eq!(None, Line2D{start: point!(0, 0), end: point!(5, 5)}.axis());
    }

    #[test]
    fn test_line_intersection() {
        // Nice simple horizontal + vertical lines that have the same end
        assert_eq!(Some(point!(5, 5)), axis_aligned_line_intersection(
            &Line2D{start: point!(0, 5), end: point!(5, 5)},
            &Line2D{start: point!(5, 0), end: point!(5, 5)},
        ));
        // Arguments reversed
        assert_eq!(Some(point!(5, 5)), axis_aligned_line_intersection(
            &Line2D{start: point!(5, 0), end: point!(5, 5)},
            &Line2D{start: point!(0, 5), end: point!(5, 5)},
        ));
        // Line "directions" reversed
        assert_eq!(Some(point!(5, 5)), axis_aligned_line_intersection(
            &Line2D{start: point!(5, 5), end: point!(5, 0)},
            &Line2D{start: point!(5, 5), end: point!(0, 5)},
        ));
        // Lines with same start
        assert_eq!(Some(point!(0, 0)), axis_aligned_line_intersection(
            &Line2D{start: point!(0, 0), end: point!(5, 0)},
            &Line2D{start: point!(0, 0), end: point!(0, 5)},
        ));
        // First line ends on second line
        assert_eq!(Some(point!(5, 5)), axis_aligned_line_intersection(
            &Line2D{start: point!(0, 5), end: point!(5, 5)},
            &Line2D{start: point!(5, 0), end: point!(5, 10)},
        ));
        // Second line ends on first line
        assert_eq!(Some(point!(5, 5)), axis_aligned_line_intersection(
            &Line2D{start: point!(5, 0), end: point!(5, 10)},
            &Line2D{start: point!(0, 5), end: point!(5, 5)},
        ));
        // Lines intersect somewhere that's not a line end
        assert_eq!(Some(point!(5, 3)), axis_aligned_line_intersection(
            &Line2D{start: point!(5, 0), end: point!(5, 10)},
            &Line2D{start: point!(3, 3), end: point!(20, 3)},
        ));
        // Lines don't intersect at all
        assert_eq!(None, axis_aligned_line_intersection(
            &Line2D{start: point!(5, 0), end: point!(5, 10)},
            &Line2D{start: point!(0, 5), end: point!(3, 5)},
        ));
    }

    #[test]
    fn test_line_intersections() {
        let wire1 = "R8,U5,L5,D3".parse::<Wire>().unwrap();
        let wire2 = "U7,R6,D4,L4".parse::<Wire>().unwrap();
        let intersections = find_intersections(&wire1.lines, &wire2.lines, axis_aligned_line_intersection);
        let expected = [point!(0, 0), point!(3, 3), point!(6, 5)].iter().cloned().collect();
        assert_eq!(intersections, expected);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 860);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 9238);
    }
}
