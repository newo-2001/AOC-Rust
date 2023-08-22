use std::num::ParseIntError;
use aoc_lib::parsing::parse_lines;
use aoc_runner_api::SolverResult;
use itertools::Itertools;

type Present := Vec<u32>;

fn parse_present(str: &str) -> Result<Present, ParseIntError> {
    str.split('x')
        .map(str::parse::<u32>)
        .collect::<Result<Vec<u32>, ParseIntError>>()
}

fn required_wrapping_paper(present: &Present) -> u32 {
    let areas: Vec<u32> = present.iter()
        .combinations(2)
        .map(|sides| sides.into_iter().fold(1, |acc, x| acc * x))
        .collect();

    let min_area: &u32 = areas.iter().min().unwrap_or(&0);
    let total_area: u32 = areas.iter().sum::<u32>() * 2;
    min_area + total_area
}

fn required_ribbon(present: &Present) -> u32 {
    let smallest_perimeter: u32 = present.iter()
        .combinations(2)
        .map(|sides| sides.into_iter().sum::<u32>() * 2)
        .min().unwrap_or(0);

    let volume: u32 = present.iter()
        .fold(1, |acc, x| acc * x);
    
    smallest_perimeter + volume
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let wrapping_paper: u32 = parse_lines(parse_present, input)?
        .iter()
        .map(required_wrapping_paper)
        .sum();

    Ok(Box::new(wrapping_paper))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let ribbon: u32 = parse_lines(parse_present, input)?
        .iter()
        .map(required_ribbon)
        .sum();

    Ok(Box::new(ribbon))
}