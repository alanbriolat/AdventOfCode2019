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
}

macro_rules! point {
    ($x:expr, $y:expr) => { Point2D{x: $x, y: $y} };
}

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

impl ops::Add<Vector2D> for Point2D {
    type Output = Point2D;

    fn add(self, rhs: Vector2D) -> Point2D {
        point!(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub<Point2D> for Point2D {
    type Output = Vector2D;

    fn sub(self, rhs: Point2D) -> Vector2D {
        vector!(self.x - rhs.x, self.y - rhs.y)
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
