use std::{collections::VecDeque, iter::once, fmt::Display};

use aoc_lib::{math::Bit, parsing::InvalidTokenError, geometry::{Dimensions, grid::{Grid, GridLike}, Point2D, WrongDimensionsError, CardinalDirection}, iteration::{queue::{SearchDepth, Dedupable, FindState}, SingleError, ExtraIter}, NoSolutionError};
use aoc_runner_api::SolverResult;
use itertools::{Itertools, Either};
use thiserror::Error;
use tupletools::fst;

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
    fn new(position: Point2D<usize>) -> Self {
        State {
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
    fn shortest_path(&self, should_return: bool) -> Result<usize, NoSolutionError> {
        let queue: VecDeque<_> = once(SearchDepth::new(State::new(self.start))).collect();
        queue.filter_duplicates().recursive_find(|mut search| {
            if search.state.visited_targets.len() == self.targets.len() {
                if !should_return || search.state.position == self.start {
                    return FindState::Result(search.depth)
                } else if !search.state.returning { search.state.returning = true }
            }

            let neighbours = search.state.position.neighbours(CardinalDirection::all());
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
        }).ok_or(NoSolutionError)
    }
}

#[derive(Debug, Error)]
enum VentParseError {
    InvalidToken(#[from] InvalidTokenError<char>),
    NonRectangularGrid(#[from] WrongDimensionsError),
    StartError(#[from] SingleError)
}

impl Display for VentParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::InvalidToken(err) => err.to_string(),
            Self::NonRectangularGrid(_) => String::from("The grid is not rectangular"),
            Self::StartError(err) => match err {
                SingleError::None => String::from("The input contains no starting position"),
                SingleError::More => String::from("The input contains multiple starting positions")
            }
        };

        write!(f, "{message}")
    }
}

fn parse_vent(input: &str) -> Result<Vent, VentParseError> {
    let tiles: Vec<Vec<_>> = input.lines()
        .map(|line| line.chars().map(parse_tile).try_collect())
        .try_collect()?;

    let dimensions = Dimensions(tiles[0].len(), tiles.len());
    let point_from_index = |index| Point2D(index % dimensions.width(), index / dimensions.width());

    let (start, targets): (Vec<_>, Vec<_>) = tiles.iter()
        .flatten()
        .enumerate()
        .filter_map(|(index, (_, target))| {
            target.map(|target| (target, point_from_index(index)))
        }).partition_map(|target| match target {
            (0, position) => Either::Left(position),
            (_, position) => Either::Right(position)
        });
    
    let start = start.into_iter().single()?;
    let tiles = tiles.into_iter().flatten().map(fst);
    let grid = Grid::from_iter(dimensions, tiles)?;
    
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