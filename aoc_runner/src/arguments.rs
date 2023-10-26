use aoc_runner_api::{Puzzle, PuzzlePart};
use nom::{combinator::{eof, all_consuming, value}, character::complete::{u16, char, u8}, sequence::{separated_pair, tuple, preceded}, Parser, branch::alt};
use thiserror::Error;

use crate::RunnerAction;

#[derive(Clone, Copy)]
enum RunnerScope {
    All,
    Year(u16),
    Day(u16, u8),
    Puzzle(Puzzle)
}

impl RunnerScope {
    fn parse(input: &str) -> Result<Self, nom::Err<nom::error::Error<&str>>> {
        Ok(all_consuming(alt((
            value(RunnerScope::All, eof),
            tuple((
                u16,
                preceded(char('-'), u8),
                preceded(char('-'), Parser::or(
                    value(PuzzlePart::First, char('1')),
                    value(PuzzlePart::Second, char('2'))
                ))
            )).map(|(year, day, part)| {
                RunnerScope::Puzzle(Puzzle { year, day, part })
            }),
            separated_pair(u16, char('-'), u8)
                .map(|(year, day)| RunnerScope::Day(year, day)),
            u16.map(RunnerScope::Year)
        ))).parse(input)?.1)
    }

    fn puzzles(self) -> impl Iterator<Item=Puzzle> {
        match self {
            RunnerScope::Puzzle(puzzle) => vec![puzzle],
            RunnerScope::Day(year, 25) => vec![
                Puzzle { year, day: 25, part: PuzzlePart::First }
            ],
            RunnerScope::Day(year, day) => vec![
                Puzzle { year, day, part: PuzzlePart::First },
                Puzzle { year, day ,part: PuzzlePart::Second }
            ],
            RunnerScope::Year(year) => (1..=25).flat_map(|day| {
                RunnerScope::Day(year, day).puzzles()
            }).collect(),
            RunnerScope::All => (2015..=2022).flat_map(|year| {
                RunnerScope::Year(year).puzzles()
            }).collect()
        }.into_iter()
            .filter(|&puzzle| aoc_solvers::get_solver(puzzle).is_some())
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Parsing(#[from] nom::Err<nom::error::Error<String>>),
    #[error("Encountered unexpected position argument: `{0}`")]
    UnexpectedArgument(String)
}

// TODO: clean this up, possibly using clap
impl RunnerAction {
    pub fn parse(arguments: &[&str]) -> Result<RunnerAction, Error> {
        match arguments {
            ["verify", scope] => {
                let scope = RunnerScope::parse(scope)
                    .map_err(nom::Err::<nom::error::Error<&str>>::to_owned)?;
                Ok(RunnerAction::Verify(scope.puzzles().collect()))
            },
            ["verify"] | [] => {
                Ok(RunnerAction::Verify(RunnerScope::All.puzzles().collect()))
            },
            [scope] => {
                let scope = RunnerScope::parse(scope)
                    .map_err(nom::Err::<nom::error::Error<&str>>::to_owned)?;
                Ok(RunnerAction::Run(scope.puzzles().collect()))
            },
            ["verify", _, arg] | [_, arg, ..] => {
                Err(Error::UnexpectedArgument((*arg).to_owned()))
            }
        }
    }
}