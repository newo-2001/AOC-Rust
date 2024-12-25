use itertools::Itertools;
use yuki::{iterators::ExtraIter, spatial::Matrix};
use anyhow::Result;

use crate::SolverResult;

fn is_valid(lock: &[usize; 5], key: &[usize; 5]) -> bool {
    lock
        .iter()
        .zip(key)
        .all(|(n, m)| n + m <= 5)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let (keys, locks): (Vec<_>, Vec<_>) = input
        .split("\n\n")
        .map(|grid| {
            let heights: [usize; 5] = grid
                .lines()
                .map(str::chars)
                .try_collecting::<Matrix<char>>()?
                .into_cols()
                .map(|col| col
                    .into_iter()
                    .filter(|&char| char == '#')
                    .count() - 1
                )
                .collect_vec()
                .try_into()
                .unwrap();

            let is_lock = grid.lines().next().unwrap().chars().all(|c| c == '#');

            Ok((heights, is_lock))
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .partition(|(_, is_lock)| *is_lock);

    let combinations = locks
        .into_iter()
        .cartesian_product(keys)
        .filter(|((lock, _), (key, _))| is_valid(lock, key))
        .count();

    Ok(Box::new(combinations))
}