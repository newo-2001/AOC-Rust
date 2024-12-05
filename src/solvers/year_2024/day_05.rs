use std::collections::HashMap;

use itertools::Itertools;
use nom::{character::complete::{char, line_ending, u32}, combinator::map, multi::{count, separated_list1}, sequence::separated_pair, Parser};
use yuki::parsing::{combinators::lines, ParserExt, ParsingResult};

use crate::SolverResult;

#[derive(Debug)]
struct Update(Vec<u32>);

fn parse_manual(input: &str) -> ParsingResult<(HashMap<u32, Vec<u32>>, Vec<Update>)> {
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
    fn is_ordered(&self, rules: &HashMap<u32, Vec<u32>>) -> bool {
        self.0
            .iter()
            .enumerate()
            .all(|(i, num)| rules
                .get(num)
                .map_or(true, |dependant| !dependant
                    .iter()
                    .any(|dependant| self.0[..i].contains(dependant))
                )
            )
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