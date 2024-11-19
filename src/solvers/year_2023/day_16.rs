use std::iter::once;

use aoc_lib::{geometry::{grid::{Grid, GridLike}, Axis, Point2D, CardinalDirection, Directional}, iteration::queue::{Dedupable, IterState}, errors::NoInput, parsing::InvalidTokenError};
use crate::SolverResult;
use anyhow::Result;
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::hash::Hash;

enum Tile {
    Splitter(Axis),
    Slash,
    Backslash,
    Air
}

impl TryFrom<char> for Tile {
    type Error = InvalidTokenError<char>;
    
    fn try_from(value: char) -> Result<Tile, Self::Error> {
        Ok(match value {
            '|' => Self::Splitter(Axis::Vertical),
            '-' => Self::Splitter(Axis::Horizontal),
            '/' => Self::Slash,
            '\\' => Self::Backslash,
            '.' => Self::Air,
            _ => return Err(InvalidTokenError(value))
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Beam(Point2D<usize>, CardinalDirection);

impl Default for Beam {
    fn default() -> Self { Beam(Point2D::zero(), CardinalDirection::East) }
}

impl Beam {
    fn forward(self, grid: &Grid<Tile>) -> Option<Beam> {
        let Beam(location, direction) = self;
        let new_pos = location.checked_add::<isize>(direction.direction_vector())?;
        grid.get(new_pos).is_some().then_some(Beam(new_pos, direction))
    }

    fn energize(self, grid: &Grid<Tile>) -> usize {
        let mut filter = once(self).collect_vec().filter_duplicates();
        filter.recursive_iter(|Beam(location, direction)| {
            type Dir = CardinalDirection;

            if let Some(tile) = grid.get(location) {
                let beams = match (tile, direction) {
                    (Tile::Splitter(Axis::Horizontal), Dir::North | Dir::South) => vec![Dir::East, Dir::West],
                    (Tile::Splitter(Axis::Vertical), Dir::East | Dir::West) => vec![Dir::North, Dir::South],
                    (Tile::Slash, Dir::North) | (Tile::Backslash, Dir::South) => vec![Dir::East],
                    (Tile::Slash, Dir::South) | (Tile::Backslash, Dir::North) => vec![Dir::West],
                    (Tile::Slash, Dir::West) | (Tile::Backslash, Dir::East) => vec![Dir::South],
                    (Tile::Slash, Dir::East) | (Tile::Backslash, Dir::West) => vec![Dir::North],
                    (Tile::Air | Tile::Splitter(_), direction) => vec![direction]
                }.into_iter()
                    .filter_map(|direction| Beam(location, direction).forward(grid))
                    .collect_vec();

                IterState::Branch(beams)
            } else { IterState::Leaf }
        });

        filter.seen
            .into_iter()
            .unique_by(|&Beam(location, _)| location)
            .count()
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let grid = Grid::parse(input)?;
    Ok(Box::new(Beam::default().energize(&grid)))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let grid = Grid::parse(input)?;
    let area = grid.area();
    let dimensions = area.dimensions();

    let max_energized = (0..dimensions.width()).flat_map(|x| [
        Beam (Point2D(x, area.top()), CardinalDirection::South),
        Beam (Point2D(x, area.bottom()), CardinalDirection::North)
    ]).chain((0..dimensions.height()).flat_map(|y| [
        Beam(Point2D(area.left(), y), CardinalDirection::East),
        Beam(Point2D(area.right(), y), CardinalDirection::West)
    ])).par_bridge()
        .map(|beam| beam.energize(&grid))
        .max()
        .ok_or(NoInput)?;
    
    Ok(Box::new(max_energized))
}