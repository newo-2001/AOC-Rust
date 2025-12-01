use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;
use nom::{character::complete::{char, line_ending, u32}, combinator::map, multi::{count, separated_list1}, sequence::separated_pair, Parser};
use yuki::parsing::{combinators::lines, ParserExt, ParsingResult};

use crate::SolverResult;

type Rules = HashMap<u32, Vec<u32>>;
struct Update(Vec<u32>);

fn parse_manual(input: &str) -> ParsingResult<'_, (Rules, Vec<Update>)> {
    separated_pair(
        map(
            lines(
                separated_pair(u32, char('|'), u32)
            ),
            |rules| rules
                .into_iter()
                .into_group_map()
        ),
        count(line_ending, 2),
        lines(
            map(
                separated_list1(char(','), u32),
                Update
            )
        )
    )
    .parse(input)
}

impl Update {
    fn is_ordered(&self, rules: &Rules) -> bool {
        self.0
            .iter()
            .enumerate()
            .all(|(i, num)| rules
                .get(num)
                    .is_none_or(|dependant| !dependant
                    .iter()
                    .any(|dependant| self.0[..i].contains(dependant)))
            )
    }

    fn order(&mut self, rules: &Rules) {
        self.0.sort_by(|a, b| rules
            .get(a)
            .map_or(Ordering::Equal, |dependant| {
                if dependant.contains(b) { Ordering::Less }
                else { Ordering::Greater }
            })
        );
    }

    fn middle(&self) -> Option<u32> {
        self.0
            .get(self.0.len() / 2)
            .copied()
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let (rules, updates) = parse_manual.run(input)?;
    
    let sum: u32 = updates
        .into_iter()
        .filter(|update| update.is_ordered(&rules))
        .filter_map(|update| update.middle())
        .sum();

    Ok(Box::new(sum))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let (rules, updates) = parse_manual.run(input)?;

    let sum: u32 = updates
        .into_iter()
        .filter(|update| !update.is_ordered(&rules))
        .update(|update| update.order(&rules))
        .filter_map(|update| update.middle())
        .sum();

    Ok(Box::new(sum))
}