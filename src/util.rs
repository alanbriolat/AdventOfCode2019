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

pub fn read_data<T>(filename: &str) -> Vec<T>
    where T: FromStr, <T as FromStr>::Err: Debug {
    let reader = open_data(filename);
    let mut data: Vec<T> = Vec::new();
    for line in reader.lines() {
        data.push(line.unwrap().parse::<T>().unwrap())
    }
    data
}
