use std::fmt::Debug;

use anyhow::{Context, Result, anyhow};
use aoc_lib::{math::Bit, parsing::{ParseError, TextParser}, iteration::ExtraIter, geometry::Axis};
use aoc_runner_api::SolverResult;
use itertools::Itertools;
use nom::{multi::{separated_list1, count, many1}, character::complete::{anychar, line_ending}, combinator::map_res};
use rayon::iter::{ParallelIterator, IntoParallelRefMutIterator};

fn parse(input: &str) -> Result<Vec<Vec<Vec<Bit>>>, ParseError> {
    separated_list1(
        count(line_ending, 2),
        separated_list1(
            line_ending,
            many1(map_res(anychar, Bit::try_from))
        )
    ).run(input)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Mirror {
    axis: Axis,
    offset: usize
}

impl Mirror {
    fn summary(&self) -> usize {
        match self.axis {
            Axis::Horizontal => self.offset * 100,
            Axis::Vertical => self.offset
        }
    }
}

fn reflection_locations<T: PartialEq>(pattern: &[T]) -> impl Iterator<Item=usize> {
    fn reflect<T: PartialEq>(pattern: &[T]) -> Option<usize> {
        if pattern.len() < 2 { return None; }

        let pivot = pattern.len() / 2;
        let start = &pattern[..pivot];
        let end = &pattern[pivot..pivot * 2];
        
        let reflection = start.iter()
            .zip(end.iter().rev())
            .all(|(a, b)| a == b);

        if reflection { Some(pivot) }
        else { reflect(&pattern[..pattern.len() - 1]) }
    }

    [
        reflect(pattern),
        reflect(&pattern.iter().rev().collect_vec())
            .map(|offset| pattern.len() - offset)
    ].into_iter()
        .flatten()
        .dedup()
}

fn find_mirrors<T: PartialEq>(pattern: &[Vec<T>]) -> impl Iterator<Item=Mirror> + '_ {
    let transposed = pattern.iter()
        .transpose()
        .collect_vec();

    reflection_locations(&transposed)
        .map(|offset| Mirror { offset, axis: Axis::Vertical })
        .chain(reflection_locations(pattern).map(|offset| Mirror { offset, axis: Axis::Horizontal}))
}

fn find_smudgy_mirror(pattern: &mut [Vec<Bit>]) -> Result<Mirror> {
    let clean_mirror: Mirror = find_mirrors(pattern)
        .single()
        .context("Failed to identify clean mirror")?;
    
    // I will find a better way to do this
    // I just want to go to bed
    (0..pattern.len()).find_map(|row_index| {
        (0..pattern[0].len()).find_map(|col_index| {
            let row = &mut pattern[row_index];
            let tile = &mut row[col_index];
            *tile = tile.invert();

            let result = find_mirrors(pattern)
                .filter(|&mirror| mirror != clean_mirror)
                .single()
                .ok();

            let row = &mut pattern[row_index];
            let tile = &mut row[col_index];
            *tile = tile.invert();

            result
        })
    }).ok_or_else(|| anyhow!("{:?}", &pattern))
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let total_points = parse(input)?
        .into_iter()
        .map(|pattern| {
            find_mirrors(&pattern)
                .single()
                .context("Failed to identify mirror")
        }).collect::<Result<Vec<Mirror>>>()?
        .iter()
        .sum_by(Mirror::summary);

    Ok(Box::new(total_points))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let total_points = parse(input)?
        .par_iter_mut()
        .map(|pattern| find_smudgy_mirror(pattern))
        .collect::<Result<Vec<Mirror>>>()?
        .iter()
        .sum_by(Mirror::summary);

    Ok(Box::new(total_points))
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use crate::aoc_2023::day_13::reflection_locations;

    #[test]
    fn reflections() {
        assert_eq!(Vec::<usize>::new(), reflection_locations(&[1, 2, 3]).collect_vec());
        assert_eq!(Vec::<usize>::new(), reflection_locations(&[1, 2, 3, 2, 1, 0]).collect_vec());
        assert_eq!(vec![1], reflection_locations(&[1, 1]).collect_vec());
        assert_eq!(vec![2], reflection_locations(&[1, 2, 2]).collect_vec());
        assert_eq!(vec![4], reflection_locations(&[-1, 1, 2, 3, 3, 2, 1]).collect_vec());
        assert_eq!(vec![3], reflection_locations(&[1, 2, 3, 3, 2, 1, 0, -1, -2, -3, -4, -5, -6]).collect_vec());
        assert_eq!(vec![8], reflection_locations(&[1, 2, 3, 4, 5, 3, 2, 1, 1]).collect_vec());
    }
}