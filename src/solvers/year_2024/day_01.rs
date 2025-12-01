use itertools::Itertools;
use nom::{character::complete::{space1, u32}, sequence::separated_pair, Parser};
use yuki::parsing::{combinators::lines, run_parser, ParserExt, ParsingResult};

use crate::SolverResult;

fn parse_list(input: &str) -> ParsingResult<'_, (Vec<u32>, Vec<u32>)> {
    lines(separated_pair(u32, space1, u32))
        .map(|lists| lists.into_iter().unzip())
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
    let (left, right) = parse_list.run(input)?;

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