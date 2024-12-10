use ahash::{HashSet, HashSetExt};
use anyhow::{Context, Result};
use yuki::{iterators::ExtraIter, spatial::{direction, Matrix, Point}};

use crate::SolverResult;

fn parse_map(input: &str) -> Result<Matrix<u32>> {
    let map = input
        .lines()
        .map(|line| line
            .chars()
            .map(|char| char.to_digit(10))
            .collect()
        )
        .collect::<Option<Vec<Vec<_>>>>()
        .context("Failed to parse digit")?
        .into_iter()
        .try_collecting()?;
    
    Ok(map)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Metric {
    Score,
    Rating
}

fn measure_map(map: &Matrix<u32>, metric: Metric) -> u32 {
    let mut score = 0;
    let mut seen = HashSet::<Point<usize>>::new();
    let mut queue: Vec<(Point<usize>, u32)> = map
        .enumerate()
        .filter(|(_, height)| **height == 0)
        .map(|(point, &value)| (point, value))
        .collect();

    while let Some((pos, height)) = queue.pop() {
        if metric == Metric::Score && !seen.insert(pos) { continue; };

        if height == 9 {
            score += 1;
            continue;
        }

        pos
            .neighbours::<direction::Cardinal>()
            .filter_map(|pos| Some((pos, *map.get(pos)?)))
            .filter(|(_, neighbour_height)| *neighbour_height == height + 1)
            .collect_into(&mut queue);
    }

    score
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let map = parse_map(input)?;
    Ok(Box::new(measure_map(&map, Metric::Score)))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let map = parse_map(input)?;
    Ok(Box::new(measure_map(&map, Metric::Rating)))
}