use nom::{Parser, character::complete::char, combinator::value};
use yuki::{iterators::Enumerate2D, parsing::{Parsable, ParsingResult, parse}, spatial::{Matrix, Point, direction}};

use crate::SolverResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Paper,
    Air
}

impl Parsable<'_> for Tile {
    fn parse(input: &str) -> ParsingResult<'_, Self> {
        Parser::or(
            value(Tile::Paper, char('@')),
            value(Tile::Air, char('.'))
        ).parse(input)
    }
}

fn accessible_rolls(grid: &Matrix<Tile>) -> impl Iterator<Item=Point<usize>> {
    grid
        .iter_rows()
        .enumerate2d()
        .filter_map(|(point, tile)| (*tile == Tile::Paper).then_some(point))
        .filter(|point| point
            .neighbours::<direction::Compass>()
            .filter(|&pos| matches!(grid.get(pos), Some(Tile::Paper)))
            .count() < 4
        )
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let grid: Matrix<Tile> = parse(input)?;

    Ok(Box::new(accessible_rolls(&grid).count()))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mut grid: Matrix<Tile> = parse(input)?;
    let mut total_removable_rolls = 0;

    loop {
        let positions: Box<[Point<usize>]> = accessible_rolls(&grid).collect();

        for &pos in &positions {
            *(&mut grid[pos]) = Tile::Air;
        }

        match positions.len() {
            0 => return Ok(Box::new(total_removable_rolls)),
            removed => total_removable_rolls += removed
        };
    }
}