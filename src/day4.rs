#![allow(dead_code)]

use crate::{aocbail, regex, utils};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumString};
use utils::{AOCError, AOCResult};

lazy_static! {
    static ref YEAR_REGEX: Regex = regex!(r"^(\d\d\d\d)$");
    static ref HEIGHT_REGEX: Regex = regex!(r"^(\d+)(in|cm)$");
    static ref HAIR_REGEX: Regex = regex!(r"^#[0-9a-f]{6}$");
    static ref EYE_REGEX: Regex = regex!(r"^amb|blu|brn|gry|grn|hzl|oth$");
    static ref COUNTRY_REGEX: Regex = regex!(r"^\d{9}$");
}

#[derive(EnumString, EnumCountMacro, Eq, PartialEq, Hash)]
#[strum(serialize_all = "snake_case")]
pub enum PassportKey {
    BYR,
    IYR,
    EYR,
    HGT,
    HCL,
    ECL,
    PID,
    CID,
}

impl PassportKey {
    fn is_valid_year(value: &str, min: usize, max: usize) -> bool {
        for capture in YEAR_REGEX.captures_iter(value) {
            if let Ok(year) = capture[1].parse::<usize>() {
                return year >= min && year <= max;
            }
        }
        return false;
    }

    pub fn is_valid(&self, value: &str) -> bool {
        match self {
            PassportKey::BYR => Self::is_valid_year(value, 1920, 2002),
            PassportKey::IYR => Self::is_valid_year(value, 2010, 2020),
            PassportKey::EYR => Self::is_valid_year(value, 2020, 2030),
            PassportKey::HGT => {
                for capture in HEIGHT_REGEX.captures_iter(value) {
                    if let Ok(height) = capture[1].parse::<usize>() {
                        return if &capture[2] == "cm" {
                            height >= 150 && height <= 193
                        } else {
                            height >= 59 && height <= 76
                        };
                    }
                }
                return false;
            }
            PassportKey::HCL => HAIR_REGEX.is_match(value),
            PassportKey::ECL => EYE_REGEX.is_match(value),
            PassportKey::PID => COUNTRY_REGEX.is_match(value),
            PassportKey::CID => true,
        }
    }
}

pub struct Passport {
    valid_data_only: bool,
    entries: HashSet<PassportKey>,
}

impl Passport {
    pub fn new(valid_data_only: bool) -> Self {
        Passport {
            valid_data_only,
            entries: HashSet::new(),
        }
    }

    pub fn parse_entry(&mut self, keyvalue: &str) -> AOCResult<()> {
        let re = regex::Regex::new(r"^([a-z]+):([a-z0-9#]+)$")?;
        for capture in re.captures_iter(keyvalue) {
            let key: PassportKey = PassportKey::from_str(&capture[1])?;
            if !self.valid_data_only || key.is_valid(&capture[2]) {
                self.entries.insert(key);
            }
            return Ok(());
        }
        aocbail!("Unable to parse passport key from {}", keyvalue)
    }

    pub fn parse_entries(&mut self, input: &str) -> AOCResult<()> {
        input
            .split(" ")
            .map(|entry: &str| self.parse_entry(entry))
            .collect::<AOCResult<Vec<()>>>()
            .and(Ok(()))
    }

    pub fn is_valid(&self) -> bool {
        let mut expected_key_count = PassportKey::COUNT;
        if !self.entries.contains(&PassportKey::CID) {
            expected_key_count -= 1;
        }
        self.entries.len() == expected_key_count
    }
}

pub fn day4() {
    // 230
    let mut input = utils::get_input("day4");
    println!(
        "Passport processing part 1: {}",
        num_valid_passports(input, false).unwrap()
    );

    // 156
    input = utils::get_input("day4");
    println!(
        "Passport processing part 2: {}",
        num_valid_passports(input, true).unwrap()
    );
}

pub fn num_valid_passports(
    mut input: impl Iterator<Item = String>,
    validate_data: bool,
) -> AOCResult<usize> {
    let mut count: usize = 0;
    let mut passport = Passport::new(validate_data);
    while let Some(line) = input.next() {
        if line.len() == 0 {
            count += if passport.is_valid() { 1 } else { 0 };
            passport = Passport::new(validate_data);
        } else {
            passport.parse_entries(&line)?;
        }
    }

    count += if passport.is_valid() { 1 } else { 0 };
    Ok(count)
}

#[test]
pub fn basic_passport_processing() {
    let test_input = utils::get_input("test_day4");
    assert_eq!(num_valid_passports(test_input, false).unwrap(), 2);

    assert!(PassportKey::BYR.is_valid("2002"));
    assert!(!PassportKey::BYR.is_valid("2003"));

    assert!(PassportKey::HGT.is_valid("60in"));
    assert!(PassportKey::HGT.is_valid("190cm"));
    assert!(!PassportKey::HGT.is_valid("190in"));
    assert!(!PassportKey::HGT.is_valid("190"));

    assert!(PassportKey::HCL.is_valid("#123abc"));
    assert!(!PassportKey::HCL.is_valid("#123abz"));
    assert!(!PassportKey::HCL.is_valid("123abc"));

    assert!(PassportKey::ECL.is_valid("brn"));
    assert!(!PassportKey::ECL.is_valid("wat"));

    assert!(PassportKey::PID.is_valid("000000001"));
    assert!(!PassportKey::PID.is_valid("0123456789"));
}
