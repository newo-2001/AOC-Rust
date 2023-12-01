use aoc_runner_api::SolverResult;
use itertools::Itertools;
use tupletools::snd;
use anyhow::{anyhow, Result};

fn calibration(digits: impl IntoIterator<Item=u32>) -> Result<u32> {
    let mut digits = digits.into_iter();
    let first = digits.next().ok_or(anyhow!("A line in the input does not contain a digit"))?;
    let last = digits.last().unwrap_or(first);
    Ok(first * 10 + last)
}

fn total_calibration<I: IntoIterator<Item=u32>>(digits: impl IntoIterator<Item=I>) -> Result<u32> {
    let total = digits.into_iter()
        .map(calibration)
        .collect::<Result<Vec<u32>>>()?
        .into_iter()
        .sum();

    Ok(total)
}

fn numeric_digits(input: &str) -> impl Iterator<Item=u32> + '_ {
    input.chars().filter_map(|char| char.to_digit(10))
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let digits = input.lines().map(numeric_digits);
    Ok(Box::new(total_calibration(digits)?))
}

const NUMBERS: [&str; 18] = [
    "one", "1", "two", "2", "three", "3",
    "four", "4", "five", "5", "six", "6",
    "seven", "7", "eight", "8", "nine", "9"
];

#[allow(clippy::cast_possible_truncation)]
fn digits(input: &str) -> Vec<u32> {
    NUMBERS.iter()
        .enumerate()
        .flat_map(|(number, word)| {
            input.match_indices(word)
                .map(move |(index, _)| (index, number as u32 / 2 + 1))
        }).sorted_by_key(|(index, _)| *index)
        .map(snd)
        .collect()
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let digits = input.lines().map(digits);
    Ok(Box::new(total_calibration(digits)?))
}