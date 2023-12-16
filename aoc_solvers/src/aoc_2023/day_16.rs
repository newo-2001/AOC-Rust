use std::iter::once;

use ahash::HashMap;
use aoc_lib::{geometry::{Axis, Point2D, CardinalDirection, Directional}, iteration::queue::{Dedupable, IterState}, errors::NoInput};
use aoc_runner_api::SolverResult;
use anyhow::{Result, bail};
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::hash::Hash;

enum Tile {
    Splitter(Axis),
    Slash,
    Backslash,
    Air
}

impl Tile {
    fn parse(value: char) -> Result<Option<Self>> {
        Ok(Some(match value {
            '|' => Self::Splitter(Axis::Vertical),
            '-' => Self::Splitter(Axis::Horizontal),
            '/' => Self::Slash,
            '\\' => Self::Backslash,
            '.' => return Ok(None),
            _ => bail!("Encountered invalid tile in input: {}", value)
        }))
    }
}

struct Grid {
    tiles: HashMap<Point2D<usize>, Tile>,
    width: usize,
    height: usize
}

impl Grid {
    fn new(input: &str) -> Result<Self> {
        let lines = input.lines().collect_vec();
        let height = lines.len();
        let width = lines.first().map_or(0, |line| line.chars().count());
        let tiles = lines.into_iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .filter_map(move |(x, tile)| match Tile::parse(tile) {
                        Ok(rock) => rock.map(|rock| Ok((Point2D(x, y), rock))),
                        Err(err) => Some(Err(err)),
                    })
            }).collect::<Result<HashMap<Point2D<usize>, Tile>>>()?;

        Ok(Self { tiles, width, height })
    }

    fn get(&self, location: Point2D<usize>) -> Option<&Tile> {
        if location.x() >= self.width || location.y() >= self.height { None }
        else { self.tiles.get(&location).or(Some(&Tile::Air)) }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Beam(Point2D<usize>, CardinalDirection);

impl Default for Beam {
    fn default() -> Self { Beam(Point2D::zero(), CardinalDirection::East) }
}

impl Beam {
    fn forward(self, grid: &Grid) -> Option<Beam> {
        let Beam(location, direction) = self;
        let new_pos = location.checked_add::<isize>(direction.direction_vector())?;
        grid.get(new_pos).is_some().then_some(Beam(new_pos, direction))
    }

    fn energize(self, grid: &Grid) -> usize {
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
    let grid = Grid::new(input)?;
    Ok(Box::new(Beam::default().energize(&grid)))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let grid = Grid::new(input)?;

    let max_energized = (0..grid.width).flat_map(|x| [
        Beam (Point2D(x, 0), CardinalDirection::South),
        Beam (Point2D(x, grid.height - 1), CardinalDirection::North)
    ]).chain((0..grid.height).flat_map(|y| [
        Beam(Point2D(0, y), CardinalDirection::East),
        Beam(Point2D(grid.width - 1, y), CardinalDirection::West)
    ])).par_bridge()
        .map(|beam| beam.energize(&grid))
        .max()
        .ok_or(NoInput)?;
    
    Ok(Box::new(max_energized))
}