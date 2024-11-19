use std::{collections::hash_map::OccupiedError, num::ParseIntError};

use ahash::{HashMap, HashMapExt};
use aoc_lib::datastructures::GrowableRingBuffer;
use crate::SolverResult;
use itertools::Itertools;

struct Cycle {
    index: u32,
    length: u32
}

// This is disgusting, I will look for a declarative approach later
fn first_cycle(mut banks: GrowableRingBuffer<u32>) -> Cycle {
    let mut seen: HashMap<GrowableRingBuffer<u32>, u32> = HashMap::new();
    let mut index = 0;

    loop {
        if let Err(OccupiedError { entry, .. }) = seen.try_insert(banks.clone(), index) {
            break Cycle { index, length: index - entry.get() }
        }

        // `position_max()` returns the last instance in case of tie, the problem wants the first
        // To solve this we reverse the iterator and subtract the index from `banks.len() - 1`
        let max_bank = banks.len() - 1 - banks.iter().rev().position_max().unwrap();
        let bank_blocks = banks.get_mut(max_bank).unwrap();
        let blocks = *bank_blocks;
        *bank_blocks = 0;
        index += 1;

        for block in 1..=blocks {
            *banks.get_mut(max_bank + block as usize).unwrap() += 1;
        }
    }
}

fn parse_banks(input: &str) -> Result<GrowableRingBuffer<u32>, ParseIntError> {
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