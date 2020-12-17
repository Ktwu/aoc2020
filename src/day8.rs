#![allow(dead_code)]

use crate::{aocbail, regex, utils};
use lazy_static::lazy_static;
use regex::Regex;
use utils::{AOCResult, AOCError};

lazy_static! {
    static ref CMD_REGEX: Regex = regex!(r"([a-z]+) \+?(-?[0-9]+)");
}

#[derive(Clone, Copy, Debug)]
pub enum CMD {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

#[derive(Default, Clone, Copy)]
pub struct ProgramState {
    ip: usize,
    acc: i32,
}

#[derive(Debug)]
pub struct Program {
    commands: Vec<CMD>,
}

impl ProgramState {
    pub fn apply(&mut self, cmd: CMD) {
        match cmd {
            CMD::Acc(v) => {
                self.ip += 1;
                self.acc += v;
            },
            CMD::Jmp(v) => {
                self.ip = (self.ip as i32 + v) as usize;
            },
            CMD::Nop(_) => {
                self.ip += 1;
            },
        };
    }
}

impl Program {
    pub fn load(input: impl Iterator<Item = String>) -> AOCResult<Program> {
        let mut commands = Vec::new();
        for line in input {
            let capture = CMD_REGEX.captures_iter(&line).next()?;
            let value = capture[2].parse::<i32>()?;
            commands.push(match &capture[1] {
                "acc" => CMD::Acc(value),
                "jmp" => CMD::Jmp(value),
                "nop" => CMD::Nop(value),
                _ => aocbail!("Unable to parse command")
            });
        }
        Ok(Program { commands })
    }

    pub fn run_until_halt(&self) -> i32 {
        let cmd_count = self.commands.len();
        let mut seen_instructions: Vec<bool> = Vec::with_capacity(cmd_count);
        seen_instructions.resize(cmd_count, false);
        self.run(ProgramState::default(), seen_instructions).acc
    }

    pub fn fix_and_run(&mut self) -> AOCResult<i32> {
        let cmd_count = self.commands.len();
        let mut seen_instructions: Vec<bool> = Vec::with_capacity(cmd_count);
        seen_instructions.resize(cmd_count, false);

        let mut state = ProgramState::default();
        while !seen_instructions[state.ip] {
            let final_state = self.run(state, seen_instructions.clone());
            if final_state.ip == cmd_count {
                return Ok(final_state.acc);
            }

            if self.toggle(state.ip) {
                let final_state = self.run(state, seen_instructions.clone());
                if final_state.ip == cmd_count {
                    return Ok(final_state.acc);
                }
                self.toggle(state.ip);
            }

            seen_instructions[state.ip] = true;
            state.apply(self.commands[state.ip]);
        }

        aocbail!("Unable to fix program to not terminate!");
    }

    fn toggle(&mut self, ip: usize) -> bool {
        self.commands[ip] = match self.commands[ip] {
            CMD::Acc(_) => { return false; },
            CMD::Jmp(v) => CMD::Nop(v),
            CMD::Nop(v) => CMD::Jmp(v),
        };
        true
    }

    fn run(&self, mut state: ProgramState, mut seen_instructions: Vec<bool>) -> ProgramState {
        let cmd_count = self.commands.len();
        while state.ip < cmd_count && !seen_instructions[state.ip] {
            seen_instructions[state.ip] = true;
            state.apply(self.commands[state.ip]);
        }
        state
    }
}

pub fn day8() {
    let input = utils::get_input("day8");
    let mut program = Program::load(input).unwrap();
    println!("handheld_halting part 1: {}", program.run_until_halt());
    println!("handheld_halting part 2: {}", program.fix_and_run().unwrap());
}

#[test]
pub fn basic_handheld_halting() {
    let input = utils::get_input("test_day8");
    let mut program = Program::load(input).unwrap();
    assert_eq!(
        program.run_until_halt(),
        5
    );

    assert_eq!(
        program.fix_and_run().unwrap(),
        8
    );
}
