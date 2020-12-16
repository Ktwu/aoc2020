#![allow(dead_code)]

use crate::{aocbail, regex, utils};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use utils::{AOCError, AOCResult};

lazy_static! {
    static ref MEM_REGEX: Regex = regex!(r"^mem\[([0-9]+)\] = ([0-9]+)$");
    static ref MASK_REGEX: Regex = regex!(r"^mask = ([X01]+)$");
}

pub fn day15() {
    println!("rambunctious_recitation part 1: {:?}", recitation(vec![12,20,0,6,1,17,7], 2020));
    println!("rambunctious_recitation part 1: {:?}", recitation(vec![12,20,0,6,1,17,7], 30000000));
}

pub fn recitation(input: Vec<u64>, max_turn: u64) -> u64 {
    let mut cache: HashMap<u64, u64> = HashMap::new();

    let mut turn: u64 = 0;
    let mut last_number_turn: u64 = 0;
    let mut last_number: u64 = 0;
    for i in input.iter() {
        turn += 1;
        last_number = *i;
        cache.insert(last_number, turn);
    }

    while turn < max_turn {
        last_number = if last_number_turn > 0 {
            turn - last_number_turn
        } else {
            0
        };
        last_number_turn = *cache.get(&last_number).unwrap_or(&0);
        turn += 1;
        cache.insert(last_number, turn);
    }

    last_number
}

#[test]
fn test_day15() {
    assert_eq!(
        recitation(vec![0, 3, 6], 2020), 436
    );

    assert_eq!(
        recitation(vec![3, 1, 2], 2020), 1836
    );

    assert_eq!(
        recitation(vec![3, 1, 2], 30000000), 362
    );
}