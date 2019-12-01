use std::cmp::max;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn open_data(filename: &str) -> io::BufReader<File>{
    let path = Path::new("data").join(filename);
    let file = File::open(path).unwrap();
    io::BufReader::new(file)
}

fn read_data<T>(filename: &str) -> Vec<T>
where T: FromStr, <T as FromStr>::Err: Debug {
    let reader = open_data(filename);
    let mut data: Vec<T> = Vec::new();
    for line in reader.lines() {
        data.push(line.unwrap().parse::<T>().unwrap())
    }
    data
}

fn day01part1() {
    let data: Vec<i32> = read_data("day01_input.txt");
    let fuel = data.iter().map(|mass| mass / 3 - 2).fold(0, |acc, x| acc + x);
    println!("day01part1: {}", fuel);
}

fn day01part2() {
    fn calc_fuel(mass: &i32) -> i32 {
        max(mass / 3 - 2, 0)
    }

    let data: Vec<i32> = read_data("day01_input.txt");
    let mut total_fuel = 0;
    for mass in data {
        let mut fuel = calc_fuel(&mass);
        while fuel > 0 {
            total_fuel += fuel;
            fuel = calc_fuel(&fuel);
        }
    }
    println!("day01part2: {}", total_fuel);
}

fn main() {
    day01part1();
    day01part2();
}
