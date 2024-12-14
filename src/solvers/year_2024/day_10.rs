use std::iter::once;

use anyhow::{Context, Result};
use itertools::Itertools;
use yuki::{iterators::{Enumerate2D, ExtraIter}, spatial::{direction, Matrix, Point}};

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

fn trail_heads(map: &Matrix<u32>) -> impl Iterator<Item=(Point<usize>, u32)> {
    map
        .iter_rows()
        .enumerate2d()
        .filter(|(_, height)| **height == 0)
        .map(|(point, &value)| (point, value))
}

struct TrailWalker<'a> {
    map: &'a Matrix<u32>,
    queue: Vec<(Point<usize>, u32)>
}

impl<'a> TrailWalker<'a> {
    fn new(map: &'a Matrix<u32>, trail_head: (Point<usize>, u32)) -> Self {
        Self { map, queue: once(trail_head).collect() }
    }
}

impl Iterator for TrailWalker<'_> {
    type Item = Point<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((pos, height)) = self.queue.pop() {
            if height == 9 { return Some(pos) }

            pos
                .neighbours::<direction::Cardinal>()
                .filter_map(|pos| Some((pos, *self.map.get(pos)?)))
                .filter(|(_, neighbour_height)| *neighbour_height == height + 1)
                .collect_into(&mut self.queue);
        }

        None
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let map = parse_map(input)?;
    let score: usize = trail_heads(&map)
        .map(|head| TrailWalker::new(&map, head).unique().count())
        .sum();

    Ok(Box::new(score))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let map = parse_map(input)?;
    let rating: usize = trail_heads(&map)
        .map(|head| TrailWalker::new(&map, head).count())
        .sum();

    Ok(Box::new(rating))
}