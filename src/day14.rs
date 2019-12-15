use std::cmp::min;
use std::collections::{HashMap};
use std::num::ParseIntError;
use std::str::FromStr;
use crate::util::read_lines;

#[derive(Clone,Debug)]
struct Component {
    name: String,
    amount: usize,
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
    produced: HashMap<String, usize>,
    surplus: HashMap<String, usize>,
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
    fn consume_surplus(&mut self, name: &String, amount: usize) -> usize {
        let surplus = self.surplus.entry(name.clone()).or_insert(0);
        let consumed = min(amount, *surplus);
        *surplus -= consumed;
        return amount - consumed;
    }

    /// Consume `amount` of `name`, producing more if necessary
    fn consume(&mut self, name: &String, amount: usize) {
        let amount = self.consume_surplus(name, amount);
        if amount != 0 {
            self.produce(name, amount);
        }
    }

    /// Produce `amount` of `name`, saving any surplus amount
    fn produce(&mut self, name: &String, amount: usize) {
        if let Some(reaction) = self.reactions.get(name) {
            let reaction = reaction.clone();
            let count = (amount as f32 / reaction.output.amount as f32).ceil() as usize;
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

fn ore_required(filename: &str) -> usize {
    let mut factory = Factory::from_data_file(filename);
    factory.produce(&"FUEL".to_string(), 1);
    *factory.produced.get("ORE").unwrap()
}

pub fn part1() -> usize {
    ore_required("day14_input.txt")
}

pub fn part2() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ore_required_examples() {
        assert_eq!(ore_required("day14_example1.txt"), 31);
        assert_eq!(ore_required("day14_example2.txt"), 165);
        assert_eq!(ore_required("day14_example3.txt"), 13312);
        assert_eq!(ore_required("day14_example4.txt"), 180697);
        assert_eq!(ore_required("day14_example5.txt"), 2210736);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 374457);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(), unimplemented!());
    }
}
