use std::collections::BTreeSet;

use ahash::{HashSet, HashSetExt};
use aoc_lib::{parsing::{Parsable, TextParserResult, lines, TextParser}, errors::NoSolution};
use crate::SolverResult;
use itertools::Itertools;
use nom::{character::complete::{alpha1, line_ending}, Parser, sequence::separated_pair, bytes::complete::tag};

struct Replacement<'a> {
    from: &'a str,
    to: &'a str
}

#[derive(PartialEq, Eq, Clone)]
struct Mutation {
    chemical: String,
    distance: usize,
}

impl PartialOrd for Mutation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Mutation {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.distance).cmp(&(other.distance)).reverse()
    }
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
    let mut queue: BTreeSet<Mutation> = BTreeSet::new();
    let mut cache: HashSet<String> = HashSet::new();
    
    _ = queue.insert(Mutation {
        chemical: String::from(target),
        distance: 0
    });

    while let Some(current_mutation) = queue.iter().next() {
        let Mutation { chemical, distance, .. } = current_mutation.clone();

        if chemical == "e" { return Ok(distance); }
       
        let mutations = replacements
            .iter()
            .flat_map(|replacement| backwards_mutations(&chemical, replacement))
            .filter(|x| !cache.contains(x))
            .collect_vec();

        for mutation in mutations {
            cache.insert(mutation.clone());

            _ = queue.insert(Mutation {
                chemical: mutation,
                distance: distance + 1
            });
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