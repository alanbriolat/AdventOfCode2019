use std::collections::HashMap;
use crate::util;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn count_bytes(layer: &[u8]) -> HashMap<u8, usize> {
    let mut out = HashMap::new();
    for c in layer {
        let counter = out.entry(*c).or_insert(0);
        *counter += 1;
    }
    return out;
}

pub fn part1() -> usize {
    let data = util::read_lines("day08_input.txt").into_iter().nth(0).unwrap();
    let counters: Vec<HashMap<u8, usize>> = data.as_bytes().chunks(WIDTH * HEIGHT).map(|chunk| count_bytes(chunk)).collect();
    let most_zeroes = counters.iter().min_by_key(|x| x.get(&b'0').or(Some(&0)).unwrap()).unwrap();
    most_zeroes.get(&b'1').unwrap() * most_zeroes.get(&b'2').unwrap()
}

pub fn part2() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 2250);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), unimplemented!());
    }
}
