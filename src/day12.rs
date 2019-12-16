extern crate num;
use num::Integer;
use crate::util::{Vector3D, Point3D, read_lines};
use std::collections::HashMap;

fn parse_point3d(input: &str) -> Point3D {
    let parts: Vec<&str> = input[1 .. input.len()-1].split(", ").collect();
    Point3D {
        x: parts[0].split("=").nth(1).unwrap().parse().unwrap(),
        y: parts[1].split("=").nth(1).unwrap().parse().unwrap(),
        z: parts[2].split("=").nth(1).unwrap().parse().unwrap(),
    }
}

fn parse_input_points(filename: &str) -> Vec<Point3D> {
    read_lines(filename).into_iter().map(|x| parse_point3d(x.as_str())).collect()
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
}

type State = Vec<Moon>;
type SubState = Vec<(i32, i32)>;

#[derive(Clone)]
struct Simulation {
    initial: State,
    state: State,
}

impl Simulation {
    fn new(state: &State) -> Simulation {
        Simulation {
            initial: state.clone(),
            state: state.clone(),
        }
    }
}

impl Iterator for Simulation {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        self.state = simulate_step(&self.state);
        return Some(self.state.clone());
    }
}

fn read_input(filename: &str) -> State {
    parse_input_points(filename).into_iter().map(|p| Moon{position: p, velocity: vector!(0, 0, 0)}).collect()
}

fn simulate_step(moons: &State) -> State {
    let mut new = moons.clone();
    // Update velocities
    for i in 0 .. new.len() {
        for j in i + 1 .. new.len() {
            // Get velocity change to pull a towards b
            let dv = (new[j].position - new[i].position).signum();
            new[i].velocity += dv;
            new[j].velocity -= dv;
        }
    }
    // Update positions
    for moon in new.iter_mut() {
        moon.position += moon.velocity;
    }
    return new;
}

fn simulate(moons: &State, steps: usize) -> State {
    let mut state: State = moons.clone();
    for _ in 0 .. steps {
        state = simulate_step(&state);
    }
    return state;
}

fn total_energy(moons: &State) -> i32 {
    moons.iter().map(|moon| moon.energy()).sum()
}

pub fn part1() -> i32 {
    let mut simulation = Simulation::new(&read_input("day12_input.txt"));
    let last = simulation.nth(999).unwrap();
    total_energy(&last)
}

fn substate_x(state: &State) -> SubState {
    state.iter().map(|moon| (moon.position.x, moon.velocity.x)).collect()
}

fn substate_y(state: &State) -> SubState {
    state.iter().map(|moon| (moon.position.y, moon.velocity.y)).collect()
}

fn substate_z(state: &State) -> SubState {
    state.iter().map(|moon| (moon.position.z, moon.velocity.z)).collect()
}

pub fn part2() -> usize {
    let simulation = Simulation::new(&read_input("day12_input.txt"));
    let mut substates_x: HashMap<SubState, usize> = HashMap::new();
    let mut substates_y: HashMap<SubState, usize> = HashMap::new();
    let mut substates_z: HashMap<SubState, usize> = HashMap::new();
    let mut cycle_x: Option<(usize, usize)> = None;
    let mut cycle_y: Option<(usize, usize)> = None;
    let mut cycle_z: Option<(usize, usize)> = None;
    let mut count = 0;
    substates_x.insert(substate_x(&simulation.state), count);
    substates_y.insert(substate_y(&simulation.state), count);
    substates_z.insert(substate_z(&simulation.state), count);
    for state in simulation {
        count += 1;
        if cycle_x.is_none() {
            let substate = substate_x(&state);
            if let Some(pos) = substates_x.get(&substate) {
                cycle_x = Some((*pos, count - *pos));
                println!("cycle_x: {:?}", cycle_x);
            } else {
                substates_x.insert(substate, count);
            }
        }
        if cycle_y.is_none() {
            let substate = substate_y(&state);
            if let Some(pos) = substates_y.get(&substate) {
                cycle_y = Some((*pos, count - *pos));
                println!("cycle_y: {:?}", cycle_y);
            } else {
                substates_y.insert(substate, count);
            }
        }
        if cycle_z.is_none() {
            let substate = substate_z(&state);
            if let Some(pos) = substates_z.get(&substate) {
                cycle_z = Some((*pos, count - *pos));
                println!("cycle_z: {:?}", cycle_z);
            } else {
                substates_z.insert(substate, count);
            }
        }
        if cycle_x.is_some() && cycle_y.is_some() && cycle_z.is_some() {
            break;
        }
    }
    cycle_x.unwrap().1.lcm(&cycle_y.unwrap().1).lcm(&cycle_z.unwrap().1)
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
        let moons = read_input("day12_example1.txt");
        let a = simulate_step(&moons);
        assert_eq!(a, vec![
            Moon{position: point!(2, -1, 1), velocity: vector!(3, -1, -1)},
            Moon{position: point!(3, -7, -4), velocity: vector!(1, 3, 3)},
            Moon{position: point!(1, -7, 5), velocity: vector!(-3, 1, -3)},
            Moon{position: point!(2, 2, 0), velocity: vector!(-1, -3, 1)},
        ]);
        let b = simulate_step(&a);
        assert_eq!(b, vec![
            Moon{position: point!(5, -3, -1), velocity: vector!(3, -2, -2)},
            Moon{position: point!(1, -2, 2), velocity: vector!(-2, 5, 6)},
            Moon{position: point!(1, -4, -1), velocity: vector!(0, 3, -6)},
            Moon{position: point!(1, -4, 2), velocity: vector!(-1, -6, 2)},
        ]);
    }

    #[test]
    fn test_simulate_example2() {
        let moons = read_input("day12_example2.txt");
        let last = simulate(&moons, 100);
        assert_eq!(last, vec![
            Moon{position: point!(8, -12, -9), velocity: vector!(-7, 3, 0)},
            Moon{position: point!(13, 16, -3), velocity: vector!(3, -11, -5)},
            Moon{position: point!(-29, -11, -1), velocity: vector!(-3, 7, 4)},
            Moon{position: point!(16, -13, 23), velocity: vector!(7, 1, 1)},
        ]);
    }

    #[test]
    fn test_total_energy_example2() {
        let moons = read_input("day12_example2.txt");
        let last = simulate(&moons, 100);
        assert_eq!(total_energy(&last), 1940);
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
