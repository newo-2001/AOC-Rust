use nom::{bytes::complete::tag, character::complete::{alpha1, line_ending}, combinator::map, multi::{count, separated_list0}, sequence::separated_pair, Parser};
use recursive::recursive;
use yuki::parsing::{Parsable, ParserExt, ParsingResult};

use crate::SolverResult;

struct Towel<'a>(&'a str);

impl<'a> Parsable<'a> for Towel<'a> {
    fn parse(input: &'a str) -> ParsingResult<'a, Self> {
        map(
            alpha1,
            Self
        )
        .parse(input)
    }
}

fn parse_towels(input: &str) -> ParsingResult<'_, (Vec<Towel<'_>>, Vec<Towel<'_>>)> {
    separated_pair(
        separated_list0(tag(", "), Towel::parse),
        count(line_ending, 2),
        separated_list0(line_ending, Towel::parse)
    )
    .parse(input)
}

impl Towel<'_> {
    #[recursive]
    fn is_composite_of(&self, towels: &[Self]) -> bool {
        if self.0.is_empty() { return true; }

        towels
            .iter()
            .filter(|&towel| self.0.starts_with(towel.0))
            .any(|towel| Towel(&self.0[towel.0.len()..]).is_composite_of(towels))
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let (towels, desired_patterns) = parse_towels.run(input)?;

    let possible = desired_patterns
        .into_iter()
        .filter(|pattern| pattern.is_composite_of(&towels))
        .count();

    Ok(Box::new(possible))
}