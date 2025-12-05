use aoc_lib::math::Range;
use itertools::Itertools;
use nom::{Parser, character::complete::{char, line_ending, u64}, multi::count, sequence::separated_pair};
use yuki::parsing::{Parsable, ParsingResult, combinators::lines, parse};

use crate::SolverResult;

#[derive(Debug)]
struct Inventory {
    fresh_ingredients: Vec<Range<u64>>,
    available_ingredients: Vec<u64>
}

fn parse_range(input: &'_ str) -> ParsingResult<'_, Range<u64>> {
    separated_pair(u64, char('-'), u64)
        .map_res(|(lower, upper)| Range::inclusive(lower, upper))
        .parse(input)
}

impl Parsable<'_> for Inventory {
    fn parse(input: &'_ str) -> ParsingResult<'_, Self> {
        separated_pair(
            lines(parse_range),
            count(line_ending, 2),
            lines(u64)
        )
        .map(|(fresh_ingredients, available_ingredients)| Inventory {
            fresh_ingredients,
            available_ingredients
        })
        .parse(input)
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let inventory: Inventory = parse(input)?;

    let fresh_ingredients = inventory.available_ingredients
        .into_iter()
        .filter(|ingredient| inventory.fresh_ingredients
            .iter()
            .any(|range| range.contains(ingredient))
        )
        .count();

    Ok(Box::new(fresh_ingredients))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mut inventory: Inventory = parse(input)?;
    inventory.fresh_ingredients.sort_unstable();

    let num_fresh_ingredients: u64 = inventory.fresh_ingredients
        .into_iter()
        .coalesce(|left, right| left.merge(right))
        .map(Range::interval)
        .sum();

    Ok(Box::new(num_fresh_ingredients))
}