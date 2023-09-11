use std::collections::HashSet;

use aoc_lib::{geometry::{CardinalDirection, Point2D}, parsing::{Runnable, ParseError}};
use aoc_runner_api::SolverResult;
use nom::multi::many0;

fn unique_houses<'a>(directions: impl IntoIterator<Item=&'a CardinalDirection>) -> HashSet<Point2D<i32>> {
    let mut position = Point2D::<i32>::zero();

    directions.into_iter()
        .map(|direction| {
            position += direction.direction_vector();
            position
        }).chain(std::iter::once(Point2D::zero()))
        .collect()
}

fn parse_movements(input: &str) -> Result<Vec<CardinalDirection>, ParseError> {
    many0(CardinalDirection::parse).run(input)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let movements = parse_movements(input)?;
    let visited_houses = unique_houses(&movements).len();

    Ok(Box::new(visited_houses))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let movements = parse_movements(input)?;
    let santa_houses = unique_houses(movements.iter().step_by(2));
    let robo_santa_houses = unique_houses(movements.iter().skip(1).step_by(2));
    let all_houses = santa_houses.union(&robo_santa_houses);

    Ok(Box::from(all_houses.count()))
}