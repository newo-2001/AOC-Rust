use std::iter::repeat;

use ahash::{HashMap, HashMapExt};
use aoc_lib::{parsing::{TextParserResult, InvalidTokenError, ParseError, parse_lines, usize, TextParser}, iteration::ExtraIter};
use aoc_runner_api::SolverResult;
use bitvec::{index::BitIdx, prelude::Lsb0, store::BitStore};
use itertools::Itertools;
use nom::{character::complete::{anychar, char}, Parser, combinator::map_res, sequence::separated_pair, multi::{many1, separated_list1}};
use anyhow::Result;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum HotSpring {
    Broken,
    Operational,
}

impl HotSpring {
    fn parse(input: &str) -> TextParserResult<Option<HotSpring>> {
        map_res(anychar, |char| Ok(match char {
            '.' => Some(Self::Operational),
            '#' => Some(Self::Broken),
            '?' => None,
            _ => return Err(InvalidTokenError(char))
        })).parse(input)
    }
}

fn group_permutations<'a>(group: &'a [Option<HotSpring>], tail: &'a [Option<HotSpring>]) -> impl Iterator<Item=Vec<Option<HotSpring>>> + 'a {
    let unknowns = u32::try_from(group.iter().count_where(Option::is_none)).unwrap();
    let permutations: u32 = 2u32.pow(unknowns);

    // Associate every permutation with a number
    (0..permutations).map(|replacements| {
        let mut index: BitIdx<u32> = BitIdx::MIN;
        group.iter().map(|hot_spring| {
            hot_spring.or_else(|| {
                let current_index = index;
                (index, _) = index.next();

                // This number is a bit vector containing the locations of the broken hot springs
                if replacements.get_bit::<Lsb0>(current_index) { Some(HotSpring::Broken) }
                else { Some(HotSpring::Operational) }
            })
        }).chain(tail.iter().copied()).collect_vec()
    })
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Row {
    hot_springs: Vec<Option<HotSpring>>,
    counts: Vec<usize>
}

impl Row {
    fn parse(input: &str) -> Result<Self, ParseError> {
        separated_pair(
            many1(HotSpring::parse),
            char(' '),
            separated_list1(char(','), usize)
        ).map(|(hot_springs, counts)| Row {
            hot_springs, counts
        }).run(input)
    }

    fn arrangement_count(self, memo: &mut HashMap<Row, usize>) -> usize {
        // Check if we have seen this arrangement before
        if let Some(&result) = memo.get(&self) { return result; }

        // Remove all operational hot springs from the start
        let hot_springs: &[Option<HotSpring>] = self.hot_springs.iter()
            .position(|hot_spring| !matches!(hot_spring, Some(HotSpring::Operational)))
            .map_or(&[], |first_broken| &self.hot_springs[first_broken..]);

        // If there are no groups remaining, return one if there are no remaining hot springs and zero otherwise
        let empty = || hot_springs.iter().all(|hot_spring| matches!(hot_spring, None | Some(HotSpring::Operational))).into();
        let result = self.counts.split_first().map_or_else(empty, |(&group_size, tail_counts)| {
            // Extract the next group from the sequence
            // If there are not enough items to form a group, then this branch contains no solutions
            hot_springs.get(..group_size).map_or(0, |group| {
                // Check if the entire group is materialized
                if let Some(group) = group.iter().copied().collect::<Option<Vec<_>>>() {
                    // All the hot springs in the group have to be broken
                    if !group.iter().all(|&hot_spring| hot_spring == HotSpring::Broken) { return 0; }

                    let mut hot_springs = &hot_springs[group_size..];
                    match hot_springs.first() {
                        // If the next hot spring is broken, the group is larger, so we don't have a match
                        Some(Some(HotSpring::Broken)) => return 0,
                        // If the next hot spring is unknown, we know it must be operational to make this group valid
                        Some(None) => hot_springs = &hot_springs[1..],
                        Some(Some(HotSpring::Operational)) | None => {}
                    }

                    // We have matched this group and will recurse down to the next one
                    Row {
                        counts: tail_counts.to_vec(),
                        hot_springs: hot_springs.to_vec()
                    }.arrangement_count(memo)
                } else {
                    // If the group wasn't materialized we will materialize all permutations and recurse them down
                    group_permutations(group, &hot_springs[group_size..])
                        .map(|hot_springs| {
                            Row { counts: self.counts.clone(), hot_springs }.arrangement_count(memo)
                        }).sum()
                }
            })
        });

        // Store the arrangement in the memo for future reference
        memo.insert(self, result);
        result
    }

    fn unfold(self) -> Self {
        let counts = self.counts.repeat(5);
        let hot_springs = Iterator::intersperse(
            repeat(self.hot_springs).take(5),
            vec![None]
        ).flatten().collect_vec();

        Row { hot_springs, counts }
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let arrangement_sum: usize = parse_lines(Row::parse, input)?
        .into_par_iter()
        .map(|row: Row| row.arrangement_count(&mut HashMap::new()))
        .sum();

    Ok(Box::new(arrangement_sum))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let arrangement_sum: usize = parse_lines(Row::parse, input)?
        .into_par_iter()
        .map(|row| row.unfold().arrangement_count(&mut HashMap::new()))
        .sum();

    Ok(Box::new(arrangement_sum))
}