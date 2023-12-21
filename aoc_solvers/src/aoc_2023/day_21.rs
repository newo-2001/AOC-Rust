use std::{collections::BTreeSet, iter::once};

use anyhow::{Context, Result, bail};
use aoc_lib::{geometry::{grid::{Grid, GridLike}, Point2D, CardinalDirection}, functional::repeat_apply};
use aoc_runner_api::SolverResult;
use nom::InputIter;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    GardenPlot,
    Rock
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Rock),
            '.' | 'S' => Ok(Self::GardenPlot),
            _ => bail!("Invalid tile: '{value}'")
        }
    }
}

fn parse_grid(input: &str) -> Result<(Point2D<usize>, Grid<Tile>)> {
    let start = input.lines()
        .enumerate()
        .find_map(|(y, line)| Some(Point2D(line.position(|char| char == 'S')?, y)))
        .context("Input contains no starting position")?;

    match Grid::parse(input) {
        Ok(grid) => Ok((start, grid)),
        Err(error) => bail!(error)
    }
}

fn step(mut locations: BTreeSet<Point2D<usize>>, grid: &Grid<Tile>) -> BTreeSet<Point2D<usize>> {
    let mut new_locations: BTreeSet<Point2D<usize>> = BTreeSet::new();

    while let Some(location) = locations.pop_first() {
        location.neighbours::<isize, _>(CardinalDirection::all())
            .filter_map(|neighbour| {
                (grid.get(neighbour)? == &Tile::GardenPlot).then_some(neighbour)
            })
            .collect_into(&mut new_locations);
    }

    new_locations
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let (start, grid) = parse_grid(input)?;

    let initial_steps: BTreeSet<Point2D<usize>> = once(start).collect();
    let final_steps = repeat_apply(64, initial_steps, |steps| step(steps, &grid));

    Ok(Box::new(final_steps.len()))
}