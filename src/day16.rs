use std::cmp::{min, max};
use std::iter::repeat;
use std::ops::Range;
use crate::util;

fn read_input(filename: &str) -> Vec<i32> {
    util::read_lines(filename)[0].chars().map(|x| x.to_string().parse().unwrap()).collect()
}

#[derive(Copy,Clone,Debug)]
struct Stride {
    offset: usize,
    interval: usize,
    width: usize,
}

impl Stride {
    fn iter_chunks<'a, T>(&'a self, data: &'a[T]) -> impl Iterator<Item=&'a[T]> + 'a {
        (self.offset .. data.len())
            .step_by(self.interval)
            .map(move |i| &data[i .. min(i + self.width, data.len())])
    }
}

fn strides_for_index(i: usize) -> (Stride, Stride) {
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
    let width = i + 1;
    let interval = width * 4;
    return (
        Stride {offset: i, interval, width},
        Stride {offset: i + width * 2, interval, width},
    );
}

fn next_value_at(data: &[i32], i: usize) -> i32 {
    let (pos, neg) = strides_for_index(i);
    let pos_val: i32 = pos.iter_chunks(data).map(|chunk| -> i32 {chunk.iter().sum()}).sum();
    let neg_val: i32 = neg.iter_chunks(data).map(|chunk| -> i32 {chunk.iter().sum()}).sum();
    return (pos_val - neg_val).abs() % 10;
}

fn step_range_in_place(data: &mut [i32], range: Range<usize>) {
    let before = range.start .. min(range.end, data.len() / 2);
    let after = max(range.start, data.len() / 2) .. range.end;
    for i in before {
        data[i] = next_value_at(data, i);
    }
    /*
    For every digit in the second half of the data, the pattern to create the new digit expands
    to a multiplier of 1 for that position and everything after it, and therefore the digit is
    created from a simple sum of the remaining data. A naive implementation would calculate the
    sum once per digit, resulting in O(n^2) time complexity, however if we subtract each digit
    from a running total calculated once, it becomes O(n).
    */
    let mut sum: i32 = data[after.clone()].iter().sum();
    for i in after {
        let old = data[i];
        data[i] = sum % 10;
        sum -= old;
    }
}

pub fn part1() -> String {
    let mut data = read_input("day16_input.txt");
    let n = data.len();
    for _ in 0 .. 100 {
        step_range_in_place(&mut data, 0 .. n);
    }
    let output: Vec<String> = data[.. 8].iter().map(|x| format!("{}", x)).collect();
    output.join("")
}

pub fn part2() -> String {
    let position: usize = util::read_lines("day16_input.txt")[0][.. 7].parse().unwrap();
    let mut data: Vec<i32> = repeat(read_input("day16_input.txt")).take(10000).flatten().collect();
    let n = data.len();
    for _ in 0 .. 100 {
        // Only need to run the end of the data, because it's unaffected by anything earlier
        step_range_in_place(data.as_mut_slice(), position .. n);
    }
    let output: Vec<String> = data[position .. position + 8].iter().map(|x| format!("{}", x)).collect();
    output.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), "82525123");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), "49476260");
    }
}
