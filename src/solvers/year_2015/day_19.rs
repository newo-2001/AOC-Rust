use std::iter::once;

use ahash::{HashSet, HashSetExt};
use aoc_lib::parsing::{Parsable, TextParserResult, lines, TextParser};
use priority_queue::PriorityQueue;
use yuki::errors::NoSolution;
use crate::SolverResult;
use itertools::Itertools;
use nom::{character::complete::{alpha1, line_ending}, Parser, sequence::separated_pair, bytes::complete::tag};

struct Replacement<'a> {
    from: &'a str,
    to: &'a str
}

impl<'a> Parsable<'a> for Replacement<'a> {
    fn parse(input: &'a str) -> TextParserResult<'a, Self> {
        separated_pair(alpha1, tag(" => "), alpha1)
            .map(|(from, to)| Replacement { from, to })
            .parse(input)
    }
}

fn mutations(chemical: &str, from: &str, to: &str) -> Vec<String> {
    chemical.match_indices(from).map(|(index, _)| {
        let (start, end) = chemical.split_at(index);

        let end = end.to_owned().replacen(from, to, 1);
        let mut start = start.to_owned();
        start.push_str(&end);
        
        start
    }).collect_vec()
}

fn forward_mutations(chemical: &str, replacement: &Replacement) -> Vec<String> {
    mutations(chemical, replacement.from, replacement.to)
}

fn backwards_mutations(chemical: &str, replacement: &Replacement) -> Vec<String> {
    mutations(chemical, replacement.to, replacement.from)
}

fn fastest_synthesis(target: &str, replacements: &[Replacement]) -> Result<usize, NoSolution> {
    let mut seen: HashSet<String> = HashSet::new();
    let mut queue: PriorityQueue<String, usize> = once((String::from(target), 0)).collect();

    while let Some((chemical, distance)) = queue.pop() {
        if chemical == "e" { return Ok(distance); }

        let mutations = replacements
            .iter()
            .flat_map(|replacement| backwards_mutations(&chemical, replacement));

        for mutation in mutations {
            if seen.insert(mutation.clone()) {
                queue.push(mutation, distance + 1);
            }
        }
    }

    Err(NoSolution)
}

fn parse_data(input: &str) -> TextParserResult<(Vec<Replacement>, &str)> {
    separated_pair(
        lines(Replacement::parse),
        line_ending.and(line_ending),
        alpha1
    ).parse(input)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let (replacements, target) = parse_data.run(input)?;

    let unique_mutations = replacements
        .iter()
        .flat_map(|replacement| forward_mutations(target, replacement))
        .unique()
        .count();

    Ok(Box::new(unique_mutations))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let (replacements, target) = parse_data.run(input)?;
    let fastest = fastest_synthesis(target, &replacements)?;

    Ok(Box::new(fastest))
}