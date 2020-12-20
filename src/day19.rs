#![allow(dead_code)]

use crate::{aocbail, regex, utils};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use utils::{AOCError, AOCResult};

lazy_static! {
    static ref RULE_REGEX: Regex = regex!(r"^([0-9]+): ");
}

#[derive(Debug)]
pub enum Rule {
    And(Vec<Rule>),
    Or(Vec<Rule>),
    Id(usize),
    A,
    B,
}

#[derive(Debug)]
pub struct Grammar {
    rules: HashMap<usize, Rule>,
}

impl Rule {
    pub fn load(input: &str) -> AOCResult<Rule> {
        Ok(match input.trim() {
            "\"a\"" => Rule::A,
            "\"b\"" => Rule::B,
            s if s.contains("|") => {
                let subrules = s.split('|').collect::<Vec<_>>();
                Rule::Or(vec![Rule::load(subrules[0])?, Rule::load(subrules[1])?])
            }
            s if s.contains(" ") => Rule::And(
                s.split(' ')
                    .map(|subrule| Rule::load(subrule))
                    .collect::<AOCResult<Vec<_>>>()?,
            ),
            s => Rule::Id(s.parse::<usize>()?),
        })
    }
}

impl Grammar {
    pub fn load(input: &mut impl Iterator<Item = String>) -> AOCResult<Grammar> {
        let mut rules = HashMap::new();
        while let Some(line) = input.next() {
            if line.len() == 0 {
                break;
            }
            let mut rule_parts = line.split(":");
            rules.insert(
                rule_parts.next()?.parse::<usize>()?,
                Rule::load(rule_parts.next()?)?,
            );
        }

        Ok(Grammar { rules })
    }

    pub fn eats(&self, input: &str) -> bool {
        match self.eat(&self.rules[&0], input) {
            Ok(leftover) => leftover.len() == 0,
            Err(_) => false,
        }
    }

    pub fn eats_loop(&self, input: &str) -> bool {
        (1..input.len()).any(|i| {
            let n1 = self.repeat_eat(input.get(0..i).unwrap(), 42);
            if n1 >= 2 {
                let n2 = self.repeat_eat(input.get(i..input.len()).unwrap(), 31);
                return n2 >= 1 && n1 > n2;
            }
            false
        })
    }

    fn repeat_eat(&self, mut input: &str, id: usize) -> i32 {
        let mut count = 0;
        while let Ok(submatch) = self.eat(&self.rules[&id], input) {
            count += 1;
            if submatch.len() == 0 {
                return count;
            }
            input = submatch;
        }
        0
    }

    fn eat<'a>(&self, rule: &Rule, mut input: &'a str) -> AOCResult<&'a str> {
        Ok(match rule {
            Rule::A if input.starts_with('a') => input.get(1..)?,
            Rule::B if input.starts_with('b') => input.get(1..)?,
            Rule::Id(id) => self.eat(&self.rules[id], input)?,
            Rule::And(rules) => {
                for rule in rules.iter() {
                    input = self.eat(rule, input)?
                }
                input
            }
            Rule::Or(rules) => {
                for rule in rules.iter() {
                    if let Ok(eatd) = self.eat(rule, input) {
                        return Ok(eatd);
                    }
                }
                aocbail!("Unable to match Or() rule")
            }
            _ => aocbail!("{:?} does not match {}", rule, input),
        })
    }
}

pub fn day19() {
    let mut input = utils::get_input("day19");
    let grammar = Grammar::load(&mut input).unwrap();
    let (s1, s2) = input.fold((0, 0), |(a1, a2), line| {
        (
            a1 + if grammar.eats(&line) { 1 } else { 0 },
            a2 + if grammar.eats_loop(&line) { 1 } else { 0 },
        )
    });
    println!("monster_messages part 1: {}", s1);
    println!("monster_messages part 2: {}", s2);
}

#[cfg(test)]
mod tests {
    use crate::day19::*;

    #[test]
    pub fn test_day19() {
        let mut test_input = utils::get_input("test_day19");
        let grammar = Grammar::load(&mut test_input).unwrap();

        assert!(grammar.eats("ababbb"));
        assert!(grammar.eats("abbbab"));
        assert!(!grammar.eats("bababa"));
        assert!(!grammar.eats("aaabbb"));
        assert!(!grammar.eats("aaaabbb"));
    }

    #[test]
    pub fn test_day19_loop() {
        let mut input = utils::get_input("test_day19_2");
        let grammar = Grammar::load(&mut input).unwrap();

        assert!(grammar.eats_loop("babbbbaabbbbbabbbbbbaabaaabaaa"));

        assert_eq!(
            input.fold(0, |a, line| a + if grammar.eats_loop(&line) {
                1
            } else {
                0
            }),
            12
        );
    }
}
