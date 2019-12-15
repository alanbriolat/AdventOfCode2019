use std::cmp::{max, min};
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops;
use std::path::Path;
use std::str::FromStr;

fn open_data(filename: &str) -> io::BufReader<File>{
    let path = Path::new("data").join(filename);
    let file = File::open(path).unwrap();
    io::BufReader::new(file)
}

pub fn read_lines(filename: &str) -> Vec<String> {
    let reader = open_data(filename);
    reader.lines().map(|x| x.unwrap()).collect()
}

pub fn read_data<T>(filename: &str) -> Vec<T>
    where T: FromStr, <T as FromStr>::Err: Debug {
    let reader = open_data(filename);
    let mut data: Vec<T> = Vec::new();
    for line in reader.lines() {
        data.push(line.unwrap().parse::<T>().unwrap())
    }
    data
}

macro_rules! vector {
    ($x:expr, $y:expr) => { Vector2D{x: $x, y: $y} };
    ($x:expr, $y:expr, $z:expr) => { Vector3D{x: $x, y: $y, z: $z} };
}

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
pub struct Vector2D {
    pub x: i32,
    pub y: i32,
}

impl Vector2D {
    pub fn manhattan_length(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn to_unit_vector(&self) -> Vector2D {
        // Use GCD implementation copied from https://doc.rust-lang.org/std/ops/trait.Div.html
        let mut x = self.x.abs();
        let mut y = self.y.abs();
        while y != 0 {
            let t = y;
            y = x % y;
            x = t;
        }
        Vector2D{x: self.x / x, y: self.y / x}
    }

    pub fn min(&self, other: &Vector2D) -> Vector2D {
        Vector2D {
            x: min(self.x, other.x),
            y: min(self.y, other.y),
        }
    }

    pub fn max(&self, other: &Vector2D) -> Vector2D {
        Vector2D {
            x: max(self.x, other.x),
            y: max(self.y, other.y),
        }
    }
}

impl ops::Add<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Vector2D) -> Self::Output {
        Vector2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign<Vector2D> for Vector2D {
    fn add_assign(&mut self, rhs: Vector2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn sub(self, rhs: Vector2D) -> Self::Output {
        Vector2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::SubAssign<Vector2D> for Vector2D {
    fn sub_assign(&mut self, rhs: Vector2D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[derive(Clone,Copy,Debug,Default,Eq,Hash,PartialEq)]
pub struct Vector3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vector3D {
    pub fn manhattan_length(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    pub fn signum(&self) -> Vector3D {
        Vector3D {
            x: self.x.signum(),
            y: self.y.signum(),
            z: self.z.signum(),
        }
    }
}

impl ops::Add<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: Vector3D) -> Self::Output {
        Vector3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign<Vector3D> for Vector3D {
    fn add_assign(&mut self, rhs: Vector3D) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: Vector3D) -> Self::Output {
        Vector3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::SubAssign<Vector3D> for Vector3D {
    fn sub_assign(&mut self, rhs: Vector3D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

macro_rules! point {
    ($x:expr, $y:expr) => { Point2D{x: $x, y: $y} };
    ($x:expr, $y:expr, $z:expr) => { Point3D{x: $x, y: $y, z: $z} };
}

pub type Point2D = Vector2D;
pub type Point3D = Vector3D;

#[derive(Clone,Debug,Eq,PartialEq)]
pub struct BoundingBox2D {
    pub min: Point2D,
    pub max: Point2D,
}

impl BoundingBox2D {
    pub fn new(initial: &Point2D) -> BoundingBox2D {
        BoundingBox2D {
            min: initial.clone(),
            max: initial.clone(),
        }
    }

    pub fn include(&mut self, point: &Point2D) {
        self.min = self.min.min(point);
        self.max = self.max.max(point);
    }

    pub fn contains(&self, point: &Point2D) -> bool {
        self.min.x <= point.x && point.x <= self.max.x && self.min.y <= point.y && point.y <= self.max.y
    }

    pub fn iter(&self) -> impl Iterator<Item=Point2D> + '_ {
        (self.min.y ..= self.max.y).flat_map(move |y| (self.min.x ..= self.max.x).map(move |x| point!(x, y)))
    }
}

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
pub struct Line2D {
    pub start: Point2D,
    pub end: Point2D,
}

impl Line2D {
    pub fn manhattan_length(&self) -> i32 {
        (self.end - self.start).manhattan_length()
    }
}

#[derive(Debug,Eq,PartialEq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Debug,Eq,Hash,PartialEq)]
pub struct Intersection(pub Point2D, pub i32, pub i32);

impl Line2D {
    pub fn axis(&self) -> Option<Axis> {
        // Aligned with axis 0 means axis 1 values are the same
        let aligned_0 = (self.start).y == (self.end).y;
        // Aligned with axis 1 means axis 0 values are the same
        let aligned_1 = (self.start).x == (self.end).x;
        match (aligned_0, aligned_1) {
            (true, false) => Some(Axis::Horizontal),
            (false, true) => Some(Axis::Vertical),
            _ => None,
        }
    }

    pub fn bounding_box(&self) -> (Point2D, Point2D) {
        (
            point!(min((self.start).x, (self.end).x), min((self.start).y, (self.end).y)),
            point!(max((self.start).x, (self.end).x), max((self.start).y, (self.end).y)),
        )
    }

    pub fn intersection_with(&self, other: &Line2D) -> Option<Intersection> {
        // Only allow axis-aligned lines, will panic if not
        let axis_self = self.axis().unwrap();
        let axis_other = other.axis().unwrap();
        // Parallel lines never intersect!
        if axis_self == axis_other {
            return None;
        };
        // Make orientation predictable
        let (a, b) = if axis_self == Axis::Horizontal {
            (self, other)
        } else {
            (other, self)
        };
        // Vertical line's X coordinate within horizontal line's range
        let overlap_x = (min(a.start.x, a.end.x) ..= max(a.start.x, a.end.x)).contains(&b.start.x);
        // Horizontal line's Y coordinate within vertical line's range
        let overlap_y = (min(b.start.y, b.end.y) ..= max(b.start.y, b.end.y)).contains(&a.start.y);

        if overlap_x && overlap_y {
            let p = point!(b.start.x, a.start.y);
            if axis_self == Axis::Horizontal {
                Some(Intersection(p, (p.x - self.start.x).abs(), (p.y - other.start.y).abs()))
            } else {
                Some(Intersection(p, (p.y - self.start.y).abs(), (p.x - other.start.x).abs()))
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point2d_to_unit_vector() {
        // One coordinate is prime, so no division
        assert_eq!(Vector2D{x: 12, y: 17}.to_unit_vector(), Vector2D{x: 12, y: 17});
        assert_eq!(Vector2D{x: 11, y: 16}.to_unit_vector(), Vector2D{x: 11, y: 16});
        // GCD is one of the coordinates
        assert_eq!(Vector2D{x: 12, y: 36}.to_unit_vector(), Vector2D{x: 1, y: 3});
        // GCD is not one of the coordinates
        assert_eq!(Vector2D{x: 12, y: 16}.to_unit_vector(), Vector2D{x: 3, y: 4});
        // Signs are preserved
        assert_eq!(Vector2D{x: 12, y: -16}.to_unit_vector(), Vector2D{x: 3, y: -4});
        assert_eq!(Vector2D{x: -12, y: 16}.to_unit_vector(), Vector2D{x: -3, y: 4});
        assert_eq!(Vector2D{x: -12, y: -16}.to_unit_vector(), Vector2D{x: -3, y: -4});
    }
}
