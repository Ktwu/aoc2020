#![allow(dead_code)]

use crate::{aocbail, utils};
use utils::{AOCResult, AOCError};

pub fn day9() {
    let input = utils::get_input("day9");
    let bad_value = find_bad_value(input, 25).unwrap();
    println!("encoding_error part 1: {}", bad_value.1);

    let input = utils::get_input("day9");
    println!("encoding_error part 2: {}", find_sum(input, bad_value.0).unwrap());
}

pub fn find_bad_value(input: impl Iterator<Item = String>, cache_size: usize) -> AOCResult<(usize, i32)> {
    let mut cache: Vec<i32> = Vec::with_capacity(cache_size);
    'outer: for (i, line) in input.enumerate() {
        let value = line.parse::<i32>()?;
        if i < cache_size {
            cache.push(value);
            continue;
        } else {
            for j in 0..cache.len() {
                for k in 1..cache.len() {
                    if cache[j] + cache[k] == value {
                        cache.remove(0);
                        cache.push(value);
                        continue 'outer;
                    }
                }
            }
        }

        return Ok((i, value));
    }

    aocbail!("Unable to find illegal entry");
}

pub fn find_sum(mut input: impl Iterator<Item = String>, target_id: usize) -> AOCResult<i32> {
    let mut values: Vec<i32> = Vec::new();
    for _ in 0..=target_id {
        let value = input.next()?;
        values.push(value.parse::<i32>()?);
    }

    let target = values[target_id];
    let mut end = 0;
    let mut running_sum = values[0];
    'outer: for start in 0..target_id {
        while running_sum != target {
            if running_sum > target {
                running_sum -= values[start];
                while running_sum > target {
                    running_sum -= values[end];
                    end -= 1;
                }
                continue 'outer;
            }

            end += 1;
            running_sum += values[end];
        }

        let mut min = values[start];
        let mut max = values[start];
        for i in start..=end {
            min = std::cmp::min(min, values[i]);
            max = std::cmp::max(max, values[i]);
        }
        return Ok(min + max);
    }

    aocbail!("Something horrible happened");
}

#[test]
pub fn encoding_error() {
    let input = utils::get_input("test_day9");
    let bad_value = find_bad_value(input, 5).unwrap();
    assert_eq!(
        bad_value.1,
        127
    );

    let input = utils::get_input("test_day9");
    assert_eq!(
        find_sum(input, bad_value.0).unwrap(),
        62
    );
}
