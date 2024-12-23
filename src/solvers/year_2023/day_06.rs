use aoc_lib::{parsing::TextParser, math::{Quadratic, Roots}};
use crate::SolverResult;
use itertools::Itertools;
use anyhow::Context;
use nom::{bytes::complete::tag, character::complete::{line_ending, space1, u64}, multi::separated_list1, sequence::{pair, preceded, separated_pair}};

struct Race {
    time: u64,
    distance: u64
}

impl Race {
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    fn winning_options(&self) -> u64 {
        let formula = Quadratic {
            a: -1f64,
            b: self.time as f64,
            c: -(self.distance as f64)
        };

        match formula.roots() {
            Roots::Pair(left, right) => (left.ceil() - right.floor()) as u64 - 1,
            Roots::Single(_) => 1,
            Roots::None => 0
        }
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let (times, distances) = separated_pair(
        preceded(
            pair(tag("Time:"), space1),
            separated_list1(space1, u64),
        ),
        line_ending,
        preceded(
            pair(tag("Distance:"), space1),
            separated_list1(space1, u64)
        )
    ).run(input)?;

    let margin_of_error: u64 = times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Race { time, distance }.winning_options())
        .product();

    Ok(Box::new(margin_of_error))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let (time, distance) = input
        .lines()
        .map(|line| line
            .chars()
            .filter(char::is_ascii_digit)
            .collect::<String>()
            .parse::<u64>()
        )
        .collect_tuple()
        .context("Input did not have exactly 2 lines")?;

    let race = Race { time: time?, distance: distance? };
    Ok(Box::new(race.winning_options()))
}