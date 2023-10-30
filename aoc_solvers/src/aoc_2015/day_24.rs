use std::{num::ParseIntError, iter::IntoIterator};

use anyhow::Result;
use aoc_lib::NoSolutionError;
use aoc_runner_api::SolverResult;
use itertools::Itertools;
use tupletools::fst;

fn parse_items(str: &str) -> Result<Vec<u32>, ParseIntError> {
    str.lines()
        .map(str::parse::<u32>)
        .try_collect()
}

fn target_weight(items: &[u32], groups: u32) -> u32 {
    items.iter().sum::<u32>() / groups
}

fn is_valid_group<'a>(items: impl IntoIterator<Item= &'a u32>, target: u32) -> bool {
    items.into_iter().sum::<u32>() == target
}

fn quantum_entanglement<'a >(group: impl IntoIterator<Item = &'a u32>) -> u64 {
    group.into_iter()
        .map(|&x| u64::from(x))
        .product::<u64>()
}

fn optimal_group(items: Vec<u32>, groups: u32) -> Result<Vec<u32>, NoSolutionError> {
    let target = target_weight(&items, groups);

    items.into_iter()
        .powerset()
        .skip_while(|group| !is_valid_group(group, target))
        .tuple_windows()
        .take_while(|(current, next)| current.len() == next.len())
        .filter(|(group, _)| is_valid_group(group, target))
        .map(fst)
        .min_by_key(|group| quantum_entanglement(group))
        .ok_or(NoSolutionError)
}

fn entanglement_optimal_group(input: &str, groups: u32) -> Result<u64> {
    let items = parse_items(input)?;
    let group = optimal_group(items, groups)?;
    Ok(quantum_entanglement(&group))
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let entanglement = entanglement_optimal_group(input, 3)?;
    Ok(Box::new(entanglement))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let entanglement = entanglement_optimal_group(input, 4)?;
    Ok(Box::new(entanglement))
}