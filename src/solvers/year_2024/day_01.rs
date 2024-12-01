use itertools::Itertools;
use nom::{character::complete::{line_ending, space1, u32}, combinator::map, multi::separated_list0, sequence::separated_pair, Parser};
use yuki::parsing::{run_parser, ParsingResult};

use crate::SolverResult;

fn parse_list(input: &str) -> ParsingResult<(Vec<u32>, Vec<u32>)> {
    map(
        separated_list0(
            line_ending,
            separated_pair(u32, space1, u32),
        ),
        |lists| lists.into_iter().unzip() 
    )
    .parse(input)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let (mut left, mut right) = run_parser(parse_list, input)?;
    left.sort_unstable();
    right.sort_unstable();

    let total_distance: u32 = left
        .into_iter()
        .zip(right)
        .map(|(left, right)| left.abs_diff(right))
        .sum();

    Ok(Box::new(total_distance))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let (left, right) = run_parser(parse_list, input)?;

    let frequencies = right
        .into_iter()
        .counts();

    let similarity: usize = left
        .into_iter()
        .map(|num| frequencies
            .get(&num)
            .unwrap_or(&0) * num as usize
        )
        .sum();

    Ok(Box::new(similarity))
}