#![allow(dead_code)]

use crate::{aocbail, regex, utils};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use utils::{AOCError, AOCResult};

lazy_static! {
    static ref MEM_REGEX: Regex = regex!(r"^mem\[([0-9]+)\] = ([0-9]+)$");
    static ref MASK_REGEX: Regex = regex!(r"^mask = ([X01]+)$");
}

type Mask = (u64, u64, u64);

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Instruction {
    Memory { address: u64, value: u64},
    Mask(Mask),
}

pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn load(filename: &str) -> AOCResult<Self> {
        let mut input = utils::get_input(filename);
        let mut instructions = Vec::new();
        for line in input {
            let instruction = if let Some(capture) = MASK_REGEX.captures_iter(&line).next() {
                Instruction::Mask(Program::mask_instruction(&capture[1]))
            } else {
                let mem_capture = MEM_REGEX.captures_iter(&line).next()?;
                Instruction::Memory{
                    address: mem_capture[1].parse::<u64>()?,
                    value: mem_capture[2].parse::<u64>()?,
                }
            };
            instructions.push(instruction);
        }

        Ok(Program {
            instructions,
        })
    }

    fn mask_instruction(mask_capture: &str) -> Mask {
        let mut mask_1 = 0;
        let mut mask_0 = 0;
        let mut mask_x = 0;
        for bit in mask_capture.chars() {
            mask_1 <<= 1;
            mask_0 <<= 1;
            mask_x <<= 1;
            if bit == '1' {
                mask_1 += 1;
            }
            if bit != '0' {
                mask_0 += 1;
            }
            if bit == 'X' {
                mask_x += 1;
            }
        } 
    
        (mask_0, mask_1, mask_x)
    }

    fn all_masks(mask_0: u64, mask_1: u64, mask_x: u64) -> Vec<(u64, u64)> {
        // APPARENTLY 0 masks by default means "don't do anything"
        // but we'll leverage it later to force overwritting some values to 0
        let mut masks: Vec<(u64, u64)> = vec![(!0, mask_1)];
        if mask_x != 0 {
            for x in 0..36 {
                let candidate = 1 << x;
                if mask_x & candidate != 0 {
                    for mask in masks.iter_mut() {
                        mask.1 += candidate;
                    }
                    masks.extend(
                        masks.iter().map(|mask| (
                            mask.0 - candidate,
                            mask.1 - candidate,
                    )).collect::<Vec<(u64, u64)>>());
                }
            }
        }
        masks
    }

    fn apply_mask(mask_0: u64, mask_1: u64, value: u64) -> u64 {
        (value & mask_0) | mask_1
    }

    pub fn part_1(&self) -> u64 {
        let mut memory = HashMap::new();
        let mut mask_0 = 0;
        let mut mask_1 = 1;
        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::Mask((m0, m1, _)) => {
                    mask_0 = *m0;
                    mask_1 = *m1;
                },
                Instruction::Memory{ address, value } => {
                    memory.insert(*address, Program::apply_mask(mask_0, mask_1, *value));
                }
            }
        }
        memory.iter().fold(0, |a, (k, v)| a + v)
    }

    pub fn part_2(&self) -> u64 {
        let mut memory = HashMap::new();
        let mut masks = Vec::new();
        for instruction in self.instructions.iter() {
            match instruction {
                Instruction::Mask((m0, m1, mx)) => {
                    masks = Program::all_masks(*m0, *m1, *mx)
                },
                Instruction::Memory{ address, value } => {
                    for mask in masks.iter() {
                        memory.insert( Program::apply_mask(mask.0, mask.1, *address), *value);
                    }
                }
            }
        }
        memory.iter().fold(0, |a, (k, v)| a + v)
    }
}

pub fn day14() {
    let program = Program::load("day14").unwrap();
    println!("docking_data part 1: {}", program.part_1());
    println!("docking_data part 2: {}", program.part_2());
}

#[test]
pub fn test_day14() {
    //let program = Program::load("test_day14").unwrap();
    //assert_eq!(program.part_1(), 165);

    let program = Program::load("test_day14_2").unwrap();
    assert_eq!(program.part_2(), 208)
}

#[test]
pub fn test_day14_apply() {
    let mask = Program::mask_instruction("000000000000000000000000000000X1001X");
    let all_masks =  Program::all_masks(mask.0, mask.1, mask.2);
    assert_eq!(
        all_masks.iter().map(|(m0, m1)| Program::apply_mask(*m0, *m1, 42)).collect::<Vec<u64>>(),
        vec![
            59, 58, 27, 26
        ]
    );
}
