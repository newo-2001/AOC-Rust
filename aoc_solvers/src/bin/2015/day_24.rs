use std::{error::Error, num::ParseIntError, iter::IntoIterator};

use aoc_lib::io::read_puzzle_input;
use itertools::Itertools;
use tupletools::fst;

fn parse_items(str: &str) -> Result<Vec<u32>, ParseIntError> {
    str.lines()
        .map(str::parse::<u32>)
        .collect::<Result<Vec<u32>, ParseIntError>>()
}

fn target_weight(items: &[u32], groups: u32) -> u32 {
    items.iter().sum::<u32>() / groups
}

fn is_valid_group<'a>(items: impl IntoIterator<Item= &'a u32>, target: u32) -> bool {
    items.into_iter().sum::<u32>() == target
}

fn quantum_entanglement<'a >(group: impl IntoIterator<Item = &'a u32>) -> u64 {
    group.into_iter()
        .map(|&x| x as u64)
        .product::<u64>()
}

fn optimal_group(items: Vec<u32>, groups: u32) -> Result<Vec<u32>, &'static str> {
    let target = target_weight(&items, groups);

    items.into_iter()
        .powerset()
        .skip_while(|group| !is_valid_group(group, target))
        .tuple_windows()
        .take_while(|(current, next)| current.len() == next.len())
        .filter(|(group, _)| is_valid_group(group, target))
        .map(fst)
        .min_by_key(|group| quantum_entanglement(group))
        .ok_or("No valid configurations found")
}

fn entanglement_optimal_group(items: Vec<u32>, groups: u32) -> Result<(), &'static str> {
    let group = optimal_group(items, groups)?;
    let entanglement = quantum_entanglement(&group);
    println!("The quantum entanglement of the optimal configurations when using {} groups is: {}", groups, entanglement);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents = read_puzzle_input(2015, 24)?;
    let items = parse_items(&contents)?;

    entanglement_optimal_group(items.clone(), 3)?;
    entanglement_optimal_group(items, 4)?;

    Ok(())
}