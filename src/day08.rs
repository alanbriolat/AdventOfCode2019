use crate::util;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const SIZE: usize = WIDTH * HEIGHT;

const BLACK: u8 = b'0';
const WHITE: u8 = b'1';
const TRANSPARENT: u8 = b'2';

fn count_byte(layer: &[u8], byte: u8) -> usize {
    layer.iter().filter(|&x| *x == byte).count()
}

fn merge_layers(current: &mut [u8], next: &[u8]) {
    for (a, b) in current.iter_mut().zip(next.iter()) {
        if *a == TRANSPARENT {
            *a = *b;
        }
    }
}

fn get_checksum(data: &[u8]) -> usize {
    data
        .chunks(SIZE)
        .min_by_key(|&chunk| count_byte(chunk, BLACK))
        .map(|chunk| count_byte(chunk, WHITE) * count_byte(chunk, TRANSPARENT))
        .unwrap()
}

pub fn part1() -> usize {
    let data = util::read_lines("day08_input.txt").into_iter().nth(0).unwrap().into_bytes();
    get_checksum(data.as_slice())
}

pub fn part2() -> String {
    let data = util::read_lines("day08_input.txt").into_iter().nth(0).unwrap().into_bytes();
    let layers: Vec<&[u8]> = data.chunks(SIZE).collect();
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
