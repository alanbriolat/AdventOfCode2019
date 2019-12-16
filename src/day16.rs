use std::iter::repeat;
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

fn step(input: &[i32]) -> Vec<i32> {
    (1 ..= input.len())
        .map(|i| -> i32 {
            input.iter().cloned()
                .zip(generate_pattern(i))
                .map(|(a, b)| { a * b })
                .sum::<i32>()
                .abs() % 10
        })
        .collect()
}

pub fn part1() -> String {
    let mut data = read_input("day16_input.txt");
    for _ in 0 .. 100 {
        data = step(data.as_slice());
    }
    let output: Vec<String> = data[.. 8].iter().map(|x| format!("{}", x)).collect();
    output.join("")
}

pub fn part2() -> i32 {
    0
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
        assert_eq!(part2(), unimplemented!());
    }
}
