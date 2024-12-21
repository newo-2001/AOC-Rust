use std::{collections::VecDeque, iter::once};

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use anyhow::{anyhow, Result};
use yuki::{iterators::{Enumerate2D, ExtraIter, SingleError}, spatial::{direction, Point}};

use crate::SolverResult;

#[derive(Debug)]
struct Track {
    walls: HashSet<Point<usize>>,
    start: Point<usize>,
    end: Point<usize>
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

    fn count_cheats(&self, cheat_length: usize, min_gain: usize) -> usize {
        let distances = self.distance_map();
        let cheat_offsets: Vec<(Point<isize>, usize)> = cheats(cheat_length).collect();

        distances
            .iter()
            .flat_map(|(pos, distance)| cheat_offsets
                .iter()
                .filter_map(|&(offset, cheat_length)| Some((pos.add_signed(offset)?, cheat_length)))
                .filter_map(|(pos, cheat_length)| Some((distances.get(&pos)?, cheat_length)))
                .filter(move |&(end_distance, cheat_length)| *end_distance >= distance + min_gain + cheat_length)
            )
            .count()
    }
}

fn cheats(cheat_length: usize) -> impl Iterator<Item=(Point<isize>, usize)> {
    let mut dest = HashMap::<Point<isize>, usize>::new();
    let mut seen = HashSet::<(Point::<isize>, usize)>::new();
    let mut queue: Vec<(Point::<isize>, usize)> = once((Point::zero(), 0)).collect();

    while let Some((position, distance)) = queue.pop() {
        if dest.get(&position).is_none_or(|&dist| dist > distance) {
            dest.insert(position, distance);
        }

        if distance == cheat_length { continue }

        position
            .neighbours::<direction::Cardinal>()
            .map(|position| (position, distance + 1 ))
            .filter(|&state| seen.insert(state))
            .collect_into(&mut queue);
    }

    dest.into_iter()
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let track = Track::parse(input)?;
    let cheats = track.count_cheats(2, 100);

    Ok(Box::new(cheats))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let track = Track::parse(input)?;
    let cheats = track.count_cheats(20, 100);

    Ok(Box::new(cheats))
}