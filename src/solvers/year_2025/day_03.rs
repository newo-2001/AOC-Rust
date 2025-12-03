use nom::{Parser, character::complete::digit1};
use yuki::parsing::{ParserExt, ParsingResult, combinators::lines};
use anyhow::{Context, Result};

use crate::SolverResult;

fn parse_bank(input: &str) -> ParsingResult<'_, Box<[u8]>> {
    digit1.map_opt(|digits: &str| digits
        .chars()
        .map(|char| char
            .to_digit(10)
            .map(|digit| digit as u8)
        )
        .collect::<Option<Vec<u8>>>()
        .map(Vec::into_boxed_slice)
    ).parse(input)
}

fn max_joltage<const N: usize>(bank: &[u8]) -> Result<u64> {
    let mut max_joltage = [Option::<u8>::None; N];

    for (i, &battery) in bank.iter().enumerate() {
        let start = N.saturating_sub(bank.len() - i);
        let mut iter = max_joltage[start..].iter_mut();

        let joltage = iter
            .by_ref()
            .find(|joltage| joltage
                .is_none_or(|joltage| joltage < battery)
            );

        if let Some(joltage) = joltage {
            joltage.replace(battery);
            iter.for_each(|x| *x = None);
        }
    }

    max_joltage
        .iter()
        .try_fold(0, |acc, &joltage| joltage
            .map(|joltage| acc * 10 + u64::from(joltage))
        )
        .with_context(|| format!("Not enough batteries in input, at least {N} are required"))
}

fn solve<const N: usize>(input: &str) -> SolverResult {
    let banks = lines(parse_bank).run(input)?;
    
    let total_joltage = banks
        .iter()
        .try_fold(0, |acc, bank| max_joltage::<N>(&bank)
            .map(|x| acc + x)
        )?;

    Ok(Box::new(total_joltage))
}

pub fn solve_part_1(input: &str) -> SolverResult {
    solve::<2>(input)
}

pub fn solve_part_2(input: &str) -> SolverResult {
    solve::<12>(input)
}