#![allow(dead_code)]

use crate::{utils};
use utils::{AOCResult};

pub fn day6() {
    let input = utils::get_input("day6");
    println!("custom_customs part 1: {}", sum_answers(input).unwrap());

    let input = utils::get_input("day6");
    println!(
        "custom_customs part 2: {}",
        sum_intersection_answers(input).unwrap()
    );
}

pub fn sum_answers(input: impl Iterator<Item = String>) -> AOCResult<usize> {
    let mut answers: [u8; 26] = [0; 26];
    let mut answer_count: usize = 0;
    let mut sum = 0;
    for line in input {
        if line.len() == 0 {
            sum += answer_count;
            answer_count = 0;
            answers = [0; 26];
        } else {
            for answer in line.chars() {
                let i = answer as usize - 'a' as usize;
                answer_count += (answers[i] ^ 1) as usize;
                answers[i] = 1;
            }
        }
    }

    Ok(sum + answer_count)
}

pub fn sum_intersection_answers(input: impl Iterator<Item = String>) -> AOCResult<u32> {
    let mut answers: u32 = 0;
    let mut count = 0;
    let mut first_entry = true;
    for line in input {
        answers = if line.len() == 0 {
            count += bitfield_count(answers);
            first_entry = true;
            0
        } else {
            let temp_answers = answers_to_bitfield(&line);
            if first_entry {
                first_entry = false;
                temp_answers
            } else {
                answers & temp_answers
            }
        }
    }

    Ok(count + bitfield_count(answers))
}

pub fn answers_to_bitfield(answers: &str) -> u32 {
    answers
        .chars()
        .map(|a| 1 << (a as usize - 'a' as usize))
        .fold(0, |acc, answer| acc | answer)
}

pub fn bitfield_count(bitfield: u32) -> u32 {
    let mut count = 0;
    for i in 0..26 {
        count += (bitfield >> i) & 1;
    }
    count
}

#[test]
pub fn basic_custom_customs() {
    let input = utils::get_input("test_day6");
    assert_eq!(sum_answers(input).unwrap(), 11);

    let input = utils::get_input("test_day6");
    assert_eq!(sum_intersection_answers(input).unwrap(), 6);
}
