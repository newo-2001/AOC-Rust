use itertools::Itertools;
use yuki::{errors::NoInput, parsing::parse_lines, spatial::Point};

use crate::SolverResult;

pub fn solve_part_1(input: &str) -> SolverResult {
    let tiles: Vec<Point<u64>> = parse_lines(input)?;

    let max_size = tiles
        .into_iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let Point { x: width, y: height} = a.abs_diff(b) + Point::one();
            width * height
        })
        .max()
        .ok_or(NoInput)?;

    Ok(Box::new(max_size))
}