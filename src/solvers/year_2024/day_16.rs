use std::{hash::Hash, iter::once};

use crate::SolverResult;
use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use anyhow::{anyhow, Result};
use priority_queue::PriorityQueue;
use yuki::{errors::NoSolution, iterators::{Enumerate2D, ExtraIter, SingleError}, spatial::{direction::{self, Directions}, Point}};

#[derive(Debug)]
struct Grid {
    start: Point<usize>,
    end: Point<usize>,
    tiles: HashSet<Point<usize>>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    position: Point<usize>,
    direction: direction::Cardinal,
    path: Vec<Point<usize>>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Score(u32);

impl Ord for Score {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Grid {
    fn parse(input: &str) -> Result<Self> {
        let chars = input.lines().map(str::chars);
        let tiles: HashSet<Point<usize>> = chars
            .clone()
            .enumerate2d()
            .filter_map(|(pos, char)| match char {
                '#' => Some(Ok(pos)),
                '.' | 'S' | 'E' => None,
                _ => Some(Err(anyhow!("Invalid character found in grid: {char}")))
            })
            .collect::<Result<_>>()?;

        let (start, _) = chars
            .clone()
            .enumerate2d()
            .filter(|(_, char)| *char == 'S')
            .single()
            .map_err(|err| match err {
                SingleError::More => anyhow!("More than one starting point in input"),
                SingleError::None => anyhow!("No starting point in input")
            })?;

        let (end, _) = chars
            .enumerate2d()
            .filter(|(_, char)| *char == 'E')
            .single()
            .map_err(|err| match err {
                SingleError::More => anyhow!("More than one end point in input"),
                SingleError::None => anyhow!("No end point in input")
            })?;

        Ok(Self { start, end, tiles })
    }

    fn lowest_score(&self) -> Result<(u32, usize), NoSolution> {
        let mut seen = HashMap::<(Point<usize>, direction::Cardinal), Score>::new();
        let mut winning_path = HashSet::<Point<usize>>::new();
        let mut minimal_score: Option<u32> = None;

        let mut queue: PriorityQueue<State, Score> = once((
            State {
                position: self.start,
                direction: direction::Cardinal::East,
                path: once(self.start).collect(),
            },
            Score(0)
        )).collect();

        while let Some((state, Score(score))) = queue.pop() && score <= minimal_score.unwrap_or(u32::MAX) {
            if state.position == self.end {
                winning_path.extend(state.path);
                let _ = minimal_score.insert(score);
                continue;
            }

            let forward = state.position
                .add_signed(state.direction.vector())
                .and_then(|position| (!self.tiles.contains(&position)).then(|| {
                    let mut path = state.path.clone();
                    path.push(state.position);

                    let state = State {
                        position,
                        direction: state.direction,
                        path
                    };

                    (state, Score(score + 1))
                }));
            
            let directions = [
                state.direction.turn(direction::Rotation::Clockwise),
                state.direction.turn(direction::Rotation::CounterClockwise)
            ];
        
            let states = directions
                .map(|direction| Some((
                    State {
                        position: state.position,
                        direction,
                        path: state.path.clone()
                    },
                    Score(score + 1000)
                )))
                .into_iter()
                .chain(once(forward))
                .flatten();

            for (state, score) in states {
                let seen_state = (state.position, state.direction);
                if seen.get(&seen_state).is_none_or(|&other| score.0 <= other.0) {
                    seen.insert(seen_state, score);
                    queue.push(state, score);
                }
            }
        }

        minimal_score
            .map(|score| (score, winning_path.len() + 1))
            .ok_or(NoSolution)
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let grid = Grid::parse(input)?;
    Ok(Box::new(grid.lowest_score()?.0))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let grid = Grid::parse(input)?;
    Ok(Box::new(grid.lowest_score()?.1))
}