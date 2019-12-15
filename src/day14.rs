use std::str::FromStr;
use std::num::ParseIntError;
use crate::util::read_lines;
use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;

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

#[derive(Debug)]
struct Reaction {
    inputs: Vec<Component>,
    output: Component,
}

impl FromStr for Reaction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(" => ").collect();
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
        let mut f = Factory {
            reactions: read_lines(filename).iter().map(|x| (x.clone(), x.parse().unwrap())).collect(),
            produced: HashMap::new(),
            surplus: HashMap::new(),
        };
        for name in f.reactions.keys() {
            f.produced.insert(name.clone(), 0);
            f.surplus.insert(name.clone(), 0);
        }
        return f;
    }

    fn produce(&mut self, component: &Component) {
        if component.name == "ORE" {
            // We don't need to do anything to produce any amount of ORE, just do it
            *self.produced.entry("ORE".to_string()).or_insert(0) += component.amount;
        } else {
            if *self.surplus.get(&component.name).unwrap() >= component.amount {
                // Already got enough of the chemical, we're done here
                *self.surplus.get_mut(&component.name).unwrap() -= component.amount;
            } else {
                // Not enough, let's produce some more! Let's get the reaction required to produce this chemical
                let reaction = self.reactions.get(component.name.as_str()).unwrap();
                // Figure out how many more we need to make
                let required = component.amount - *self.surplus.get(&component.name).unwrap();
                *self.surplus.get_mut(&component.name).unwrap() = 0;
                // Figure out many productions of the reaction that equates to
                let productions = (required as f32 / reaction.output.amount as f32).ceil() as usize;
                let amount = productions * reaction.output.amount;
                // Make sure we have enough of each prerequisite
                for input in reaction.inputs.iter().cloned() {
                    self.produce(&Component{name: input.name.clone(), amount: productions * input.amount});
                }
                // Now produce the chemical
                *self.produced.get_mut(&component.name).unwrap() += amount;
                *self.surplus.get_mut(&component.name).unwrap() += amount - required;
            }
        }
    }
}

fn ore_required(filename: &str) -> usize {
    let reactions: Vec<Reaction> =
        read_lines(filename)
            .iter()
            .map(|x| x.parse().unwrap())
            .collect();
    let mut dependencies: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut reaction_map: HashMap<&str, &Reaction> = HashMap::new();
    for reaction in reactions.iter() {
        let prev = reaction_map.insert(&reaction.output.name, reaction);
        assert!(prev.is_none());    // assert there's only one reaction to produce each chemical
        let inputs = dependencies.entry(reaction.output.name.as_str()).or_insert(HashSet::new());
        for input in reaction.inputs.iter() {
            inputs.insert(&input.name);
        }
    }
    println!("{:#?}", reaction_map);
    println!("{:#?}", dependencies);

    let mut ordering: Vec<&str> = dependencies.keys().cloned().collect();
    ordering.push("ORE");
    ordering.sort_by(|&a, &b| {
        let result = if dependencies.get(b).map(|x| x.contains(a)).unwrap_or(false) {
            Ordering::Less
        } else if dependencies.get(a).map(|x| x.contains(b)).unwrap_or(false) {
            Ordering::Greater
        } else {
            Ordering::Equal
        };
        println!("compared {} vs. {}, {:?}", a, b, result);
        result
    });
    println!("ordering: {:?}", ordering);

    let mut ore_count: usize = 0;
    let mut process: VecDeque<Component> = VecDeque::new();
    process.push_back(Component{name: "FUEL".to_string(), amount: 1});
    while let Some(next) = process.pop_front() {
        if next.name == "ORE" {
            ore_count += next.amount;
        } else {
            let reaction = *reaction_map.get(next.name.as_str()).unwrap();
            let amount = ((next.amount as f32) / (reaction.output.amount as f32)).ceil() as usize;
            for input in reaction.inputs.iter() {
                process.push_back(Component{name: input.name.clone(), amount: amount * input.amount});
            }
        }
    }
    ore_count
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
    fn test_ore_required_examples() {
        assert_eq!(ore_required("day14_example1.txt"), 31);
        assert_eq!(ore_required("day14_example2.txt"), 165);
        assert_eq!(ore_required("day14_example3.txt"), 13312);
        assert_eq!(ore_required("day14_example4.txt"), 180697);
        assert_eq!(ore_required("day14_example5.txt"), 2210736);
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
