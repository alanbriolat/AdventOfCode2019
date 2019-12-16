use std::iter::repeat;

const BASE_PATTERN: [i8; 4] = [0, 1, 0, -1];

fn generate_pattern(n: usize) -> impl Iterator<Item=i8> {
    BASE_PATTERN.iter()             // Iterate over the base pattern
        .cycle()                    // ... repeatedly, forever
        .cloned()                   // Copies, not references
        .flat_map(move |x| {     // For each element in the base pattern
            repeat(x).take(n)   // ... repeat it n times
        })
        .skip(1)                 // Shift the pattern left by one place
}

pub fn part1() -> i32 {
    0
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
    fn test_part1() {
        assert_eq!(part1(), unimplemented!());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), unimplemented!());
    }
}
