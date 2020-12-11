#![allow(dead_code)]

use crate::{aocbail, utils};
use utils::{AOCResult, AOCError};

pub fn day_template() {
    let input = utils::get_input("day10");
    println!("part 1: {:?}", input);
}

#[test]
pub fn test_template() {
    let input = utils::get_input("test_day10");
    assert_eq!(
        127,
        127
    );
}
