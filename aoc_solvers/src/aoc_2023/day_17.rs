use std::{iter::once, collections::BinaryHeap, cmp::min, hash::Hash};

use ahash::{HashSet, HashSetExt};
use aoc_lib::{geometry::{CardinalDirection, Point2D, grid::{Grid, GridLike}, Directional, RotationDirection}, parsing::InvalidTokenError};
use aoc_runner_api::SolverResult;
use anyhow::{Result, Context};

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

#[derive(Clone, Copy)]
struct State {
    direction: CardinalDirection,
    location: Point2D<usize>,
    goal: Point2D<usize>,
    since_turn: u8,
    distance: u32
}

impl Eq for State {}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.direction == other.direction &&
        self.location == other.location &&
        self.since_turn == other.since_turn
    }
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.direction.hash(state);
        self.location.hash(state);
        self.since_turn.hash(state);
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let directions = CardinalDirection::all();

        self.location.manhattan_distance(self.goal)
            .cmp(&other.location.manhattan_distance(other.goal))
            .reverse()
            .then_with(|| self.distance.cmp(&other.distance))
            .reverse()
            .then_with(|| self.location.0.cmp(&other.location.0))
            .then_with(|| self.location.1.cmp(&other.location.1))
            .then_with(|| self.since_turn.cmp(&other.since_turn))
            .then_with(|| {
                directions.iter().position(|&dir| dir == self.direction).unwrap()
                    .cmp(&directions.iter().position(|&dir| dir == other.direction).unwrap())
            })
    }
}

impl State {
    fn next(self, grid: &Grid<Tile>, direction: CardinalDirection, min_turn: u8, max_turn: u8) -> Option<Self> {
        let location = self.location.checked_add::<isize>(direction.direction_vector())?;
        let distance = self.distance + grid.get(location)?.0;
        let since_turn = if direction == self.direction {
            (self.since_turn < max_turn).then_some(self.since_turn + 1)?
        } else {
            (self.since_turn >= min_turn).then_some(0)?
        };
        
        Some(State { direction, location, since_turn, distance, goal: self.goal })
    }
}

fn shortest_distance(grid: &Grid<Tile>, min_turn: u8, max_turn: u8) -> Result<u32> {
    let initial_state = State {
        location: Point2D::zero(),
        goal: grid.area().bottom_right(),
        direction: CardinalDirection::East,
        since_turn: 0,
        distance: 0,
    };

    let mut seen = HashSet::<State>::new();
    let mut queue: BinaryHeap<State> = once(initial_state).collect();
    let mut best: Option<u32> = None;

    while let Some(state) = queue.pop() {        
        if state.distance >= best.unwrap_or(u32::MAX) { continue; }
        
        if let Some(seen) = seen.get(&state) {
            if seen.distance <= state.distance { continue; }
        }
        
        seen.replace(state);

        if state.location == state.goal && state.since_turn >= min_turn {
            best = Some(min(best.unwrap_or(u32::MAX), state.distance));
            continue;
        }

        let states = [RotationDirection::Left, RotationDirection::Right].map(|direction| {
            state.next(grid, state.direction.rotate(direction), min_turn, max_turn)
        }).into_iter()
            .chain(once(state.next(grid, state.direction, min_turn, max_turn)))
            .flatten();

        queue.extend(states);
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