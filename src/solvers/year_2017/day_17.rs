use std::iter::once;

use aoc_lib::datastructures::GrowableRingBuffer;
use crate::SolverResult;

pub fn solve_part_1(input: &str) -> SolverResult {
    const CYCLES: usize = 2017;
    let step_size: usize = input.parse()?;

    let mut buffer: GrowableRingBuffer<usize> = once(0).collect();
    let mut index = 0;

    for i in 1..=2017 {
        index = buffer.wrap_index(index + step_size + 1);
        buffer.insert(index, i);
    }
    
    let final_index = buffer.iter()
        .position(|&value| value == CYCLES)
        .unwrap();
   
    Ok(Box::new(buffer[final_index + 1]))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let step_size: usize = input.parse()?;

    let mut index = 0;
    let mut zero_index = 0;
    let mut after_zero = 0;
    for i in 1..=50_000_000 {
        index = (index + step_size + 1) % i;
        
        let next_index = (index + 1) % (i + 1);
        if next_index <= zero_index {
            zero_index += 1;
        }

        if index == zero_index {
            after_zero = i;
        }
    }

    Ok(Box::new(after_zero))
}