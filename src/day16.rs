use std::cmp::min;
use std::iter::repeat;
use num::range_step;
use crate::util;

const BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];

fn generate_pattern(n: usize) -> impl Iterator<Item=i32> {
    BASE_PATTERN.iter()             // Iterate over the base pattern
        .cycle()                    // ... repeatedly, forever
        .cloned()                   // Copies, not references
        .flat_map(move |x| {        // For each element in the base pattern
            repeat(x).take(n)       // ... repeat it n times
        })
        .skip(1)                    // Shift the pattern left by one place
}

fn read_input(filename: &str) -> Vec<i32> {
    util::read_lines(filename)[0].chars().map(|x| x.to_string().parse().unwrap()).collect()
}

/*
To generate the following digits:
abcdefghijklmnop

Applies the following patterns to the input (a = 1, A = -1):
a_A_a_A_a_A_a_A_
_bb__BB__bb__BB_
__ccc___CCC___cc
___dddd____DDDD_
____eeeee_____EE

So to generate e, i.e. index 4, is:
- The sum of strides of 5, starting at 4, every 20
- ... minus the sum of strides of 5, starting at 14, every 20
*/

fn step(input: &[i32]) -> Vec<i32> {
    (0 .. input.len())
        .map(|offset| {
            let width = offset + 1;
            let interval = width * 4;
            let pos: i32 = stride(input, offset, interval, width)
                .map(|chunk| -> i32 { chunk.iter().sum() }).sum();
            let neg: i32 = stride(input, offset + width * 2, interval, width)
                .map(|chunk| -> i32 { chunk.iter().sum() }).sum();
            (pos - neg).abs() % 10
        })
        .collect()
}

fn stride<T>(data: &[T], offset: usize, interval: usize, width: usize) -> impl Iterator<Item=&[T]>
{
    range_step(offset, data.len(), interval)
        .map(move |i| &data[i .. min(i + width, data.len())])
}

pub fn part1() -> String {
    let mut data = read_input("day16_input.txt");
    for _ in 0 .. 100 {
        data = step(data.as_slice());
    }
    let output: Vec<String> = data[.. 8].iter().map(|x| format!("{}", x)).collect();
    output.join("")
}

pub fn part2() -> String {
    let position: usize = util::read_lines("day16_input.txt")[0][.. 7].parse().unwrap();
    let mut data: Vec<i32> = repeat(read_input("day16_input.txt")).take(10000).flatten().collect();
    for i in 0 .. 100 {
        println!("iteration {}", i);
        data = step(data.as_slice());
    }
    let output: Vec<String> = data[position .. position + 8].iter().map(|x| format!("{}", x)).collect();
    output.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_pattern() {
        assert_eq!(generate_pattern(2).take(15).collect::<Vec<_>>(), vec![0, 1, 1, 0, 0, -1, -1, 0, 0, 1, 1, 0, 0, -1, -1]);
    }

    #[test]
    fn test_step() {
        assert_eq!(step(&[1, 2, 3, 4, 5, 6, 7, 8]), vec![4, 8, 2, 2, 6, 1, 5, 8]);
        assert_eq!(step(&[4, 8, 2, 2, 6, 1, 5, 8]), vec![3, 4, 0, 4, 0, 4, 3, 8]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "82525123");
    }

    #[test]
    fn test_part2() {
//        assert_eq!(part2(), "");
    }
}
