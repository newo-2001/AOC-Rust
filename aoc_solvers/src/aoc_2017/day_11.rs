use std::ops::Add;

use aoc_lib::{geometry::{Point3D, HexDirection}, parsing::{TextParser, ParseError}, NoInputError};
use aoc_runner_api::SolverResult;
use nom::{multi::separated_list0, character::complete::char};

pub fn parse_steps(input: &str) -> Result<Vec<HexDirection>, ParseError> {
    separated_list0(char(','), HexDirection::parse).run(input)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let distance: i32 = parse_steps(input)?
        .into_iter()
        .map(HexDirection::direction_vector)
        .fold(Point3D::zero(), Add::add)
        .hex_distance(Point3D::zero());

    Ok(Box::new(distance))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let furthest = parse_steps(input)?
        .into_iter()
        .scan(Point3D::zero(), |position, direction| {
            *position += direction.direction_vector::<i32>();
            Some(position.hex_distance(Point3D::zero()))
        }).max().ok_or(NoInputError)?;

    Ok(Box::new(furthest))
}