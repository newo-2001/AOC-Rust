use std::ops::Add;

use aoc_lib::{geometry::{Point3D, HexDirection, Directional}, parsing::{TextParser, ParseError}};
use yuki::errors::NoInput;
use crate::SolverResult;
use nom::{multi::separated_list0, character::complete::char};

pub fn parse_steps(input: &str) -> Result<Vec<HexDirection>, ParseError> {
    separated_list0(char(','), HexDirection::parse).run(input)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let distance: i32 = parse_steps(input)?
        .into_iter()
        .map(Directional::direction_vector)
        .fold(Point3D::zero(), Add::add)
        .hex_distance(Point3D::zero());

    Ok(Box::new(distance))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let furthest = parse_steps(input)?
        .into_iter()
        .scan(Point3D::<i32>::zero(), |position, direction| {
            *position += direction.direction_vector();
            Some(position.hex_distance(Point3D::zero()))
        })
        .max()
        .ok_or(NoInput)?;

    Ok(Box::new(furthest))
}