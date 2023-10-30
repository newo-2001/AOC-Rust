use ahash::{HashSet, HashSetExt};
use aoc_lib::{geometry::{grid::{BitGrid, Grid, GridLike}, CardinalDirection, Point2D}, math::Bit};
use aoc_runner_api::SolverResult;
use hex::decode;
use itertools::Itertools;
use bitvec::{prelude::Msb0, vec::BitVec};

use super::knot_hash::hash;

fn parse_grid(input: &str) -> Grid<Bit> {
    let bits = (0..128u8).map(|n| {
        let hash = decode(hash(format!("{input}-{n}"))).unwrap();
        
        BitVec::<u8, Msb0>::from_vec(hash)
            .into_iter()
            .map(Bit::from)
            .collect_vec()
    }).collect_vec();

    Grid::new(bits).unwrap()
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve_part_1(input: &str) -> SolverResult {
    let grid = parse_grid(input);
    Ok(Box::new(grid.pop_count()))
}

#[allow(clippy::unnecessary_wraps)]
pub fn solve_part_2(input: &str) -> SolverResult {
    let grid = parse_grid(input);

    // TODO: Clean this up at some point
    let mut regions = 0;
    let mut seen = HashSet::<Point2D<usize>>::new();
    for location in grid.area() {
        if !seen.contains(&location) && grid[location].is_enabled() { regions += 1; }

        let mut todo = vec![location];
        while let Some(location) = todo.pop() {
            let is_enabled = grid.get(location).map_or(false, |state| state.is_enabled());
            if !is_enabled || seen.contains(&location) { continue; }

            todo.extend(location.neighbours(CardinalDirection::all()));
            seen.insert(location);
        }
    }
    
    Ok(Box::new(regions))
}