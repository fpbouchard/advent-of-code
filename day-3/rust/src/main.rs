use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
struct Journey {
    rucksacks: Vec<Rucksack>,
}

impl FromStr for Journey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Journey {
            rucksacks: s
                .lines()
                .map(|line| Rucksack::from_str(line).unwrap())
                .collect(),
        })
    }
}

#[derive(Debug)]
struct Rucksack {
    compartments: [Compartment; 2],
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len() / 2;
        Ok(Rucksack {
            compartments: [
                Compartment {
                    item_set: s[0..len].chars().collect(),
                },
                Compartment {
                    item_set: s[len..].chars().collect(),
                },
            ],
        })
    }
}

impl Rucksack {
    fn diff_compartments(&self) -> &char {
        let shared_chars: Vec<&char> = self.compartments[0]
            .item_set
            .intersection(&self.compartments[1].item_set)
            .collect();
        assert!(shared_chars.len() == 1);
        shared_chars[0]
    }
}

#[derive(Debug)]
struct Compartment {
    item_set: HashSet<char>,
}

fn char_priority(c: &char) -> u32 {
    match *c {
        'a'..='z' => *c as u32 - 'a' as u32 + 1,
        'A'..='Z' => *c as u32 - 'A' as u32 + 27,
        _ => panic!("invalid char"),
    }
}

fn load_data() -> String {
    return std::fs::read_to_string("./input.txt").unwrap();
}

fn main() {
    let journey = Journey::from_str(&load_data()).unwrap();
    let diffs = journey
        .rucksacks
        .iter()
        .map(|r| r.diff_compartments())
        .collect::<Vec<&char>>();
    println!("journey diffs: {:?}", diffs);
    let diff_priorities: Vec<u32> = diffs.iter().map(|c| char_priority(c)).collect();
    println!("diff priorities: {:?}", diff_priorities);
    let diff_priority: u32 = diff_priorities.iter().sum();
    println!("diff priority: {:?}", diff_priority);
}
