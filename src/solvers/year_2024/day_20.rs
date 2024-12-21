use std::{collections::VecDeque, iter::once};

use ahash::{HashMap, HashMapExt, HashSet};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use yuki::{iterators::{Enumerate2D, ExtraIter, SingleError}, spatial::{direction, Point}};

use crate::SolverResult;

#[derive(Debug)]
struct Track {
    walls: HashSet<Point<usize>>,
    start: Point<usize>,
    end: Point<usize>
}

#[derive(Debug, Clone, Copy)]
struct State {
    position: Point<usize>,
    distance: usize
}

impl Track {
    fn parse(input: &str) -> Result<Self> {
        let walls: HashSet<Point<usize>> = input
            .lines()
            .map(str::chars)
            .enumerate2d()
            .filter_map(|(pos, char)| (char == '#').then_some(pos))
            .collect();

        let start = input
            .lines()
            .map(str::chars)
            .enumerate2d()
            .filter_map(|(pos, char)| (char == 'S').then_some(pos))
            .single()
            .map_err(|err| match err {
                SingleError::More => anyhow!("More than one starting point is defined"),
                SingleError::None => anyhow!("No starting point is defined"),
            })?;

        let end = input
            .lines()
            .map(str::chars)
            .enumerate2d()
            .filter_map(|(pos, char)| (char == 'E').then_some(pos))
            .single()
            .map_err(|err| match err {
                SingleError::More => anyhow!("More than one end point is defined"),
                SingleError::None => anyhow!("No end point is defined"),
            })?;

        Ok(Self { walls, start, end })
    }

    fn distance_map(&self) -> HashMap<Point<usize>, usize> {
        let mut distances = HashMap::<Point<usize>, usize>::new();
        let initial_state = State { position: self.start, distance: 0 };
        let mut queue: VecDeque<State> = once(initial_state).collect();

        while let Some(state) = queue.pop_front() {
            distances.insert(state.position, state.distance);

            if state.position == self.end { break; }

            state.position
                .neighbours::<direction::Cardinal>()
                .filter(|pos| !self.walls.contains(pos))
                .filter(|pos| distances.get(pos).is_none_or(|&distance| state.distance + 1 < distance))
                .map(|position| State { position, distance: state.distance + 1 })
                .collect_into(&mut queue);
        }

        distances
    }

    fn count_cheats(&self, min_gain: usize) -> usize {
        let distances = self.distance_map();

        distances
            .iter()
            .flat_map(|(pos, distance)| pos
                .neighbours::<direction::Cardinal>()
                .flat_map(Point::neighbours::<direction::Cardinal>)
                .unique()
                .filter(|&neighbour| pos.manhattan_distance(neighbour) == 2)
                .filter_map(|pos| Some((pos, distances.get(&pos)?)))
                .filter(move |(_, neighbour_distance)| **neighbour_distance >= distance + min_gain + 2)
            )
            .count()
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let track = Track::parse(input)?;
    let cheats = track.count_cheats(100);

    Ok(Box::new(cheats))
}