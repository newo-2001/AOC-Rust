use ahash::{HashSet, HashSetExt};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use yuki::{iterators::{ExtraIter, SingleError}, spatial::{direction::{self, Directions}, Area, Point}, tuples::fst};
use anyhow::{anyhow, Result};

use crate::SolverResult;

type State = (Point<usize>, direction::Cardinal);

#[derive(Clone)]
struct Grid {
    objects: HashSet<Point<usize>>,
    area: Area<usize>
}

fn parse_grid(input: &str) -> Result<(Grid, State)> {
    let iter = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line
            .chars()
            .enumerate()
            .map(move |(x, char)| (Point { x, y }, char))
        );
    
    let objects = iter
        .clone()
        .filter(|(_, char)| *char == '#')
        .map(fst)
        .collect();

    let initial_state = iter
        .filter_map(|(pos, char)| {
            match char {
                '^' => Some(direction::Cardinal::North),
                'v' => Some(direction::Cardinal::South),
                '<' => Some(direction::Cardinal::West),
                '>' => Some(direction::Cardinal::East),
                _ => None
            }
            .map(|direction| (pos, direction))
        })
        .single()
        .map_err(|err| match err {
            SingleError::None => anyhow!("No starting position found in input"),
            SingleError::More => anyhow!("Multiple starting positions found in input"),
        })?;

    let height = input.lines().count();
    let width = input.lines()
        .next()
        .unwrap_or_default()
        .chars()
        .count();

    let grid = Grid { objects, area: (width, height).into() };
    Ok((grid, initial_state))
}

fn walk(grid: &Grid, mut state: State) -> Option<HashSet<Point<usize>>> {
    let mut seen: HashSet<Point<usize>> = HashSet::new();
    let mut seen_states: HashSet<State> = HashSet::new();

    while grid.area.contains(state.0) {
        let (pos, direction) = state;
        seen.insert(pos);
        if !seen_states.insert(state) { return None; }

        let mut next_direction = direction;
        let Some(mut next_pos) = pos.add_signed(direction.vector()) else { break };

        while grid.objects.contains(&next_pos) {
            next_direction = next_direction.turn(direction::Rotation::Clockwise);
            next_pos = pos.add_signed(next_direction.vector()).unwrap();
        };

        state = (next_pos, next_direction);
    }

    Some(seen)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let (grid, state) = parse_grid(input)?;
    let locations = walk(&grid, state).unwrap();

    Ok(Box::new(locations.len()))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let (grid, state) = parse_grid(input)?;
    let mut locations = walk(&grid, state).unwrap();
    locations.remove(&state.0);

    let loops = locations
        .into_par_iter()
        .filter(|&location| {
            let mut grid = grid.clone();
            grid.objects.insert(location);

            walk(&grid, state).is_none()
        })
        .count();

    Ok(Box::new(loops))
}