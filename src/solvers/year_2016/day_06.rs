use std::vec;

use anyhow::Context;
use aoc_lib::iteration::ExtraIter;
use yuki::tuples::fst;
use crate::SolverResult;
use itertools::Itertools;

pub fn places(input: &str) -> impl Iterator<Item=vec::IntoIter<char>> {
    input
        .lines()
        .flat_map(|line| line.chars().enumerate())
        .into_group_map()
        .into_iter()
        .sorted_unstable_by_key(|&(index, _)| index)
        .map(|(_, chars)| chars.into_iter())
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let message = places(input)
        .map(ExtraIter::mode)
        .collect::<Option<String>>()
        .context("Column was empty")?;

    Ok(Box::new(message))
}

fn least_common(it: impl Iterator<Item=char>) -> Option<char> {
    it.counts()
        .into_iter()
        .min_by_key(|(_, count)| *count)
        .map(fst)
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let message = places(input)
        .map(least_common)
        .collect::<Option<String>>()
        .context("Column was empty")?;

    Ok(Box::new(message))
}