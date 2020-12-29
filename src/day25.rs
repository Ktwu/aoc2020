#![allow(dead_code)]

use crate::{aocbail, utils};
use utils::{AOCResult, AOCError};

pub fn solve_loop_size(subject: u64, target: u64) -> u64 {
    let mut loop_size = 0;
    let mut value = 1;
    while value != target {
        loop_size += 1;
        value = (value * subject) % 20201227;
    }
    loop_size
}

pub fn transform(subject: u64, loop_size: u64) -> u64 {
    let mut value = 1;
    for i in 0..loop_size {
        value = (value * subject) % 20201227;
    }
    value
}

pub fn day25() {
    let card_loop = solve_loop_size(7, 13233401);
    let door_loop = solve_loop_size(7, 6552760);

    println!("combo_breaker part 1: {:?} {:?}", transform(6552760, card_loop), transform(13233401, door_loop));
}

#[cfg(test)]
mod tests {
    use crate::day25::*;

    #[test]
    pub fn test_day25() {
        assert_eq!(
            solve_loop_size(7, 5764801),
            8
        );
        assert_eq!(
            transform(17807724, 8),
            14897079
        );

        assert_eq!(
            solve_loop_size(7, 17807724),
            11
        );
        assert_eq!(
            transform(5764801, 11),
            14897079
        );
    }
}
