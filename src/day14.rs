use std::cmp::min;
use std::collections::{HashMap};
use std::num::ParseIntError;
use std::str::FromStr;
use crate::util::read_lines;

#[derive(Clone,Debug)]
struct Component {
    name: String,
    amount: i64,
}

impl FromStr for Component {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(" ").collect();
        Ok(Component {
            name: parts[1].to_string(),
            amount: parts[0].parse()?,
        })
    }
}

#[derive(Clone,Debug)]
struct Reaction {
    inputs: Vec<Component>,
    output: Component,
}

impl FromStr for Reaction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" => ").collect();
        Ok(Reaction {
            inputs: parts[0].split(", ").map(|s| s.parse().unwrap()).collect(),
            output: parts[1].parse().unwrap(),
        })
    }
}

#[derive(Debug)]
struct Factory {
    reactions: HashMap<String, Reaction>,
    produced: HashMap<String, i64>,
    surplus: HashMap<String, i64>,
}

impl Factory {
    fn from_data_file(filename: &str) -> Factory {
        let reactions: HashMap<String, Reaction> = read_lines(filename)
            .iter()
            .map(|x| x.parse::<Reaction>().unwrap())
            .map(|x| (x.output.name.clone(), x))
            .collect();
        let mut f = Factory {
            reactions,
            produced: HashMap::new(),
            surplus: HashMap::new(),
        };
        for name in f.reactions.keys() {
            f.produced.insert(name.clone(), 0);
            f.surplus.insert(name.clone(), 0);
        }
        return f;
    }

    /// Consume at most `amount` of `name` from surplus only, returning the amount still required
    fn consume_surplus(&mut self, name: &String, amount: i64) -> i64 {
        let surplus = self.surplus.entry(name.clone()).or_insert(0);
        let consumed = min(amount, *surplus);
        *surplus -= consumed;
        return amount - consumed;
    }

    /// Consume `amount` of `name`, producing more if necessary
    fn consume(&mut self, name: &String, amount: i64) {
        let amount = self.consume_surplus(name, amount);
        if amount != 0 {
            self.produce(name, amount);
        }
    }

    /// Produce `amount` of `name`, saving any surplus amount
    fn produce(&mut self, name: &String, amount: i64) {
        if let Some(reaction) = self.reactions.get(name) {
            let reaction = reaction.clone();
            let count = (amount as f64 / reaction.output.amount as f64).ceil() as i64;
            for input in reaction.inputs.iter() {
                self.consume(&input.name, input.amount * count);
            }
            let produced = count * reaction.output.amount;
            *self.produced.entry(name.clone()).or_insert(0) += produced;
            *self.surplus.entry(name.clone()).or_insert(0) += produced - amount;
        } else {
            // ORE doesn't have a reaction to produce it, so just assume it exists
            *self.produced.entry(name.clone()).or_insert(0) += amount;
        }
    }
}

fn ore_required(filename: &str, fuel: i64) -> i64 {
    let mut factory = Factory::from_data_file(filename);
    factory.produce(&"FUEL".to_string(), fuel);
    *factory.produced.get("ORE").unwrap()
}

fn max_fuel_production(filename: &str) -> i64 {
    let target: i64 = 1_000_000_000_000;
    // target divided by amount for 1 FUEL is a good estimate for the minimum
    let mut current = target / ore_required(filename, 1);
    let mut increment = current;
    // Do a binary search between minimum estimate and 2x that estimate to find the actual answer
    while increment > 0 {
        while ore_required(filename, current + increment) <= target {
            current += increment;
        }
        increment /= 2;
    }
    current
}

pub fn part1() -> i64 {
    ore_required("day14_input.txt", 1)
}

pub fn part2() -> i64 {
    max_fuel_production("day14_input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ore_required_examples() {
        assert_eq!(ore_required("day14_example1.txt", 1), 31);
        assert_eq!(ore_required("day14_example2.txt", 1), 165);
        assert_eq!(ore_required("day14_example3.txt", 1), 13312);
        assert_eq!(ore_required("day14_example4.txt", 1), 180697);
        assert_eq!(ore_required("day14_example5.txt", 1), 2210736);
    }

    #[test]
    fn test_max_fuel_production() {
        assert_eq!(max_fuel_production("day14_example3.txt"), 82892753);
        assert_eq!(max_fuel_production("day14_example4.txt"), 5586022);
        assert_eq!(max_fuel_production("day14_example5.txt"), 460664);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 374457);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), 3568888);
    }
}
