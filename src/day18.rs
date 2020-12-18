#![allow(dead_code)]

use crate::{aocbail, utils};
use std::iter::Peekable;
use utils::{AOCError, AOCResult};

pub fn re_weird_parse(
    tokens: &mut Peekable<impl Iterator<Item = char>>,
    precedence: bool,
    greedy: bool,
) -> AOCResult<u64> {
    let mut left_expr = match tokens.next()? {
        '(' => {
            let expr = re_weird_parse(tokens, precedence, false)?;
            if tokens.next()? != ')' {
                aocbail!("Expected closing parens");
            }
            expr
        }
        x => x.to_digit(10)? as u64,
    };

    while let Some(token) = tokens.peek() {
        if greedy || *token == ')' {
            break;
        }

        let next_add = tokens.next()? == '+';
        let right_expr = re_weird_parse(tokens, precedence, !precedence || next_add)?;

        if next_add {
            left_expr += right_expr;
        } else {
            left_expr *= right_expr;
        }
    }

    Ok(left_expr)
}

fn weird_parse(input: &str, precedence: bool) -> AOCResult<u64> {
    re_weird_parse(
        &mut input.chars().filter(|c| *c != ' ').peekable(),
        precedence,
        false,
    )
}

pub fn day18() {
    let (s1, s2) = utils::get_input("day18").fold((0, 0), |(a1, a2), line| {
        (
            a1 + weird_parse(&line, false).unwrap(),
            a2 + weird_parse(&line, true).unwrap(),
        )
    });

    println!("operation_order part 1: {:?}", s1);
    println!("operation_order part 2: {:?}", s2);
}

#[cfg(test)]
mod tests {
    use crate::day18::*;

    #[test]
    pub fn test_day18() {
        assert_eq!(weird_parse("2 * 3 + (4 * 5)", false).unwrap(), 26);

        assert_eq!(
            weird_parse("1 + (2 * 3) + (4 * (5 + 6))", true).unwrap(),
            51
        );
        assert_eq!(weird_parse("2 * 3 + (4 * 5)", true).unwrap(), 46);
        assert_eq!(
            weird_parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", true).unwrap(),
            23340
        );
    }
}
