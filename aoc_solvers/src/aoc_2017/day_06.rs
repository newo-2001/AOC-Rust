use std::{collections::{HashMap, hash_map::OccupiedError}, num::ParseIntError};

use aoc_runner_api::SolverResult;
use itertools::Itertools;

struct Cycle {
    index: u32,
    length: u32
}

// This is disgusting, I will look for a declarative approach later
fn first_cycle(mut banks: Vec<u32>) -> Cycle {
    let mut seen: HashMap<Vec<u32>, u32> = HashMap::new();
    let mut index = 0;

    loop {
        if let Err(OccupiedError { entry, .. }) = seen.try_insert(banks.clone(), index) {
            break Cycle { index, length: index - entry.get() }
        }

        // `position_max()` returns the last instance in case of tie, the problem wants the first
        // To solve this we reverse the iterator and subtract the index from `banks.len() - 1`
        let mut bank = banks.len() - 1 - banks.iter().rev().position_max().unwrap();
        let bank_blocks = banks.get_mut(bank).unwrap();
        let blocks = *bank_blocks;
        *bank_blocks = 0;
        index += 1;

        for _ in 0..blocks {
            bank = (bank + 1) % banks.len();
            *banks.get_mut(bank).unwrap() += 1;
        }
    }
}

fn parse_banks(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.split_whitespace()
        .map(str::parse)
        .collect()
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let banks = parse_banks(input)?;
    Ok(Box::new(first_cycle(banks).index))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let banks = parse_banks(input)?;
    Ok(Box::new(first_cycle(banks).length))
}