use std::{iter::once, cmp::max, collections::BTreeSet};

use ahash::{HashSet, HashSetExt, HashMap, HashMapExt};
use anyhow::{Result, bail, Context};
use aoc_lib::geometry::{CardinalDirection, grid::{Grid, GridLike}, Point2D};
use yuki::{iterators::{ExtraIter, SingleError}, tuples::fst};
use crate::SolverResult;
use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Path,
    Forest,
    Slope(CardinalDirection)
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Path,
            '#' => Self::Forest,
            '>' => Self::Slope(CardinalDirection::East),
            '^' => Self::Slope(CardinalDirection::North),
            '<' => Self::Slope(CardinalDirection::West),
            'v' => Self::Slope(CardinalDirection::South),
            _ => bail!("Invalid tile in input: '{value}'")
        })
    }
}

struct Map {
    tiles: Grid<Tile>,
    start: Point2D<usize>,
    end: Point2D<usize>
}

fn find_hole(row: impl IntoIterator<Item=&Tile>) -> Result<usize, SingleError> {
    row
        .into_iter()
        .enumerate()
        .filter_map(|(x, &tile)| (tile == Tile::Path).then_some(x))
        .single()
}

struct State {
    position: Point2D<usize>,
    seen: HashSet<Point2D<usize>>,
    distance: usize
}

impl Map {
    fn parse(input: &str) -> Result<Self> {
        let tiles: Grid<Tile> = match Grid::parse(input) {
            Ok(tiles) => tiles,
            Err(error) => bail!(error)
        };

        let start = match find_hole(tiles.get_row(0).context("Map contains no tiles")?) {
            Ok(x) => Point2D(x, 0),
            Err(SingleError::More) => bail!("Map contained more than one entrance"),
            Err(SingleError::None) => bail!("Map contained no entrance")
        };
        
        let bottom = tiles.area().bottom();
        let end: Point2D<usize> = match find_hole(tiles.get_row(bottom).context("Map contains no tiles")?) {
            Ok(x) => Point2D(x, bottom),
            Err(SingleError::More) => bail!("Map contained more than one exit"),
            Err(SingleError::None) => bail!("Map contained no exit")
        };

        Ok(Self { tiles, start, end })
    }

    fn follow_straight(&self, mut previous: Point2D<usize>, mut position: Point2D<usize>) -> Option<(Point2D<usize>, usize)> {
        let mut distance = 0;
        loop {
            let directions = match self.tiles.get(position) {
                Some(Tile::Slope(direction)) => {
                    if direction == &previous.direction_to(position).unwrap() {
                        vec![*direction]
                    } else { vec![] }
                },
                _ => CardinalDirection::all().into_iter().collect_vec()
            };

            distance += 1;
            let branches = position
                .neighbours::<isize, _>(directions)
                .filter(|&neighbour| {
                    neighbour != previous &&
                    self.tiles.get(neighbour)
                        .is_some_and(|&tile| tile != Tile::Forest)
                }).single();
            
            match branches {
                Ok(next) => {
                    previous = position;
                    position = next;
                },
                Err(SingleError::More) => return Some((position, distance)),
                Err(SingleError::None) if position == self.end => return Some((position, distance)),
                Err(SingleError::None) => return None,
            }
        }
    }

    fn build_graph(&self) -> HashMap<Point2D<usize>, Vec<(Point2D<usize>, usize)>> {
        let mut graph = HashMap::<Point2D<usize>, Vec<(Point2D<usize>, usize)>>::new();
        let mut queue: BTreeSet<Point2D<usize>> = once(self.start).collect();

        while let Some(node) = queue.pop_first() {
            let edges = node.neighbours::<isize, _>(CardinalDirection::all())
                .filter(|&neighbour| match self.tiles.get(neighbour) {
                    Some(Tile::Path) => true,
                    Some(Tile::Forest) | None => false,
                    Some(Tile::Slope(direction)) => direction == &node.direction_to(neighbour).unwrap()
                }).filter_map(|neighbour| {
                    self.follow_straight(node, neighbour)
                }).collect_vec();
            
            edges
                .iter()
                .copied()
                .map(fst)
                .filter(|end| !graph.contains_key(end))
                .collect_into(&mut queue);

            graph.insert(node, edges);
        }

        graph
    }

    fn longest_hike(&self) -> Result<usize> {
        let graph = self.build_graph();

        let initial_state = State {
            position: self.start,
            seen: HashSet::new(),
            distance: 0
        };

        let mut best: Option<usize> = None;
        let mut queue = once(initial_state).collect_vec();

        while let Some(mut state) = queue.pop() {
            state.seen.insert(state.position);

            if state.position == self.end {
                best = Some(max(best.unwrap_or_default(), state.distance));
                continue;
            }
            
            graph
                .get(&state.position)
                .unwrap()
                .iter()
                .filter(|(node, _)| !state.seen.contains(node))
                .map(|&(node, distance)| {
                    State {
                        position: node,
                        distance: state.distance + distance,
                        seen: state.seen.clone()
                    }
                })
                .collect_into(&mut queue);
        }

        best.context("No path to the exit exists")
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let map = Map::parse(input)?;
    Ok(Box::new(map.longest_hike()?))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mut map = Map::parse(input)?;
    map.tiles = map.tiles.map(|tile| match tile {
        Tile::Slope(_) => Tile::Path,
        _ => tile
    });

    Ok(Box::new(map.longest_hike()?))
}