use std::{collections::VecDeque, iter::once};

use aoc_lib::{math::Bit, parsing::InvalidTokenError, geometry::{Dimensions, grid::{Grid, GridLike}, Point2D, CardinalDirection}, iteration::queue::{SearchDepth, Dedupable, FindState}};
use yuki::{errors::NoSolution, tuples::fst};
use crate::SolverResult;
use itertools::{Itertools, Either};
use anyhow::{Context, Result};

fn parse_tile(input: char) -> Result<(Bit, Option<u32>), InvalidTokenError<char>> {
    Ok(match input {
        '.' => (Bit::Off, None),
        '#' => (Bit::On, None),
        c if c.is_ascii_digit() => (Bit::Off, c.to_digit(10)),
        _ => Err(InvalidTokenError(input))?
    })
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    visited_targets: Vec<Point2D<usize>>,
    position: Point2D<usize>,
    returning: bool
}

impl State {
    const fn new(position: Point2D<usize>) -> Self {
        Self {
            visited_targets: Vec::new(),
            returning: false,
            position
        }
    }
}

struct Vent {
    grid: Grid<Bit>,
    targets: Vec<Point2D<usize>>,
    start: Point2D<usize>
}

impl Vent {
    fn shortest_path(&self, should_return: bool) -> Result<usize, NoSolution> {
        let queue: VecDeque<_> = once(SearchDepth::new(State::new(self.start))).collect();
        queue.filter_duplicates().recursive_find(|mut search| {
            if search.state.visited_targets.len() == self.targets.len() {
                if !should_return || search.state.position == self.start {
                    return FindState::Result(search.depth)
                } else if !search.state.returning { search.state.returning = true }
            }

            let neighbours = search.state.position.neighbours::<isize, _>(CardinalDirection::all());
            let branches = neighbours.filter(|&position| {
                let tile = self.grid.get(position).unwrap_or(&Bit::On);
                !tile.is_solid()
            }).map(|position| {
                let mut visited_targets = search.state.visited_targets.clone();
                if !search.state.visited_targets.contains(&position) && self.targets.iter().contains(&position) {
                    visited_targets.push(position);
                }

                search.with(State { visited_targets, position, returning: search.state.returning })
            }).collect_vec();
            
            FindState::Branch(branches)
        })
        .ok_or(NoSolution)
    }
}

fn parse_vent(input: &str) -> Result<Vent> {
    let tiles: Vec<Vec<_>> = input.lines()
        .map(|line| line.chars().map(parse_tile).try_collect())
        .try_collect()?;

    let dimensions: Dimensions = (&tiles).try_into()?;

    let point_from_index = |index| Point2D(index % dimensions.width(), index / dimensions.width());

    let (start, targets): (Vec<_>, Vec<_>) = tiles
        .iter()
        .flatten()
        .enumerate()
        .filter_map(|(index, (_, target))| target
            .map(|target| (target, point_from_index(index)))
        )
        .partition_map(|target| match target {
            (0, position) => Either::Left(position),
            (_, position) => Either::Right(position)
        });
    
    let start = Iterator::exactly_one(start.into_iter())
        .context("Multiple starting points in input")?;

    let tiles = tiles.into_iter()
        .map(|row| row.into_iter().map(fst).collect_vec())
        .collect_vec();
    
    let grid = Grid::new(tiles)?;
    
    Ok(Vent { grid, targets, start })
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let vent = parse_vent(input)?;
    Ok(Box::new(vent.shortest_path(false)?))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let vent = parse_vent(input)?;
    Ok(Box::new(vent.shortest_path(true)?))
}