#![allow(dead_code)]

use crate::{aocbail, utils};
use utils::{AOCResult, AOCError};

pub fn day13() {
    let input = utils::get_input("day13");
    println!("shuttle_search part 1: {:?}", find_schedule(input).unwrap());
    let input = utils::get_input("day13");
    println!("shuttle_search part 2: {:?}", chinese_remainder_theorem(input).unwrap());
}

pub fn find_schedule(mut input: impl Iterator<Item = String>) -> AOCResult<u32> {
    let departure = input.next()?.parse::<u32>()?;
    let raw_schedules = input.next()?;

    let mut min_id = 0;
    let mut min_wait_time = 0;

    for raw_schedule in raw_schedules.split(",") {
        if raw_schedule == "x" {
            continue;
        }
        let schedule_id = raw_schedule.parse::<u32>()?;
        let wait_time = ((departure / schedule_id) + 1) * schedule_id - departure;

        if min_id == 0 || wait_time < min_wait_time {
            min_id = schedule_id;
            min_wait_time = wait_time;
        }
    }

    Ok(min_id * min_wait_time)
}

pub fn mod_inverse(a: u64, m: u64) -> AOCResult<u64> {
    for x in 1..m {
        if (a * x) % m == 1 {
            return Ok(x);
        }
    }
    println!("failed for {} {}", a, m);
    aocbail!("mod_inverse failed!");
}

pub fn chinese_remainder_theorem(mut input: impl Iterator<Item = String>) -> AOCResult<u64> {
    let _ = input.next()?;
    let raw_schedules = input.next()?;
    let mut buses = Vec::new();
    let mut big_n = 1;
    for (i, raw_schedule) in raw_schedules.split(",").enumerate() {
        if raw_schedule == "x" {
            continue;
        }
        let bus = raw_schedule.parse::<i64>()?;
        let mut offset = bus - i as i64;
        while offset < 0  {
            offset += bus;
        }
        big_n *= bus as u64;
        buses.push((offset as u64, bus as u64));
    }

    let mut sum = 0;
    for (a, n) in buses.iter() {
        let coprime = big_n / n;
        sum += a * coprime * mod_inverse(coprime, *n)?;
    }

    Ok(sum % big_n)
}

#[test]
pub fn test_day13() {
    let input = utils::get_input("test_day13");
    assert_eq!(find_schedule(input).unwrap(), 295);

    assert_eq!(mod_inverse(10, 17).unwrap(), 12);
    let input = utils::get_input("test_day13_2");
    assert_eq!(chinese_remainder_theorem(input).unwrap(), 3417);
}
