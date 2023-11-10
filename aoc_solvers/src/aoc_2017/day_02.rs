use std::num::ParseIntError;

use aoc_lib::{parsing::parse_lines, errors::NoSolution};
use aoc_runner_api::SolverResult;
use itertools::{Itertools, MinMaxResult};
use num::Integer;

fn parse_row(input: &str) -> Result<Vec<u32>, ParseIntError> {
    input.split_whitespace()
        .map(str::parse)
        .try_collect()
}

fn checksum_row(row: impl IntoIterator<Item=u32>) -> u32 {
    match row.into_iter().minmax() {
        MinMaxResult::NoElements | MinMaxResult::OneElement(_) => 0,
        MinMaxResult::MinMax(min, max) => max - min
    }
}

fn divisors<I>(row: I) -> Option<u32>
    where I: IntoIterator<Item=u32>,
          I::IntoIter: Clone
{
    row.into_iter()
        .tuple_combinations()
        .find_map(|(a, b)| {
            a.divides(&b).then_some(a / b)
                .or(b.divides(&a).then_some(b / a))
        })
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let result: u32 = parse_lines(parse_row, input)?
        .into_iter()
        .map(checksum_row)
        .sum();

    Ok(Box::new(result))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let result: u32 = parse_lines(parse_row, input)?
        .into_iter()
        .map(divisors)
        .collect::<Option<Vec<_>>>()
        .ok_or(NoSolution)?
        .into_iter()
        .sum();

    Ok(Box::new(result))
}