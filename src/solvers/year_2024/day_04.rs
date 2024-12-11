use yuki::{iterators::{Enumerate2D, ExtraIter}, spatial::{direction::{self, Directions}, matrix::{Matrix, VariableRows}, Point}};

use crate::SolverResult;

fn parse_grid(input: &str) -> Result<Matrix<char>, VariableRows> {
    input
        .lines()
        .map(str::chars)
        .try_collecting()
}

fn is_word<D>(needle: &'static str, grid: &Matrix<char>, position: Point<usize>, direction: D) -> bool where
    D: Directions + Copy
{
    needle
        .chars()
        .enumerate()
        .all(|(offset, letter)| {
            #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
            let offset = Point::from(direction.vector()) * offset as isize;

            position
                .add_signed(offset)
                .and_then(|pos| grid.get(pos))
                .is_some_and(|&l| l == letter)
        })
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let grid = parse_grid(input)?;

    let solutions = grid
        .iter_rows()
        .enumerate2d()
        .flat_map(|(pos, _)| direction::Compass::all()
            .map(move |dir| (pos, dir))
        )
        .filter(|&(pos, dir)| is_word("XMAS", &grid, pos, dir))
        .count();

    Ok(Box::new(solutions))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let grid = parse_grid(input)?;

    let solutions = grid
        .iter_rows()
        .enumerate2d()
        .filter(|(pos, _)| direction::Ordinal::all()
            .filter_map(|dir| Some((pos.add_signed(dir.vector())?, dir.inverted())))
            .filter(|&(pos, dir)| is_word("MAS", &grid, pos, dir))
            .count() >= 2
        )
        .count();

    Ok(Box::new(solutions))
}