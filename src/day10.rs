#![allow(dead_code)]

use crate::{utils};
use std::collections::HashMap;

pub fn day10() {
    let adaptors = load("day10");
    println!("adaptor_array part 1: {:?}", count_deltas(&adaptors));
    println!("adaptor_array part 2: {:?}", count_arrangements(&adaptors));
}

pub fn load(filename: &str) -> Vec<u32> {
    let input = utils::get_input(filename);
    let mut adaptors = input.map(|i| i.parse::<u32>().unwrap() ).collect::<Vec<u32>>();
    adaptors.insert(0, 0);
    adaptors.sort_unstable();
    adaptors.push(adaptors[adaptors.len()-1] + 3);
    adaptors 
}

pub fn count_deltas(adaptors: &Vec<u32>) -> u32 {
    let mut count_1volt = 0;
    let mut count_3volt = 0;
    for i in 1..adaptors.len() {
        match adaptors[i] - adaptors[i-1] {
            1 => { count_1volt += 1; },
            3 => { count_3volt += 1; },
            _ => {}
        };
    }
    count_3volt * count_1volt
}

pub fn count_arrangements(adaptors: &Vec<u32>) -> u64 {
    let mut cache = HashMap::new();
    re_count_arrangements(adaptors, 0, &mut cache)
}

fn re_count_arrangements(adaptors: &Vec<u32>, from_index: usize, cache: &mut HashMap<usize, u64>) -> u64 {
    let mut count = 0;
    if from_index == adaptors.len() - 1 {
        return 1;
    }

    for next_index in from_index+1..adaptors.len() {
        if adaptors[next_index] - adaptors[from_index] <= 3 {
            count += if let Some(value) = cache.get(&next_index) {
                *value
            } else {
                let value = re_count_arrangements(adaptors, next_index, cache);
                cache.insert(next_index, value);
                value
            };
        } else {
            break;
        }
    }
    count
}

#[test]
pub fn test_day10() {
    let input = load("test_day10");
    assert_eq!(
        count_deltas(&input),
        35,
    );

    assert_eq!(
        count_arrangements(&input),
        8,
    );
}
