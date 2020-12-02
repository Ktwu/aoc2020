use regex;
use crate::utils::AOCError;

pub fn p1(input: &str) -> Result<bool, AOCError> {
    let re = regex::Regex::new(r"^([\d]+)-([\d]+) ([a-z]): ([a-z]+)$")?;
    for cap in re.captures_iter(input) {
        let min: usize = cap[1].parse()?;
        let max: usize = cap[2].parse()?;
        let target_letter = cap[3].chars().next().ok_or(AOCError {})?;
        let password = &cap[4];

        let count = password.chars().fold(
            0,
            |a: usize, c: char| if c == target_letter { a + 1 } else { a },
        );
        return Ok(count >= min && count <= max);
    }
    return Err(AOCError {});
}

pub fn p2(input: &str) -> Result<bool, AOCError> {
    let re = regex::Regex::new(r"^([\d]+)-([\d]+) ([a-z]): ([a-z]+)$")?;
    for cap in re.captures_iter(input) {
        let i: usize = cap[1].parse()?;
        let j: usize = cap[2].parse()?;
        let target = cap[3].chars().next().ok_or(AOCError {})?;
        let password: Vec<char> = cap[4].chars().collect();

        return Ok((password[i-1] == target) ^ (password[j-1] == target));
    }
    return Err(AOCError {});
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
