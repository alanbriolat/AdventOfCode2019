use std::str::FromStr;
use std::num::ParseIntError;
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

#[derive(Clone,Debug)]
struct PasswordIterator {
    start: [u8; 6],
    end: [u8; 6],
    current: [u8; 6],
    done: bool,
}

impl PasswordIterator {
    fn new(start: u32, end: u32) -> PasswordIterator {
        assert!(start <= end);
        let mut out = PasswordIterator {
            start: Default::default(),
            end: Default::default(),
            current: Default::default(),
            done: false,
        };
        Self::create_digit_array(start, &mut out.start);
        Self::create_digit_array(end, &mut out.end);
        out.current = out.start;
        out
    }

    fn create_digit_array(x: u32, dest: &mut [u8; 6]) {
        for (i, n) in (0 .. dest.len()).rev().enumerate() {
            dest[i] = (x / (10_u32.pow(n as u32)) % 10) as u8;
        }
    }
}

impl Iterator for PasswordIterator {
    type Item = [u8; 6];

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        if self.current == self.end {
            self.done = true;
        }
        let out = self.current;
        let mut increment: u8 = 1;
        let mut index: usize = 6;
        while increment > 0 && index > 0 {
            index -= 1;
            self.current[index] += increment;
            if self.current[index] == 10 {
                self.current[index] = 0;
                increment = 1;
            } else {
                increment = 0;
            }
        }
        return Some(out);
    }
}

impl FromStr for PasswordIterator {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let index = s.find("-").unwrap();
        let start: u32 = s[0 .. index].parse()?;
        let end: u32 = s[index+1 ..].parse()?;
        Ok(PasswordIterator::new(start, end))
    }
}


pub fn part1() -> usize {
    let matcher = AndMatcher(&[
        &FunctionMatcher(never_decreasing),
        &FunctionMatcher(has_double),
    ]);
    let iterator = "152085-670283".parse::<PasswordIterator>().unwrap();
    iterator.filter(|x| matcher.apply(x)).count()
}

pub fn part2() -> usize {
    let matcher = AndMatcher(&[
        &FunctionMatcher(never_decreasing),
        &FunctionMatcher(has_isolated_double),
    ]);
    let iterator = "152085-670283".parse::<PasswordIterator>().unwrap();
    iterator.filter(|x| matcher.apply(x)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_iterator_to_digits() {
        let mut dest = [0_u8; 6];
        PasswordIterator::create_digit_array(123456, &mut dest);
        assert_eq!(dest, [1, 2, 3, 4, 5, 6]);
        PasswordIterator::create_digit_array(123, &mut dest);
        assert_eq!(dest, [0, 0, 0, 1, 2, 3]);
    }

    #[test]
    fn test_password_iterator() {
        let mut pi = "1234-5678".parse::<PasswordIterator>().unwrap();
        assert_eq!(pi.start, [0, 0, 1, 2, 3, 4]);
        assert_eq!(pi.end, [0, 0, 5, 6, 7, 8]);
        assert_eq!(pi.current, [0, 0, 1, 2, 3, 4]);
        assert_eq!(pi.next(), Some([0, 0, 1, 2, 3, 4]));
        assert_eq!(pi.start, [0, 0, 1, 2, 3, 4]);
        assert_eq!(pi.end, [0, 0, 5, 6, 7, 8]);
        assert_eq!(pi.current, [0, 0, 1, 2, 3, 5]);
        pi.current = [0, 0, 5, 6, 7, 7];
        assert_eq!(pi.next(), Some([0, 0, 5, 6, 7, 7]));
        assert_eq!(pi.next(), Some([0, 0, 5, 6, 7, 8]));
        assert_eq!(pi.next(), None);
        assert_eq!(pi.next(), None);
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