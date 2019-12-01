use std::cmp::max;

use crate::util;

fn calc_fuel(mass: &i32) -> i32 {
    max(mass / 3 - 2, 0)
}

fn calc_fuel_recursive(mass: &i32) -> i32 {
    let mut total = 0;
    let mut fuel = calc_fuel(&mass);
    while fuel > 0 {
        total += fuel;
        fuel = calc_fuel(&fuel);
    }
    return total;
}

pub fn part1() -> i32 {
    let data: Vec<i32> = util::read_data("day01_input.txt");
    data.iter().map(calc_fuel).sum()
}

pub fn part2() -> i32 {
    let data: Vec<i32> = util::read_data("day01_input.txt");
    data.iter().map(calc_fuel_recursive).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_fuel() {
        assert_eq!(calc_fuel(&12), 2);
        assert_eq!(calc_fuel(&14), 2);
        assert_eq!(calc_fuel(&1969), 654);
        assert_eq!(calc_fuel(&100756), 33583);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 3224048);
    }

    #[test]
    fn test_calc_fuel_recursive() {
        assert_eq!(calc_fuel_recursive(&14), 2);
        assert_eq!(calc_fuel_recursive(&1969), 966);
        assert_eq!(calc_fuel_recursive(&100756), 50346);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 4833211);
    }
}
