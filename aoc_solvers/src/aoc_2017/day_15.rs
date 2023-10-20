use aoc_lib::{iteration::ExtraIter, parsing::{TextParserResult, TextParser, ParseError}};
use aoc_runner_api::SolverResult;
use nom::{bytes::complete::tag, character::complete::{anychar, line_ending, u128}, sequence::{tuple, preceded, terminated}, Parser, combinator::opt};
use num::Integer;


struct Generator
{
    factor: u128,
    previous: u128,
}

impl Iterator for Generator {
    type Item = u128; 

    fn next(&mut self) -> Option<Self::Item> {
        self.previous = self.previous.wrapping_mul(self.factor) % (i32::MAX as u128);
        Some(self.previous)
    }
}

fn parse_generators(input: &str) -> Result<(Generator, Generator), ParseError> {
    fn parse_seed(input: &str) -> TextParserResult<u128> {
        preceded(
            tuple((tag("Generator "), anychar, tag(" starts with "))),
            terminated(u128, opt(line_ending))
        ).parse(input)
    }

    let (seed_a, seed_b) = parse_seed.and(parse_seed).run(input)?;
    let a = Generator { factor: 16807, previous: seed_a };
    let b = Generator { factor: 48271, previous: seed_b };

    Ok((a, b))
}

fn judge((a, b): (u128, u128)) -> bool {
    let mask = u128::from(u16::MAX);
    a & mask == b & mask
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let (a, b) = parse_generators(input)?;
    
    let matches = a.zip(b)
        .take(40_000_000)
        .count_where(judge);

    Ok(Box::new(matches))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let (a, b) = parse_generators(input)?;
    
    let matches = a.filter(|n| n.is_multiple_of(&4))
        .zip(b.filter(|n| n.is_multiple_of(&8)))
        .take(5_000_000)
        .count_where(judge);

    Ok(Box::new(matches))
}