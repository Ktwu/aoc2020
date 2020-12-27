#![allow(dead_code)]

use crate::{aocbail, regex, utils};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use utils::{AOCError, AOCResult};

lazy_static! {
    static ref WORD_RE: Regex = regex!(r"([a-z]+)");
}

pub struct AllergenInfo {
    ingredients: HashMap<String, usize>,
    candidates_per_allergen: HashMap<String, HashSet<String>>,
}

impl AllergenInfo {
    pub fn load(filename: &str) -> AllergenInfo {
        let input = utils::get_input(filename);
        let mut info = AllergenInfo {
            ingredients: HashMap::new(),
            candidates_per_allergen: HashMap::new(),
        };

        input
            .map(|line| {
                let mut local_ingredients = HashSet::new();
                let mut local_allergens = HashSet::new();
                let mut processing_allergens = false;
                for capture in WORD_RE.captures_iter(&line) {
                    let word = &capture[1];
                    if word == "contains" {
                        processing_allergens = true;
                        continue;
                    }
                    if processing_allergens {
                        local_allergens.insert(word.to_owned());
                    } else {
                        local_ingredients.insert(word.to_owned());
                    }
                }

                (local_ingredients, local_allergens)
            })
            .fold(info, |mut i, (ingredients, allergens)| {
                for ingredient in ingredients.iter() {
                    *i.ingredients.entry(ingredient.clone()).or_insert(0) += 1;
                }
                for allergen in allergens.into_iter() {
                    if !i.candidates_per_allergen.contains_key(&allergen) {
                        i.candidates_per_allergen
                            .insert(allergen, ingredients.clone());
                    } else {
                        i.candidates_per_allergen
                            .get_mut(&allergen)
                            .unwrap()
                            .drain_filter(|v| !ingredients.contains(v)).for_each(drop);
                    }
                }

                i
            })
    }

    pub fn part1(&self) -> usize {
        self.non_allergens().iter().fold(0, |a, i|
            a + self.ingredients[*i]
        )
    }

    fn non_allergens(&self) -> HashSet<&String> {
        let maybe_allergen = self.candidates_per_allergen
            .iter()
            .flat_map(|(_, ingredients)| ingredients)
            .collect::<HashSet<_>>();

        self.ingredients.iter().filter_map(|(i, _)|
            if !maybe_allergen.contains(i) {
                Some(i)
            } else {
                None
            }
        ).collect()
    }

    pub fn solve(&mut self) -> HashMap<String, String> {
        let mut allergens = HashMap::new();
        while self.candidates_per_allergen.len() != 0 {
            for (_, candidates) in self.candidates_per_allergen.iter_mut() {
                candidates.drain_filter(|v| allergens.contains_key(v)).for_each(drop);
            }
            for (allergen, candidates) in self.candidates_per_allergen.drain_filter(|_, candidates| candidates.len() == 1) {
                allergens.insert(candidates.into_iter().next().unwrap(), allergen);
            }
        }

        allergens
    }

    pub fn part2(&mut self) -> String {
        let allergens = self.solve();
        let mut danger_list = allergens.into_iter().map(|(ingredient, allergen)| (ingredient, allergen)).collect::<Vec<_>>();
        danger_list.sort_by(|this, that| {
            this.1.partial_cmp(&that.1).unwrap()
        });
        danger_list.into_iter().map(|(i,_)| i).collect::<Vec<_>>().join(",")
    }
}

pub fn day21() {
    let mut info = AllergenInfo::load("day21");
    println!("allergen_assessment part 1: {:?}", info.part1());
    println!("allergen_assessment part 2: {:?}", info.part2());
}

#[cfg(test)]
mod tests {
    use crate::day21::*;

    #[test]
    pub fn test_day21() {
        let mut info = AllergenInfo::load("test_day21");
        assert_eq!(info.part1(), 5,);
        assert_eq!(info.part2(), "mxmxvkd,sqjhc,fvjkl");
    }
}
