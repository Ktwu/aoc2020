#![allow(dead_code)]

use crate::{regex, utils};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;
use utils::{AOCError, AOCResult};

lazy_static! {
    static ref SCHEMA_REGEX: Regex = regex!(r"^([a-z ]+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)$");
}

type TicketRange = (RangeInclusive<u32>, RangeInclusive<u32>);
type TicketSchema = HashMap<String, TicketRange>;
type Ticket = Vec<u32>;

fn load_ticket_values(line: &str) -> AOCResult<Ticket> {
    line.split(",")
        .map(|value| value.parse::<u32>())
        .collect::<Result<Ticket, _>>()
        .map_err(|e| AOCError::from(e))
}

pub fn load_schema(input: &mut impl Iterator<Item = String>) -> AOCResult<TicketSchema> {
    let mut schema: TicketSchema = HashMap::new();
    for line in input {
        let mut captures = SCHEMA_REGEX.captures_iter(&line);
        if let Some(capture) = captures.next() {
            schema.insert(
                capture[1].to_owned(),
                (
                    (capture[2].parse::<u32>()?..=capture[3].parse::<u32>()?),
                    (capture[4].parse::<u32>()?..=capture[5].parse::<u32>()?),
                ),
            );
        } else {
            break;
        }
    }

    Ok(schema)
}

pub fn part1(filename: &str) -> u32 {
    let mut input = utils::get_input(filename);
    let valid_ranges: Vec<TicketRange> = load_schema(&mut input).unwrap().into_values().collect();

    input.skip(4).fold(0, |acc, line| {
        load_ticket_values(&line)
            .unwrap()
            .iter()
            .fold(acc, |acc, value| {
                if !valid_ranges
                    .iter()
                    .any(|(r1, r2)| r1.contains(value) || r2.contains(value))
                {
                    acc + value
                } else {
                    acc
                }
            })
    })
}

pub fn part2(filename: &str) -> u64 {
    let mut input = utils::get_input(filename);
    let ticket_schema = load_schema(&mut input).unwrap();
    let base_options = ticket_schema.keys().collect::<HashSet<_>>();
    let valid_ranges = ticket_schema.values().collect::<Vec<_>>();

    let mut solved_count = 0;
    let mut solved_options = vec![None; base_options.len()];

    let mut ticket_options = Vec::with_capacity(base_options.len());
    for _i in 0..base_options.len() {
        ticket_options.push(base_options.clone());
    }

    input.next();
    let my_ticket = load_ticket_values(&input.next().unwrap()).unwrap();

    let mut solved_queue = Vec::new();
    'outer: for line in input.skip(2) {
        for (i, value) in load_ticket_values(&line).unwrap().iter().enumerate() {
            if solved_options[i].is_some() {
                continue;
            }

            if !valid_ranges
                .iter()
                .any(|(r1, r2)| r1.contains(value) || r2.contains(value)) {
                continue 'outer;
            }

            let options = ticket_options.get_mut(i).unwrap();
            options.drain_filter(|field| {
                let (r1, r2) = &ticket_schema[*field];
                !r1.contains(value) && !r2.contains(value)
            }).for_each(drop);

            if options.len() == 1 {
                let field = options.iter().next().unwrap().to_owned();
                solved_queue.push((i, field));
            }

            while let Some((i, field)) = solved_queue.pop() {
                solved_count += 1;
                for (j, options) in ticket_options.iter_mut().enumerate() {
                    options.remove(field);
                    if options.len() == 1 {
                        solved_queue.push((j, options.iter().next().unwrap().to_owned()));
                    }
                }
                solved_options[i] = Some(field);
            }

            if solved_count == solved_options.len() {
                break 'outer;
            }
        }
    }

    println!("Solved for {:?}", solved_options);

    my_ticket.iter().enumerate().fold(1, |a, (i, value)|
        a * match solved_options[i] {
            Some(x) if x.starts_with("departure") => *value,
            _ => 1,
        } as u64
    )
}

pub fn day16() {
    println!("ticket translation part 1: {}", part1("day16"));
    println!("ticket translation part 2: {}", part2("day16"));
}

#[test]
fn test_day16() {
    assert_eq!(part1("test_day16"), 71);
}
