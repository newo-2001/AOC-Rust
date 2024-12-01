use aoc_lib::parsing::{TextParserResult, TextParser};
use nom::{character::complete::char, combinator::value, Parser, multi::many1};
use yuki::errors::NoSolution;

use crate::SolverResult;

fn parse_directions(input: &str) -> TextParserResult<Vec<i32>> {
    many1(
        Parser::or(
            value(1, char('(')),
            value(-1, char(')'))
        )
    ).parse(input)
}

fn find_basement<'a>(directions: impl IntoIterator<Item=&'a i32>) -> Option<usize> {
    let mut floor: i32 = 0;
    for (i, direction) in directions.into_iter().enumerate() {
        floor += direction;
        if floor == -1 {
            return Some(i);
        }
    }

    None
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let directions = parse_directions.run(input)?;
    let destination: i32 = directions.iter().sum();
    Ok(Box::new(destination))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let directions = parse_directions.run(input)?;
    let index = find_basement(&directions)
        .ok_or(NoSolution)?;

    Ok(Box::new(index + 1))
}