use aoc_lib::{parsing::{parse_lines, TextParser, ParseError}, iteration::ExtraIter};
use yuki::tuples::snd;
use crate::SolverResult;
use itertools::Itertools;
use nom::{character::complete::{multispace1, u32}, combinator::map, sequence::{preceded, tuple}};

#[derive(Clone, Copy)]
struct Triangle(u32, u32, u32);

impl Triangle {
    const fn is_valid(&self) -> bool {
        let &Self(x, y, z) = self;

        x + y > z &&
        y + z > x &&
        x + z > y
    }

    fn parse(input: &str) -> Result<Self, ParseError> {
        map(
            tuple((
                preceded(multispace1, u32),
                preceded(multispace1, u32),
                preceded(multispace1, u32)
            )),
            |(x, y, z)| Self(x, y, z)
        )
        .run(input)
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let number_valid = parse_lines(Triangle::parse, input)?
        .into_iter()
        .count_where(Triangle::is_valid);

    Ok(Box::new(number_valid))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let number_valid = parse_lines(Triangle::parse, input)?
        .into_iter()
        .flat_map(|Triangle(a, b, c)| [a, b, c])
        .enumerate()
        .into_group_map_by(|(index, _)| index % 3)
        .into_values()
        .flatten()
        .map(snd)
        .tuples()
        .map(|(a, b, c)| Triangle(a, b, c))
        .count_where(Triangle::is_valid);

    Ok(Box::new(number_valid))
}