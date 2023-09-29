use std::u8;

use aoc_runner_api::SolverResult;
use composing::compose_fn;

use super::knot_hash;

pub fn solve_part_1(input: &str) -> SolverResult {
    let ones: u32 = (0..128)
        .map(|n| format!("{input}-{n}"))
        .map(compose_fn!(knot_hash::hash => hex::decode))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flat_map(|bytes| bytes.into_iter().map(u8::count_ones))
        .sum();

    Ok(Box::new(ones))
}