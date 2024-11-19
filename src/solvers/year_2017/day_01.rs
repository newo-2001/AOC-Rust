use aoc_lib::parsing::InvalidTokenError;
use crate::SolverResult;
use itertools::Itertools;

fn parse_captcha(input: &str) -> Result<Vec<u32>, InvalidTokenError<char>> {
    input.chars()
        .map(|c| char::to_digit(c, 10).ok_or(InvalidTokenError(c)))
        .try_collect()
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let captcha = parse_captcha(input)?;

    let result: u32 = captcha.into_iter()
        .circular_tuple_windows()
        .filter_map(|(a, b)| (a == b).then_some(a))
        .sum();

    Ok(Box::new(result))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let captcha = parse_captcha(input)?;

    let (start, end) = captcha.split_at(captcha.len() / 2);
    let result: u32 = captcha.iter()
        .zip(end.iter().chain(start.iter()))
        .filter_map(|(a, b)| (a == b).then_some(a))
        .sum();

    Ok(Box::new(result))
}