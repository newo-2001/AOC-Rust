use std::{collections::VecDeque, iter::once, hash::Hash};

use aoc_lib::{geometry::{Point2D, CardinalDirection}, iteration::queue::{Dedupable, FindState, IterState}, math::Bit};
use yuki::errors::NoSolution;
use crate::SolverResult;
use itertools::Itertools;
use num::Integer;

struct Grid {
    magic: usize
}

#[derive(Clone, Copy)]
struct SearchState {
    location: Point2D<usize>,
    depth: usize
}

impl Eq for SearchState {}
impl PartialEq for SearchState {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location
    }
}

impl Hash for SearchState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.location.hash(state);
    }
}

impl Grid {
    const fn new(magic: usize) -> Self {
        Self { magic }
    }

    fn get(&self, Point2D(x, y): Point2D::<usize>) -> Bit {
        let value = x*x + 3*x + 2*x*y + y + y*y + self.magic;
        (value.count_ones().is_odd()).into()
    }

    fn moves_at(&self, SearchState { location, depth }: SearchState) -> Vec<SearchState> {
        location.neighbours::<isize, _>(CardinalDirection::all())
            .filter(|&location| !self.get(location).is_solid())
            .map(|location| SearchState { location, depth: depth + 1 })
            .collect_vec()
    }

    fn shortest_distance(&self, from: Point2D<usize>, to: Point2D<usize>) -> Option<usize> {
        once(SearchState { location: from, depth: 0 })
            .collect::<VecDeque<_>>()
            .filter_duplicates()
            .recursive_find(|state| {
                if state.location == to { FindState::Result(state.depth) }
                else { FindState::Branch(self.moves_at(state)) }
            })
    }

    fn tiles_within_range(&self, from: Point2D<usize>, range: usize) -> usize {
        let mut filter = once(SearchState { location: from, depth: 0 })
            .collect::<VecDeque<_>>()
            .filter_duplicates();

        filter.recursive_iter(|state| {
            if state.depth == range { IterState::Leaf }
            else { IterState::Branch(self.moves_at(state)) }
        });

        filter.seen.len()
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let grid = Grid::new(input.parse::<usize>()?);
    let distance = grid
        .shortest_distance(Point2D::one(), Point2D(31, 39))
        .ok_or(NoSolution)?;

    Ok(Box::new(distance))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let grid = Grid::new(input.parse::<usize>()?);
    let tiles = grid.tiles_within_range(Point2D::one(), 50);

    Ok(Box::new(tiles))
}