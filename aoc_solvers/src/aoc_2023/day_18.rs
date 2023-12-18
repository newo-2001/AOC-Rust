use std::iter::once;

use anyhow::bail;
use aoc_lib::{geometry::{CardinalDirection, Point2D, Directional, Polygon}, parsing::{ParseError, Parsable, TextParser, parse_lines, usize, skip_until, parens}};
use aoc_runner_api::SolverResult;
use nom::{sequence::{separated_pair, terminated, preceded, pair}, character::complete::{char, anychar}, combinator::{rest, map_res}, Parser, bytes::complete::take};

#[derive(Debug)]
struct Instruction {
    direction: CardinalDirection,
    amount: usize
}

impl Instruction {
    fn parse(input: &str) -> Result<Self, ParseError> {
        separated_pair(
            CardinalDirection::parse,
            char(' '),
            terminated(usize, rest)
        ).map(|(direction, amount)| Self { direction, amount }).run(input)
    }

    fn parse_hex(input: &str) -> Result<Self, ParseError> {
        let direction = map_res(anychar, |char| Ok(match char {
            '0' => CardinalDirection::West,
            '1' => CardinalDirection::South,
            '2' => CardinalDirection::East,
            '3' => CardinalDirection::North,
            _ => bail!("Invalid direction hex digit: {char}")
        }));

        let amount = map_res(take(5usize), |hex| usize::from_str_radix(hex, 16));
        let hex = preceded(char('#'), pair(amount, direction))
            .map(|(amount, direction)| Self { direction, amount });

        skip_until(parens(hex)).run(input)
    }
}

fn create_polygon(instructions: impl IntoIterator<Item=Instruction>) -> Polygon<i64> {
    let instructions = instructions.into_iter().collect::<Vec<_>>();

    instructions.into_iter()
        .scan(Point2D::zero(), |point, instruction| {
            *point += instruction.direction.direction_vector() * i64::try_from(instruction.amount).unwrap();
            Some(*point)
        }).chain(once(Point2D::zero()))
            .collect()
}

fn area(mut polygon: Polygon<i64>) -> i64 {
    match polygon.pick() {
        area if area >= 0 => area,
        _ => {
            polygon.flip();
            polygon.pick()
        }
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let polygon: Polygon<i64> = create_polygon(parse_lines(Instruction::parse, input)?);
    Ok(Box::new(area(polygon)))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let polygon: Polygon<i64> = create_polygon(parse_lines(Instruction::parse_hex, input)?);
    Ok(Box::new(area(polygon)))
}