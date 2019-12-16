use std::collections::HashMap;
extern crate num;
use num::Integer;
use crate::util::{Vector3D, Point3D, read_lines};

/// The state of the system, i.e. the state of every moon
type State = Vec<Moon>;
/// The state of the system in only one axis, see Moon::x() etc.
type SubState = Vec<(i32, i32)>;

/// Parse a single `<x=X, y=Y, z=Z>` point
fn parse_point3d(input: &str) -> Point3D {
    let parts: Vec<&str> = input[1 .. input.len()-1].split(", ").collect();
    Point3D {
        x: parts[0].split("=").nth(1).unwrap().parse().unwrap(),
        y: parts[1].split("=").nth(1).unwrap().parse().unwrap(),
        z: parts[2].split("=").nth(1).unwrap().parse().unwrap(),
    }
}

/// Read file as a sequence of points
fn parse_input_points(filename: &str) -> Vec<Point3D> {
    read_lines(filename).into_iter().map(|x| parse_point3d(x.as_str())).collect()
}

/// Read file as a sequence of moons (i.e. system state) with velocity of 0
fn read_input(filename: &str) -> State {
    parse_input_points(filename).into_iter().map(|p| Moon{position: p, velocity: vector!(0, 0, 0)}).collect()
}

#[derive(Clone,Debug,Eq,PartialEq,Hash)]
struct Moon {
    position: Point3D,
    velocity: Vector3D,
}

impl Moon {
    fn energy(&self) -> i32 {
        self.position.manhattan_length() * self.velocity.manhattan_length()
    }

    /// Get state in the x axis as `(position, velocity)`
    fn x(&self) -> (i32, i32) {
        return (self.position.x, self.velocity.x)
    }

    /// Get state in the y axis as `(position, velocity)`
    fn y(&self) -> (i32, i32) {
        return (self.position.y, self.velocity.y)
    }

    /// Get state in the z axis as `(position, velocity)`
    fn z(&self) -> (i32, i32) {
        return (self.position.z, self.velocity.z)
    }
}

/// Simulate the system by one step in a specific axis only
macro_rules! simulate_axis {
    ($data:expr, $dim:ident) => {
        // Update velocities
        for i in 0 .. $data.len() {
            for j in i + 1 .. $data.len() {
                let dv = ($data[j].position.$dim - $data[i].position.$dim).signum();
                $data[i].velocity.$dim += dv;
                $data[j].velocity.$dim -= dv;
            }
        }
        // Update positions from velocities
        for m in $data.iter_mut() {
            m.position.$dim += m.velocity.$dim;
        }
    };
}

/// Get the state of the system in a specific axis
macro_rules! substate {
    ($data:expr, $dim:ident) => {
        $data.iter().map(Moon::$dim).collect()
    }
}

/// Find cycle length of system state in a specific axis
macro_rules! find_cycle {
    ($data:expr, $dim:ident) => {{
        let mut history: HashMap<SubState, usize> = HashMap::new();
        history.insert(substate!($data, $dim), 0);
        let mut result: usize = 0;
        for i in 1 .. {
            simulate_axis!($data, $dim);
            let state = substate!($data, $dim);
            if let Some(iteration) = history.get(&state) {
                result = i - *iteration;
                break;
            } else {
                history.insert(state, i);
            }
        }
        result
    }}
}

fn simulate_step(moons: &mut State) {
    simulate_axis!(moons, x);
    simulate_axis!(moons, y);
    simulate_axis!(moons, z);
}

pub fn part1() -> i32 {
    let mut state = read_input("day12_input.txt");
    for _ in 0 .. 1000 {
        simulate_step(&mut state);
    }
    state.iter().map(Moon::energy).sum()
}

pub fn part2() -> usize {
    let mut state = read_input("day12_input.txt");
    let cycle_x = find_cycle!(state, x);
    let cycle_y = find_cycle!(state, y);
    let cycle_z = find_cycle!(state, z);
    cycle_x.lcm(&cycle_y).lcm(&cycle_z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_vector3d() {
        assert_eq!(parse_point3d("<x=12, y=-3, z=1>"), vector!(12, -3, 1));
    }

    #[test]
    fn test_parse_input_points() {
        assert_eq!(parse_input_points("day12_input.txt"), vec![
            point!(16, -8, 13),
            point!(4, 10, 10),
            point!(17, -5, 6),
            point!(13, -3, 0),
        ]);
    }

    #[test]
    fn test_read_input_example1() {
        assert_eq!(read_input("day12_input.txt"), vec![
            Moon{position: point!(16, -8, 13), velocity: vector!(0, 0, 0)},
            Moon{position: point!(4, 10, 10), velocity: vector!(0, 0, 0)},
            Moon{position: point!(17, -5, 6), velocity: vector!(0, 0, 0)},
            Moon{position: point!(13, -3, 0), velocity: vector!(0, 0, 0)},
        ]);
    }

    #[test]
    fn test_simulate_step_example1() {
        let mut moons = read_input("day12_example1.txt");
        simulate_step(&mut moons);
        assert_eq!(moons, vec![
            Moon{position: point!(2, -1, 1), velocity: vector!(3, -1, -1)},
            Moon{position: point!(3, -7, -4), velocity: vector!(1, 3, 3)},
            Moon{position: point!(1, -7, 5), velocity: vector!(-3, 1, -3)},
            Moon{position: point!(2, 2, 0), velocity: vector!(-1, -3, 1)},
        ]);
        simulate_step(&mut moons);
        assert_eq!(moons, vec![
            Moon{position: point!(5, -3, -1), velocity: vector!(3, -2, -2)},
            Moon{position: point!(1, -2, 2), velocity: vector!(-2, 5, 6)},
            Moon{position: point!(1, -4, -1), velocity: vector!(0, 3, -6)},
            Moon{position: point!(1, -4, 2), velocity: vector!(-1, -6, 2)},
        ]);
    }

    #[test]
    fn test_simulate_example2() {
        let mut moons = read_input("day12_example2.txt");
        for _ in 0..100 {
            simulate_step(&mut moons);
        }
        assert_eq!(moons, vec![
            Moon { position: point!(8, -12, -9), velocity: vector!(-7, 3, 0) },
            Moon { position: point!(13, 16, -3), velocity: vector!(3, -11, -5) },
            Moon { position: point!(-29, -11, -1), velocity: vector!(-3, 7, 4) },
            Moon { position: point!(16, -13, 23), velocity: vector!(7, 1, 1) },
        ]);
        assert_eq!(moons.iter().map(Moon::energy).sum::<i32>(), 1940);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 7687);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 334945516288044);
    }
}
