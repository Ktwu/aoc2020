#![allow(dead_code)]

use crate::{regex, utils};
use utils::{AOCResult};
use strum_macros::EnumString;
use std::str::FromStr;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref COMMAND_REGEX: Regex = regex!("^([A-Z])([0-9]+)$");
}

#[derive(EnumString)]
pub enum Command {
    N, S, E, W, L, R, F
}

type CommandList = Vec<(Command, i32)>;

pub fn load(filename: &str) -> AOCResult<CommandList> {
    let input = utils::get_input(filename);
    let mut commands: CommandList = Vec::new();
    for line in input {
        let capture = COMMAND_REGEX.captures_iter(&line).next()?;
        let command = Command::from_str(&capture[1])?;
        let value = capture[2].parse::<i32>()?;
        commands.push((command, value));
    }
    Ok(commands)
}

pub fn day12() {
    let commands = load("day12").unwrap();
    println!("rain_risk part 1: {:?}", manhattan_distance(&commands));
    println!("rain_risk part 2: {:?}", waypoint_manhattan(&commands));
}

pub fn waypoint_manhattan(commands: &CommandList) -> i32 {
    let mut waypoint: (i32, i32) = (10, 1);
    let mut ship: (i32, i32) = (0, 0);

    for (command, value) in commands.iter() {
        match command {
            Command::N => waypoint.1 += value,
            Command::S => waypoint.1 -= value,
            Command::E => waypoint.0 += value,
            Command::W => waypoint.0 -= value,
            Command::L => {
                let steps = value / 90;
                for _ in 0..steps {
                    waypoint = (
                        -1 * waypoint.1,
                        waypoint.0,
                    );
                }
            },
            Command::R => {
                let steps = value / 90;
                for _ in 0..steps {
                    waypoint = (
                        waypoint.1,
                        -1 * waypoint.0,
                    );
                }
            },
            Command::F => {
                ship.0 += waypoint.0 * value;
                ship.1 += waypoint.1 * value;
            }
        }
        //println!("ship: {:?}, waypoint: {:?}", ship, waypoint);
    }

    ship.0.abs() + ship.1.abs()
}

pub fn manhattan_distance(commands: &CommandList) -> i32 {
    let directions = vec![
        (Command::E, (1, 0)),
        (Command::S, (0, -1)),
        (Command::W, (-1, 0)),
        (Command::N, (0, 1))];

    let mut direction: usize = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for (command, value) in commands.iter() {
        match command {
            Command::N => y += value,
            Command::S => y -= value,
            Command::E => x += value,
            Command::W => x -= value,
            Command::L => direction = (direction + 3 * (value / 90) as usize) % 4,
            Command::R => direction = (direction + (value / 90) as usize) % 4,
            Command::F => {
                x += directions[direction].1.0 * value;
                y += directions[direction].1.1 * value;
            }
        }
    }

    x.abs() + y.abs()
}

#[test]
pub fn test_day12() {
    let input = load("test_day12").unwrap();
    assert_eq!(
        manhattan_distance(&input),
        25,
    );
    assert_eq!(
        waypoint_manhattan(&input),
        286,
    );
}
