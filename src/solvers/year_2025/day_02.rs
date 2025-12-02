use std::{ops::RangeInclusive};

use itertools::Itertools;
use nom::{Parser, character::complete::{char, u64}, multi::separated_list0, sequence::separated_pair};
use yuki::parsing::{ParsingResult, run_parser};

use crate::SolverResult;

fn parse_range(input: &str) -> ParsingResult<'_, RangeInclusive<u64>> {
    separated_pair(u64, char('-'), u64)
        .map(|(lower, upper)| lower..=upper)
        .parse(input)
}

fn parse_ranges(input: &str) -> ParsingResult<'_, Vec<RangeInclusive<u64>>> {
    separated_list0(char(','), parse_range)
        .parse(input)
}

fn is_self_duplicated(id: u64) -> bool {
    let num_digits = (id as f32).log10().ceil() as u32;
    let mask = 10_u64.pow(num_digits / 2);

    id / mask == id % mask
}

fn is_repeated_pattern(id: u64) -> bool {
    let id = id.to_string();
    let id = id.as_bytes();

    (1..=id.len() / 2)
        .any(|stride| id
            .len()
            .div_exact(stride)
            .is_some_and(|chunks| (0..chunks)
                .map(|chunk_idx| &id[stride * chunk_idx..stride * (chunk_idx + 1)])
                .all_equal()
            )
        )
}

pub fn solve<F>(input: &str, predicate: F) -> SolverResult where
    F: Fn(u64) -> bool
{
    let ranges = run_parser(parse_ranges, input)?;

    let ids_sum: u64 = ranges
        .into_iter()
        .flat_map(|range| range
            .filter(|&id| predicate(id))
        )
        .sum();

    Ok(Box::new(ids_sum))
}

pub fn solve_part_1(input: &str) -> SolverResult {
    solve(input, is_self_duplicated)
}

pub fn solve_part_2(input: &str) -> SolverResult {
    solve(input, is_repeated_pattern)
}