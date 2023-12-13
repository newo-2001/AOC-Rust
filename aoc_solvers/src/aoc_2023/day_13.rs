use std::fmt::Debug;

use anyhow::Context;
use aoc_lib::{math::Bit, parsing::{ParseError, TextParser}, iteration::ExtraIter, geometry::Axis};
use aoc_runner_api::SolverResult;
use itertools::Itertools;
use nom::{multi::{separated_list1, count, many1}, character::complete::{anychar, line_ending}, combinator::map_res};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

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

fn reflection_location<T: PartialEq>(pattern: &[T], exclude: Option<usize>) -> Option<usize> {
    fn reflect<T: PartialEq>(pattern: &[T], exclude: Option<usize>) -> Option<usize> {
        if pattern.len() < 2 { return None }

        let pivot = pattern.len() / 2;
        let start = &pattern[..pivot];
        let end = &pattern[pivot..pivot * 2];
        
        let reflection = start.iter()
            .zip(end.iter().rev())
            .all(|(a, b)| a == b);

        if !reflection || exclude.is_some_and(|exclude| pivot == exclude) {
            reflect(&pattern[..pattern.len() - 1], exclude)
        } else { Some (pivot) }
    }

    reflect(pattern, exclude).or_else(|| {
        let exclude = exclude.map(|x| pattern.len() - x);
        reflect(&pattern.iter().rev().collect_vec(), exclude)
            .map(|offset| pattern.len() - offset)
    })
}

fn find_mirror<T: PartialEq>(pattern: &[Vec<T>], exclude: Option<Mirror>) -> Option<Mirror> {
    let exclude_horizontal = exclude.and_then(|mirror| {
        (mirror.axis == Axis::Horizontal).then_some(mirror.offset)
    });

    if let Some(offset) = reflection_location(pattern, exclude_horizontal) {
        return Some(Mirror { offset, axis: Axis::Horizontal });
    }

    let transposed = pattern.iter()
        .transpose()
        .collect_vec();

    let exclude_vertical = exclude.and_then(|mirror| {
        (mirror.axis == Axis::Vertical).then_some(mirror.offset)
    });

    reflection_location(&transposed, exclude_vertical)
        .map(|offset| Mirror { offset, axis: Axis::Vertical })
}

fn find_smudgy_mirror(mut pattern: Vec<Vec<Bit>>) -> Option<Mirror> {
    let actual_mirror = find_mirror(&pattern, None)?;

    // I will find a better way to do this
    // I just want to go to bed
    (0..pattern.len()).find_map(|row_index| {
        (0..pattern[0].len()).find_map(|col_index| {
            let row = &mut pattern[row_index];
            let tile = &mut row[col_index];
            *tile = tile.invert();

            if let Some(mirror) = find_mirror(&pattern, Some(actual_mirror)) {
                return Some(mirror);
            }

            let row = &mut pattern[row_index];
            let tile = &mut row[col_index];
            *tile = tile.invert();

            None
        })
    })
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let total_points = parse(input)?
        .into_iter()
        .map(|pattern| find_mirror(&pattern, None))
        .collect::<Option<Vec<Mirror>>>()
        .context("Input contained a pattern without a mirror")?
        .iter()
        .sum_by(Mirror::summary);

    Ok(Box::new(total_points))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let total_points = parse(input)?
        .into_par_iter()
        .map(find_smudgy_mirror)
        .collect::<Option<Vec<Mirror>>>()
        .context("Input contained a pattern without a mirror")?
        .iter()
        .sum_by(Mirror::summary);

    Ok(Box::new(total_points))
}

#[cfg(test)]
mod tests {
    use crate::aoc_2023::day_13::reflection_location;

    #[test]
    fn reflections() {
        assert_eq!(None, reflection_location(&[1, 2, 3], None));
        assert_eq!(Some(1), reflection_location(&[1, 1], None));
        assert_eq!(Some(2), reflection_location(&[1, 2, 2], None));
        assert_eq!(None, reflection_location(&[1, 2, 3, 2, 1, 0], None));
        assert_eq!(Some(4), reflection_location(&[-1, 1, 2, 3, 3, 2, 1], None));
        assert_eq!(Some(3), reflection_location(&[1, 2, 3, 3, 2, 1, 0, -1, -2, -3, -4, -5, -6], None));
        assert_eq!(Some(8), reflection_location(&[1, 2, 3, 4, 5, 3, 2, 1, 1], None));
    }
}