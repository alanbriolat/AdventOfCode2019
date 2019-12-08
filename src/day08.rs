use std::collections::HashMap;
use crate::util;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const SIZE: usize = WIDTH * HEIGHT;

#[allow(dead_code)]
const BLACK: u8 = b'0';
const WHITE: u8 = b'1';
const TRANSPARENT: u8 = b'2';

fn count_bytes(layer: &[u8]) -> HashMap<u8, usize> {
    let mut out = HashMap::new();
    for c in layer {
        let counter = out.entry(*c).or_insert(0);
        *counter += 1;
    }
    return out;
}

fn merge_layers(current: &mut [u8], next: &[u8]) {
    for (a, b) in current.iter_mut().zip(next.iter()) {
        if *a == TRANSPARENT {
            *a = *b;
        }
    }
}

pub fn part1() -> usize {
    let data = util::read_lines("day08_input.txt").into_iter().nth(0).unwrap();
    let counters: Vec<HashMap<u8, usize>> = data.as_bytes().chunks(SIZE).map(|chunk| count_bytes(chunk)).collect();
    let most_zeroes = counters.iter().min_by_key(|x| x.get(&b'0').or(Some(&0)).unwrap()).unwrap();
    most_zeroes.get(&b'1').unwrap() * most_zeroes.get(&b'2').unwrap()
}

pub fn part2() -> String {
    let data = util::read_lines("day08_input.txt").into_iter().nth(0).unwrap();
    let layers: Vec<&[u8]> = data.as_bytes().chunks(SIZE).collect();
    let mut current: [u8; SIZE] = [TRANSPARENT; SIZE];
    for layer in layers {
        merge_layers(&mut current, layer);
    }
    let strings: Vec<String> =
        current
        .chunks(WIDTH)
        .map(|x| x.iter().map(|c| if *c == WHITE { 'X' } else { ' ' }).collect())
        .collect();
    format!("\n{}\n", strings.join("\n"))
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
        assert_eq!(part2(), format!("\n{}\n", vec![
            "XXXX X  X   XX X  X X    ",
            "X    X  X    X X  X X    ",
            "XXX  XXXX    X X  X X    ",
            "X    X  X    X X  X X    ",
            "X    X  X X  X X  X X    ",
            "X    X  X  XX   XX  XXXX ",
        ].join("\n")));
    }
}
