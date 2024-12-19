use ahash::{HashMap, HashMapExt};
use nom::{bytes::complete::tag, character::complete::{alpha1, line_ending}, multi::{count, separated_list0}, sequence::separated_pair, Parser};
use recursive::recursive;
use yuki::parsing::{ParserExt, ParsingResult};

use crate::SolverResult;

fn parse_towels(input: &str) -> ParsingResult<(Vec<&str>, Vec<&str>)> {
    separated_pair(
        separated_list0(tag(", "), alpha1),
        count(line_ending, 2),
        separated_list0(line_ending, alpha1)
    )
    .parse(input)
}

#[recursive]
fn is_composite_of(target: &str, towels: &[&str]) -> bool {
    if target.is_empty() { return true; }

    towels
        .iter()
        .filter(|&towel| target.starts_with(towel))
        .any(|towel| is_composite_of(&target[towel.len()..], towels))
}

#[recursive]
fn count_possible_arrangements<'a>(target: &'a str, towels: &[&'a str], cache: &mut HashMap<(&'a str, Vec<&'a str>), usize>) -> usize {
    let state = (target, towels.to_vec());
    if let Some(&cached) = cache.get(&state) { return cached; }
    if target.is_empty() { return 1; }

    let arrangements = towels
        .iter()
        .filter(|&towel| target.starts_with(towel))
        .map(|towel| count_possible_arrangements(&target[towel.len()..], towels, cache))
        .sum();

    cache.insert(state, arrangements);
    arrangements
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let (towels, desired_patterns) = parse_towels.run(input)?;

    let possible = desired_patterns
        .into_iter()
        .filter(|pattern| is_composite_of(pattern, &towels))
        .count();

    Ok(Box::new(possible))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mut cache = HashMap::<(&str, Vec<&str>), usize>::new();
    let (towels, desired_patterns) = parse_towels.run(input)?;

    let arrangements: usize = desired_patterns
        .into_iter()
        .map(|pattern| count_possible_arrangements(pattern, &towels, &mut cache))
        .sum();

    Ok(Box::new(arrangements))
}