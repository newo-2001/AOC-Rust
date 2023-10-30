use std::vec;

use anyhow::anyhow;
use aoc_lib::iteration::ExtraIter;
use aoc_runner_api::SolverResult;
use composing::compose_fn;
use itertools::{Itertools, enumerate};
use tupletools::{snd, fst};

pub fn places(input: &str) -> impl Iterator<Item=vec::IntoIter<char>> {
    input.lines()
        .flat_map(compose_fn!(str::chars => enumerate))
        .into_group_map()
        .into_iter()
        .sorted_unstable_by_key(|&(index, _)| index)
        .map(compose_fn!(snd => Vec::into_iter))
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let message = places(input)
        .map(ExtraIter::mode)
        .collect::<Option<String>>()
        .ok_or(anyhow!("Column was empty"))?;

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
        .ok_or(anyhow!("Column was empty"))?;

    Ok(Box::new(message))
}