use std::cmp::min;

use ahash::{HashSet, HashSetExt};
use aoc_lib::{parsing::InvalidTokenError, geometry::{Point2D, grid::{Grid, GridLikeMut, GridLike}, CardinalDirection, Directional}, iteration::generate};
use aoc_runner_api::SolverResult;
use anyhow::{Result, Context, bail, anyhow};
use itertools::Itertools;
use num::Integer;

#[derive(Clone, Copy)]
enum Tile {
    Pipe(CardinalDirection, CardinalDirection),
    Ground
}

impl TryFrom<char> for Tile {
    type Error = InvalidTokenError<char>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '|' => Self::Pipe(CardinalDirection::North, CardinalDirection::South),
            '-' => Self::Pipe(CardinalDirection::East, CardinalDirection::West),
            'L' => Self::Pipe(CardinalDirection::North, CardinalDirection::East),
            'J' => Self::Pipe(CardinalDirection::North, CardinalDirection::West),
            '7' => Self::Pipe(CardinalDirection::South, CardinalDirection::West),
            'F' => Self::Pipe(CardinalDirection::South, CardinalDirection::East),
            '.' | 'S' => Self::Ground,
            c => return Err(InvalidTokenError(c))
        })
    }
}

impl Tile {
    fn exit_direction(self, facing: CardinalDirection) -> Option<CardinalDirection> {
        let entrance = facing.reverse();
        match self {
            Tile::Pipe(start, end) if entrance == start => Some(end),
            Tile::Pipe(start, end) if entrance == end => Some(start),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
struct State {
    position: Point2D<usize>,
    facing: CardinalDirection
}

impl State {
    fn next(&self, map: &Grid<Tile>) -> Result<State> {
        let tile = map.get(self.position).context("Position is not on the grid")?;
        let direction = tile.exit_direction(self.facing).context("Pipe lead to dead end")?;
        let position = self.position.checked_add::<isize>(direction.direction_vector())
            .context("Moved off the grid")?;

        Ok(State {
            facing: direction,
            position
        })
    }

    fn find_loop(mut self, map: &Grid<Tile>) -> Result<HashSet<Point2D<usize>>> {
        let mut seen: HashSet<Point2D<usize>> = HashSet::new();

        while seen.insert(self.position) {
            self = self.next(map)?;
        }

        Ok(seen)
    }
}

fn parse_map(input: &str) -> Result<(Grid<Tile>, Point2D<usize>)> {
    let mut map: Grid<Tile> = Grid::parse(input)?;
    let start = input.lines()
        .enumerate()
        .find_map(|(y, line)| line.find('S').map(|x| Point2D(x, y)))
        .context("Input contains no starting position")?;

    let [entrance, exit]: [CardinalDirection; 2] = CardinalDirection::all()
        .into_iter()
        .filter(|direction| {
            start.checked_add::<isize>(direction.direction_vector())
                .and_then(|location| map.get(location))
                .and_then(|tile| tile.exit_direction(*direction))
                .is_some()
        }).collect_vec()
        .try_into()
        .map_err(|_| anyhow!("Starting position is not connected to exactly 2 pipes"))?;

    let tile = map.get_mut(start)
        .context("Starting position is not inside the grid")?;
    
    *tile = Tile::Pipe(entrance, exit);
    
    Ok((map, start))
}

fn loop_pipes(map: &Grid<Tile>, start: Point2D<usize>) -> Result<HashSet<Point2D<usize>>> {
    match map[start] {
        Tile::Ground => bail!("Starting position is not a pipe"),
        Tile::Pipe(entrance, _) => {
            State { position: start, facing: entrance.reverse() }.find_loop(map)
        }
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let (map, start) = parse_map(input)?;
    let distance: usize = loop_pipes(&map, start)?.len() / 2;

    Ok(Box::new(distance))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let (map, start) = parse_map(input)?;
    let loop_pipes: HashSet<Point2D<usize>> = loop_pipes(&map, start)?;

    let inside: usize = map
        .area()
        .into_iter()
        .filter(|pos| !loop_pipes.contains(pos))
        .filter(|pos| {
            let (up, down) = generate(*pos, |pos| pos.checked_add(Point2D::<isize>(-1, 0)))
                .filter(|pos| loop_pipes.contains(pos))
                .map(|pos| {
                    let tile = map[pos];
                    let up = matches!(tile, Tile::Pipe(CardinalDirection::North, CardinalDirection::South | CardinalDirection::East | CardinalDirection::West));
                    let down = matches!(tile, Tile::Pipe(CardinalDirection::North, CardinalDirection::South) | Tile::Pipe(CardinalDirection::South, CardinalDirection::East | CardinalDirection::West));
                    (usize::from(up), usize::from(down))
                }).reduce(|acc, current| (acc.0 + current.0, acc.1 + current.1))
                .unwrap_or_default();

            min(up, down).is_odd()
        }).count();

    Ok(Box::new(inside))
}