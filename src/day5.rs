#![allow(dead_code)]

use crate::{aocbail, utils};
use utils::{AOCError, AOCResult};

pub struct SeatRange {
    min: usize,
    max: usize,
}

impl SeatRange {
    pub fn narrow(&mut self, take_upper: bool) -> Option<usize> {
        let delta = (self.max - self.min + 1) / 2;
        if take_upper {
            self.min += delta;
        } else {
            self.max -= delta;
        }
        if self.min == self.max {
            Some(self.min)
        } else {
            None
        }
    }
}

pub fn day5() {
    let input = utils::get_input("day5");
    println!(
        "binary_boarding part 1: {}", find_hipri_seat(input).unwrap()
    );

    let input = utils::get_input("day5");
    println!(
        "binary_boarding part 2: {}", find_your_seat(input).unwrap()
    );
}

pub fn find_your_seat(input: impl Iterator<Item=String>) -> AOCResult<usize> {
    const seat_count: usize = 8*128;
    let mut seats: [u8; seat_count] = [0; seat_count];
    let mut min: usize = seat_count-1;
    for line in input {
        let seat = find_seat(&line)?;
        min = std::cmp::min(min, seat);
        seats[seat] = 1;
    }

    for i in (min+1)..seats.len() {
        if seats[i] == 0 {
            return Ok(i);
        }
    }

    aocbail!("Unable to find seat!");
}

pub fn find_hipri_seat(input: impl Iterator<Item=String>) -> AOCResult<usize> {
    let mut hipri_seat = 0;
    for line in input {
        hipri_seat = std::cmp::max(hipri_seat, find_seat(&line)?);
    }
    Ok(hipri_seat)
}

pub fn find_seat(boarding_pass: &str) -> AOCResult<usize> {
    let mut row_range = SeatRange{min: 0, max: 127};
    let mut column_range = SeatRange{min: 0, max: 7};
    let mut row: Option<usize> = None;
    let mut column: Option<usize> = None;

    for position in boarding_pass.chars() {
        match position {
            'F' => {
                row = row_range.narrow(false);
            },
            'B' => {
                row = row_range.narrow(true);
            },
            'L' => {
                column = column_range.narrow(false);
            },
            'R' => {
                column = column_range.narrow(true);
            },
            _ => {
                aocbail!("Illegal character");
            }
        }
    }
    Ok(row? * 8 + column?)
}


#[test]
pub fn basic_binary_boarding() {
    assert_eq!(find_seat("BFFFBBFRRR").unwrap(), 567);
    assert_eq!(find_seat("FFFBBBFRRR").unwrap(), 119);
    assert_eq!(find_seat("BBFFBBFRLL").unwrap(), 820);
}
