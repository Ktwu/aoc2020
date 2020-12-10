#![allow(dead_code)]

use crate::{regex, utils};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use utils::AOCResult;

lazy_static! {
    static ref BAG_REGEX: Regex = regex!(r"([0-9])* ?([a-z]+ [a-z]+) bags?");
}

pub type BagID = usize;

#[derive(Debug)]
pub struct BagRules {
    registry: HashMap<String, BagID>,
    contained_by: HashMap<BagID, HashMap<BagID, usize>>,
    contains: HashMap<BagID, HashMap<BagID, usize>>,
}

pub fn day7() {
    let input = utils::get_input("day7");
    let mut rules = BagRules::new();
    println!(
        "handy_haversacks part 1: {}",
        rules
            .process(input)
            .unwrap()
            .container_count("shiny gold")
            .unwrap()
    );

    println!(
        "handy_haversacks part 2: {}",
        rules.contained_count("shiny gold").unwrap()
    );
}

impl BagRules {
    pub fn new() -> BagRules {
        BagRules {
            registry: HashMap::new(),
            contained_by: HashMap::new(),
            contains: HashMap::new(),
        }
    }

    fn register(&mut self, bag_type: &str) -> BagID {
        let count = self.registry.len();
        *self.registry.entry(bag_type.to_string()).or_insert(count)
    }

    pub fn process(&mut self, input: impl Iterator<Item = String>) -> AOCResult<&mut Self> {
        for line in input {
            self.process_line(&line)?;
        }
        Ok(self)
    }

    pub fn process_line(&mut self, line: &str) -> AOCResult<&mut Self> {
        let mut captures = BAG_REGEX.captures_iter(line);
        let container_bag_type = self.register(&captures.next()?[2]);

        for capture in captures {
            let count = capture
                .get(1)
                .map_or(Ok(0), |m| m.as_str().parse::<usize>())?;
            let contained_bag_type = self.register(&capture[2]);
            self.contained_by
                .entry(contained_bag_type)
                .or_insert(HashMap::new())
                .insert(container_bag_type, count);
            self.contains
                .entry(container_bag_type)
                .or_insert(HashMap::new())
                .insert(contained_bag_type, count);
        }

        Ok(self)
    }

    fn gather_containers(&self, bag_id: &BagID, seen_types: &mut HashSet<BagID>) {
        if !self.contained_by.contains_key(bag_id) {
            return;
        }
        for containing_bag in self.contained_by[bag_id].iter() {
            let id = containing_bag.0;
            if seen_types.insert(*id) {
                self.gather_containers(id, seen_types);
            }
        }
    }

    fn gather_contained(&self, bag_id: &BagID, cached_count: &mut HashMap<BagID, usize>) -> usize {
        if !self.contains.contains_key(bag_id) {
            return 0;
        }
        if cached_count.contains_key(bag_id) {
            return cached_count[bag_id];
        }
        let contained_count = self.contains[bag_id].iter().fold(0, |a, bag| {
            a + bag.1 * (1 + self.gather_contained(bag.0, cached_count))
        });
        cached_count.insert(*bag_id, contained_count);
        contained_count
    }

    pub fn container_count(&self, bag_type: &str) -> AOCResult<usize> {
        let bag_id = self.registry.get(bag_type)?;
        let mut containers: HashSet<BagID> = HashSet::new();
        self.gather_containers(&bag_id, &mut containers);
        Ok(containers.len())
    }

    pub fn contained_count(&self, bag_type: &str) -> AOCResult<usize> {
        let bag_id = self.registry.get(bag_type)?;
        let mut cached_count: HashMap<BagID, usize> = HashMap::new();
        Ok(self.gather_contained(bag_id, &mut cached_count))
    }
}

#[test]
pub fn basic_handy_haversacks() {
    let input = utils::get_input("test_day7");
    let mut rules = BagRules::new();
    assert_eq!(
        rules
            .process(input)
            .unwrap()
            .container_count("shiny gold")
            .unwrap(),
        4
    );

    assert_eq!(rules.contained_count("shiny gold").unwrap(), 32);
}
