use ahash::{HashSet, HashSetExt};
use aoc_lib::geometry::{grid::{Grid, GridLike}, Point2D, Direction2D};
use aoc_runner_api::SolverResult;
use itertools::Itertools;
use std::hash::Hash;

#[derive(Clone, Copy)]
enum Tile {
    Symbol(char),
    Digit(u32),
    Period
}

impl TryFrom<char> for Tile {
    type Error = !;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Period,
            char => char.to_digit(10)
                .map_or(Tile::Symbol(char), Tile::Digit)
        })
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Number {
    position: Point2D<usize>,
    value: u32
}

// This is quite possibly the ugliest and most error prone code in this repo
fn numbers_at(grid: &Grid<Tile>, point: Point2D<usize>) -> Vec<Number> {
    let mut seen = HashSet::<Point2D<usize>>::new();
    let mut numbers: Vec<Number> = Vec::new();

    for mut pos in point.neighbours::<isize, _>(Direction2D::all()) {
        if seen.contains(&pos) || !matches!(grid.get(pos), Some(Tile::Digit(_))) { continue; }

        // Move back to start of the number
        while let Some(previous) = pos.checked_add(Point2D::<isize>(-1, 0)) {
            if !matches!(grid.get(previous), Some(Tile::Digit(_))) { break; }
            pos = previous;
        }

        // Accumulate number whilst moving along
        let mut number = Number { value: 0, position: pos };
        while let Some(Tile::Digit(digit)) = grid.get(pos) {
            number.value = number.value * 10 + digit;
            seen.insert(pos);
            pos += Point2D(1, 0);
        }

        numbers.push(number);
    }

    numbers
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let grid: Grid<Tile> = Grid::parse(input)?;
    let parts_sum: u32 = grid.enumerate()
        .filter(|(_, tile)| matches!(tile, Tile::Symbol(_)))
        .flat_map(|(pos, _)| numbers_at(&grid, pos))
        .collect::<HashSet<Number>>()
        .into_iter()
        .dedup()
        .map(|number| number.value)
        .sum();

    Ok(Box::new(parts_sum))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let grid: Grid<Tile> = Grid::parse(input)?;

    let gear_ratio_sum: u32 = grid.enumerate()
        .filter(|(_, tile)| matches!(tile, Tile::Symbol('*')))
        .filter_map(|(pos, _)| {
            match numbers_at(&grid, pos).as_slice() {
                [Number { value: left, .. }, Number { value: right, .. }] => Some(left * right),
                _ => None
            }
        }).sum();

    Ok(Box::new(gear_ratio_sum))
}