use std::iter::repeat;

use ahash::{HashMap, HashMapExt};
use aoc_lib::{parsing::{TextParserResult, ParseError, parse_lines, usize, TextParser}, iteration::ExtraIter};
use crate::SolverResult;
use itertools::Itertools;
use nom::{character::complete::{anychar, char}, Parser, combinator::map_res, sequence::separated_pair, multi::{many1, separated_list1}};
use anyhow::{bail, Result};
use rayon::iter::{IntoParallelIterator, ParallelIterator, IntoParallelRefIterator};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum HotSpring {
    Broken,
    Operational,
    Unknown
}

impl HotSpring {
    fn parse(input: &str) -> TextParserResult<'_, Self> {
        map_res(anychar, |char| Ok(match char {
            '.' => Self::Operational,
            '#' => Self::Broken,
            '?' => Self::Unknown,
            _ => bail!("Invalid hot spring: '{char}'")
        })).parse(input)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Row {
    hot_springs: Vec<HotSpring>,
    broken_groups: Vec<usize>
}

impl Row {
    fn parse(input: &str) -> Result<Self, ParseError> {
        separated_pair(
            many1(HotSpring::parse),
            char(' '),
            separated_list1(char(','), usize)
        ).map(|(hot_springs, broken_groups)| Self {
            hot_springs, broken_groups
        }).run(input)
    }

    fn arrangement_count(&self) -> usize {
        arrangement_count(&self.hot_springs, &self.broken_groups, &mut HashMap::new())
    }

    fn unfold(self) -> Self {
        let broken_groups = self.broken_groups.repeat(5);
        let hot_springs = Iterator::intersperse(
            repeat(self.hot_springs).take(5),
            vec![HotSpring::Unknown]
        ).flatten().collect_vec();

        Self { hot_springs, broken_groups }
    }
}

fn arrangement_count(
    hot_springs: &[HotSpring],
    broken_groups: &[usize],
    memo: &mut HashMap<(Vec<HotSpring>, usize), usize>
) -> usize {
    // Check if we have seen this arrangement before
    let state = (hot_springs.to_vec(), broken_groups.len());
    if let Some(&result) = memo.get(&state) { return result; }

    // Remove all operational hot springs from the start
    let hot_springs: &[HotSpring] = hot_springs.iter()
        .position(|&hot_spring| hot_spring != HotSpring::Operational)
        .map_or(&[], |not_operational| &hot_springs[not_operational..]);

    // If there are no groups remaining, return one if there are no remaining broken hot springs and zero otherwise
    let is_done = || hot_springs.iter().none(|&hot_spring| hot_spring == HotSpring::Broken).into();
    let result = broken_groups.split_first().map_or_else(is_done, |(&group_size, broken_groups_tail)| {
        // Extract the next group from the sequence
        // If there are not enough items to form a group, then this branch contains no solutions
        hot_springs.get(..group_size).map_or(0, |group| {
            // Check if the group contains an unknown hot_spring
            if let Some(unknown_index) = group.iter().position(|&x| x == HotSpring::Unknown) {
                [
                    HotSpring::Broken,
                    HotSpring::Operational
                ].into_iter().sum_by(|replacement| {
                    let mut hot_springs = hot_springs.to_vec();
                    hot_springs[unknown_index] = replacement;

                    arrangement_count(&hot_springs, broken_groups, memo)
                })
            } else {
                // All the hot springs in the group have to be broken
                if !group.iter().all(|&x| x == HotSpring::Broken) { return 0; }

                let hot_springs = match hot_springs.get(group_size) {
                    // If the next hot spring is broken, the group is larger, so we don't have a match
                    Some(HotSpring::Broken) => return 0,
                    // If the next hot spring is unknown, we know it must be operational to make this group valid
                    Some(HotSpring::Unknown) => &hot_springs[group_size + 1..],
                    Some(HotSpring::Operational) | None => &hot_springs[group_size..]
                };

                // We have matched this group and will recurse down to the next one
                arrangement_count(hot_springs, broken_groups_tail, memo)
            }
        })
    });

    // Store the arrangement in the memo for future reference
    memo.insert(state, result);
    result
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let arrangement_sum: usize = parse_lines(Row::parse, input)?
        .par_iter()
        .map(Row::arrangement_count)
        .sum();

    Ok(Box::new(arrangement_sum))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let arrangement_sum: usize = parse_lines(Row::parse, input)?
        .into_par_iter()
        .map(|row| row.unfold().arrangement_count())
        .sum();

    Ok(Box::new(arrangement_sum))
}