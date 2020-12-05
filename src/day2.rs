use crate::{aocbail, utils, utils::AOCError};
use regex;

pub fn day2() {
    let password_input = utils::get_input("day2");
    let count = password_input.fold((0, 0), |a: (usize, usize), line: String| {
        (
            if p1(&line).unwrap() { a.0 + 1 } else { a.0 },
            if p2(&line).unwrap() { a.1 + 1 } else { a.1 },
        )
    });

    // 434
    println!("password philosophy part 1: {:?}", count.0);

    // 509
    println!("password philosophy part 2: {:?}", count.1);
}

pub fn p1(input: &str) -> Result<bool, AOCError> {
    let re = regex::Regex::new(r"^([\d]+)-([\d]+) ([a-z]): ([a-z]+)$")?;
    for cap in re.captures_iter(input) {
        let min: usize = cap[1].parse()?;
        let max: usize = cap[2].parse()?;
        let target_letter = cap[3].chars().next()?;
        let password = &cap[4];

        let count = password.chars().fold(
            0,
            |a: usize, c: char| if c == target_letter { a + 1 } else { a },
        );
        return Ok(count >= min && count <= max);
    }
    aocbail!("Didn't capture anything")
}

pub fn p2(input: &str) -> Result<bool, AOCError> {
    let re = regex::Regex::new(r"^([\d]+)-([\d]+) ([a-z]): ([a-z]+)$")?;
    for capture in re.captures_iter(input) {
        let i: usize = capture[1].parse()?;
        let j: usize = capture[2].parse()?;
        let target = capture[3].chars().next()?;
        let password: Vec<char> = capture[4].chars().collect();

        return Ok((password[i - 1] == target) ^ (password[j - 1] == target));
    }
    aocbail!("Didn't capture anything")
}

#[test]
fn basic_password_philosophy() {
    assert!(p1("1-3 a: abcde").unwrap());
    assert!(!p1("1-3 b: cdefg").unwrap());
    assert!(p1("2-9 c: ccccccccc").unwrap());

    assert!(p2("1-3 a: abcde").unwrap());
    assert!(!p2("1-3 b: cdefg").unwrap());
    assert!(!p2("2-9 c: ccccccccc").unwrap());
}
