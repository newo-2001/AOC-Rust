use aoc_lib::parsing::TextParser;
use aoc_runner_api::SolverResult;
use itertools::Itertools;
use anyhow::anyhow;
use nom::{bytes::complete::tag, character::complete::{space1, line_ending, u64}, multi::separated_list1, sequence::{preceded, delimited}, Parser};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

struct Race {
    time: u64,
    distance: u64
}

impl Race {
    fn winning_options(&self) -> usize {
        (0..=self.time)
            .into_par_iter()
            .map(|held| held * (self.time - held))
            .filter(|distance| distance > &self.distance)
            .count()
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let (times, distances) = delimited(
        tag("Time:").and(space1),
        separated_list1(space1, u64),
        line_ending
    ).and(preceded(
        tag("Distance:").and(space1),
        separated_list1(space1, u64)
    )).run(input)?;

    let margin_of_error: usize = times.into_iter()
        .zip(distances)
        .map(|(time, distance)| Race { time, distance }.winning_options())
        .product();

    Ok(Box::new(margin_of_error))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let (time, distance) = input.lines()
        .map(|line| {
            line.chars()
                .filter(char::is_ascii_digit)
                .collect::<String>()
                .parse::<u64>()
        }).collect_tuple()
        .ok_or(anyhow!("Input did not have exactly 2 lines"))?;

    let race = Race { time: time?, distance: distance? };
    Ok(Box::new(race.winning_options()))
}