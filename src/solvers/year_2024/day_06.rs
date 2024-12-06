use ahash::{HashSet, HashSetExt};
use yuki::{iterators::{ExtraIter, SingleError}, spatial::{direction::{self, Directions}, Point}, tuples::fst};
use anyhow::{anyhow, Result};

use crate::SolverResult;

type State = (Point<i32>, direction::Cardinal);

fn parse_grid(input: &str) -> Result<(State, HashSet<Point<i32>>)> {
    let iter = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line
            .chars()
            .enumerate()
            .map(move |(x, char)| (Point { x: x as i32, y: y as i32 }, char))
        );
    
    let grid = iter
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

    Ok((initial_state, grid))
}

fn walk(width: i32, height: i32, mut state: State, grid: &HashSet<Point<i32>>) -> Option<HashSet<Point<i32>>> {
    let mut seen: HashSet<Point<i32>> = HashSet::new();
    let mut seen_states: HashSet<State> = HashSet::new();

    while (0..width).contains(&state.0.x) && (0..height).contains(&state.0.y) {
        let (pos, direction) = state;
        seen.insert(pos);
        if !seen_states.insert(state) { return None; }

        let mut next_pos = pos.add_signed(direction.vector()).unwrap();
        let mut next_direction = direction;

        let mut i = 0;
        while grid.contains(&next_pos) {
            next_direction = next_direction.turn(direction::Rotation::Clockwise);
            next_pos = pos.add_signed(next_direction.vector()).unwrap();
        };

        state = (next_pos, next_direction);
    }

    Some(seen)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let height = input.lines().count() as i32;
    let width = input.lines().next().map(|line| line.chars().count()).unwrap_or_default() as i32;

    let (state, grid) = parse_grid(input)?;
    let locations = walk(width, height, state, &grid).unwrap();

    Ok(Box::new(locations.len()))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let height = input.lines().count() as i32;
    let width = input.lines().next().map(|line| line.chars().count()).unwrap_or_default() as i32;

    let (state, mut grid) = parse_grid(input)?;
    let mut locations = walk(width, height, state, &grid).unwrap();

    locations.remove(&state.0);

    let mut loops: u32 = 0;
    for location in locations {
        grid.insert(location);

        if walk(width, height, state, &grid).is_none() { loops += 1; }

        grid.remove(&location);
    }

    Ok(Box::new(loops))
}