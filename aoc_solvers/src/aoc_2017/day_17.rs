use std::iter::once;

use aoc_lib::datastructures::growable_ring_buffer::GrowableRingBuffer;
use aoc_runner_api::SolverResult;

fn spin(cycles: usize, step_size: usize) -> usize {
    let mut buffer: GrowableRingBuffer<usize> = once(0).collect();
    let mut index = 0;

    for i in 1..=cycles {
        index = buffer.wrap_index(index + step_size + 1);
        buffer.insert(index, i);
    }
    
    let final_index = buffer.iter()
        .position(|&value| value == cycles)
        .unwrap();
    
    buffer[final_index + 1]
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let step_size: usize = input.parse()?;
    Ok(Box::new(spin(2017, step_size)))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let step_size: usize = input.parse()?;
    Ok(Box::new(spin(50_000_000, step_size)))
}