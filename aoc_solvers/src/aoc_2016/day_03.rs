use aoc_lib::parsing::{parse_lines, Runnable, ParseError};
use aoc_runner_api::SolverResult;
use itertools::Itertools;
use nom::{sequence::{tuple, preceded}, character::complete::{self, multispace1}, Parser};
use tupletools::snd;

struct Triangle(u32, u32, u32);

impl Triangle {
    fn is_valid(&self) -> bool {
        let &Self(x, y, z) = self;

        x + y > z &&
        y + z > x &&
        x + z > y
    }

    fn parse(input: &str) -> Result<Triangle, ParseError> {
        tuple((
            preceded(multispace1, complete::u32),
            preceded(multispace1, complete::u32),
            preceded(multispace1, complete::u32)
        )).map(|(x, y, z)| Triangle(x, y, z))
            .run(input)
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let number_valid = parse_lines(Triangle::parse, input)?
        .iter()
        .filter(|&triangle| triangle.is_valid())
        .count();

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
        .filter(Triangle::is_valid)
        .count();

    Ok(Box::new(number_valid))
}