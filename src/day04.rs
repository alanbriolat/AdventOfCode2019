use std::ops::RangeInclusive;
use itertools::Itertools;

trait Matcher {
    fn apply(&self, digits: &[u8]) -> bool;
}

struct FunctionMatcher(fn(&[u8]) -> bool);

impl Matcher for FunctionMatcher {
    fn apply(&self, digits: &[u8]) -> bool {
        self.0(digits)
    }
}

struct AndMatcher<'a>(&'a[&'a dyn Matcher]);

impl<'a> Matcher for AndMatcher<'a> {
    fn apply(&self, digits: &[u8]) -> bool {
        self.0.iter().all(|f| f.apply(digits))
    }
}

/// Get a non-negative inclusive range of numbers from "start-end" string notation.
fn get_positive_range(s: &str) -> RangeInclusive<u32> {
    let index = s.find("-").unwrap();
    let start: u32 = s[0 .. index].parse().unwrap();
    let end: u32 = s[index+1 ..].parse().unwrap();
    start ..= end
}

fn to_digits(i: u32) -> Vec<u8> {
    i.to_string().bytes().collect()
}

fn never_decreasing(digits: &[u8]) -> bool {
    for (a, b) in digits.iter().tuple_windows() {
        if b < a {
            return false;
        }
    }
    return true;
}

fn has_double(digits: &[u8]) -> bool {
    for (a, b) in digits.iter().tuple_windows() {
        if a == b {
            return true;
        }
    }
    return false;
}

fn has_isolated_double(digits: &[u8]) -> bool {
    for i in 0 .. digits.len() - 1 {
        let valid =
            digits[i] == digits[i + 1]
            && (i == 0 || digits[i - 1] != digits[i])
            && (i + 2 == digits.len() || digits[i + 2] != digits[i]);
        if valid {
            return true;
        }
    }
    return false;
}

pub fn part1() -> usize {
    let matcher = AndMatcher(&[
        &FunctionMatcher(never_decreasing),
        &FunctionMatcher(has_double),
    ]);
    let range = get_positive_range("152085-670283");
    range.filter(|&x| matcher.apply(to_digits(x).as_slice())).count()
}

pub fn part2() -> usize {
    let matcher = AndMatcher(&[
        &FunctionMatcher(never_decreasing),
        &FunctionMatcher(has_isolated_double),
    ]);
    let range = get_positive_range("152085-670283");
    range.filter(|&x| matcher.apply(to_digits(x).as_slice())).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_positive_range() {
        assert_eq!(get_positive_range("1234-5678"), 1234..=5678);
    }

    #[test]
    fn test_to_digits() {
        assert_eq!(to_digits(1234), vec![b'1', b'2', b'3', b'4']);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 1764);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 1196);
    }
}