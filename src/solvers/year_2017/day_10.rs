use aoc_lib::errors::NoSolution;
use crate::SolverResult;
use itertools::Itertools;

use super::knot_hash;

pub fn solve_part_1(input: &str) -> SolverResult {
    let lengths: Box<[u8]> = input.split(',').map(str::parse).try_collect()?;
    let hash: u16 = knot_hash::rounds(&lengths, 1)
        .next_chunk::<2>()
        .map_err(|_| NoSolution)?
        .map(u16::from)
        .into_iter()
        .product();

    Ok(Box::new(hash))
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve_part_2(input: &str) -> SolverResult {
    let hash = knot_hash::hash(input);
    Ok(Box::new(hash))
}