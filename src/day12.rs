use crate::util::{Vector3D, Point3D, read_lines};

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

#[derive(Debug,Eq,PartialEq)]
struct Moon {
    position: Point3D,
    velocity: Vector3D,
}

impl Moon {
    fn energy(&self) -> i32 {
        self.position.manhattan_length() * self.velocity.manhattan_length()
    }
}

fn read_input(filename: &str) -> Vec<Moon> {
    parse_input_points(filename).into_iter().map(|p| Moon{position: p, velocity: vector!(0, 0, 0)}).collect()
}

/// Direction that a should move to go towards b
#[inline(always)]
fn direction(a: i32, b: i32) -> i32 {
    (b - a).signum()
}

fn simulate_step(moons: &mut Vec<Moon>) {
    // Update velocities
    for i in 0 .. moons.len() {
        for j in 0 .. moons.len() {
            if i != j {
                // Get velocity change to pull a towards b
                let dv = Vector3D {
                    x: direction(moons[i].position.x, moons[j].position.x),
                    y: direction(moons[i].position.y, moons[j].position.y),
                    z: direction(moons[i].position.z, moons[j].position.z),
                };
                // Apply to a (the inverse will get applied to b later in the iteration)
                // TODO: only process each pair once
                moons[i].velocity += dv;
            }
        }
    }
    // Update positions
    for moon in moons.iter_mut() {
        moon.position += moon.velocity;
    }
}

fn simulate(moons: &mut Vec<Moon>, steps: usize) {
    for _ in 0 .. steps {
        simulate_step(moons);
    }
}

fn total_energy(moons: &Vec<Moon>) -> i32 {
    moons.iter().map(|moon| moon.energy()).sum()
}

pub fn part1() -> i32 {
    let mut moons = read_input("day12_input.txt");
    simulate(&mut moons, 1000);
    total_energy(&moons)
}

pub fn part2() -> i32 {
    0
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
        simulate(&mut moons, 100);
        assert_eq!(moons, vec![
            Moon{position: point!(8, -12, -9), velocity: vector!(-7, 3, 0)},
            Moon{position: point!(13, 16, -3), velocity: vector!(3, -11, -5)},
            Moon{position: point!(-29, -11, -1), velocity: vector!(-3, 7, 4)},
            Moon{position: point!(16, -13, 23), velocity: vector!(7, 1, 1)},
        ]);
    }

    #[test]
    fn test_total_energy_example2() {
        let mut moons = read_input("day12_example2.txt");
        simulate(&mut moons, 100);
        assert_eq!(total_energy(&moons), 1940);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 7687);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), unimplemented!());
    }
}
