use std::{iter::once, cmp::min, hash::Hash};

use ahash::{HashMap, HashMapExt};
use aoc_lib::{geometry::{CardinalDirection, Point2D, grid::{Grid, GridLike}, Directional, RotationDirection}, parsing::InvalidTokenError};
use crate::SolverResult;
use anyhow::{Result, Context};
use priority_queue::PriorityQueue;

#[derive(Clone, Copy)]
struct Tile(u32);

impl TryFrom<char> for Tile {
    type Error = InvalidTokenError<char>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        value.to_digit(10)
            .map(Self)
            .ok_or(InvalidTokenError(value))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    direction: CardinalDirection,
    location: Point2D<usize>,
    since_turn: u8
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Priority {
    distance: u32,
    tiles_to_goal: usize
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Priority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.tiles_to_goal.cmp(&other.tiles_to_goal)
            .reverse()
            .then_with(|| self.distance.cmp(&other.distance))
            .reverse()
    }
}

impl State {
    fn next(self, direction: CardinalDirection, min_turn: u8, max_turn: u8) -> Option<Self> {
        let location = self.location.checked_add::<isize>(direction.direction_vector())?;
        let since_turn = if direction == self.direction {
            (self.since_turn < max_turn).then_some(self.since_turn + 1)?
        } else {
            (self.since_turn >= min_turn).then_some(0)?
        };
        
        Some(Self { direction, location, since_turn })
    }
}

fn shortest_distance(grid: &Grid<Tile>, min_turn: u8, max_turn: u8) -> Result<u32> {
    let goal = grid.area().bottom_right();
    let initial_state = (State {
        location: Point2D::zero(),
        direction: CardinalDirection::East,
        since_turn: 0,
    }, Priority {
        tiles_to_goal: Point2D::<usize>::zero().manhattan_distance(goal),
        distance: 0,
    });

    let mut queue: PriorityQueue<State, Priority, ahash::RandomState> = once(initial_state).collect();
    let mut best: Option<u32> = None;
    let mut seen = HashMap::<State, u32>::new();

    while let Some((state, priority)) = queue.pop() {        
        if priority.distance >= best.unwrap_or(u32::MAX) { continue; }

        if state.location == goal && state.since_turn >= min_turn {
            best = Some(min(best.unwrap_or(u32::MAX), priority.distance));
            continue;
        }

        if let Some(&shortest) = seen.get(&state) {
            if shortest <= priority.distance { continue; }
        }

        seen.insert(state, priority.distance);

        let states = [RotationDirection::Left, RotationDirection::Right].map(|direction| {
            state.next(state.direction.rotate(direction), min_turn, max_turn)
        }).into_iter()
            .chain(once(state.next(state.direction, min_turn, max_turn)))
            .flatten()
            .filter_map(|state| Some((state, Priority {
                distance: priority.distance + grid.get(state.location)?.0,
                tiles_to_goal: state.location.manhattan_distance(goal)
            })));

        for (state, priority) in states {
            queue.push_increase(state, priority);
        }
    }

    best.context("No path to the goal exists")
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let grid: Grid<Tile> = Grid::parse(input)?;
    let distance = shortest_distance(&grid, 0, 2)?;

    Ok(Box::new(distance))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let grid: Grid<Tile> = Grid::parse(input)?;
    let distance = shortest_distance(&grid, 3, 9)?;

    Ok(Box::new(distance))
}