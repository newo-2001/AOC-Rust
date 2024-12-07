use nom::{bytes::complete::tag, character::complete::{char, u64}, multi::separated_list0, sequence::separated_pair, Parser};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use yuki::parsing::{parse_lines, Parsable, ParsingResult};
use anyhow::Result;

use crate::SolverResult;

#[derive(Debug)]
struct Equation {
    result: u64,
    operands: Vec<u64>
}

impl<'a> Parsable<'a> for Equation {
    fn parse(input: &'a str) -> ParsingResult<'a, Self> {
        separated_pair(
            u64,
            tag(": "),
            separated_list0(char(' '), u64)
        )
        .map(|(result, operands)| Self { result, operands })
        .parse(input)
    }
}

impl Equation {
    fn is_possible<F, I>(&self, operators: F) -> bool where
        F: Fn(u64, u64) -> I + Copy,
        I: IntoIterator<Item=u64>
    {
        match self.operands.as_slice() {
            [] => self.result == 0,
            [head] => *head == self.result,
            [head, ..] if *head > self.result => false,
            [first, second, tail @ ..] => operators(*first, *second)
                .into_iter()
                .map(|head| {
                    let mut operands = vec![head];
                    operands.extend_from_slice(tail);
                    Self { result: self.result, operands }
                })
                .any(|equation| equation.is_possible(operators))
        }
    }
}

pub fn calibration<F, I>(operators: F, input: &str) -> Result<u64> where
    F: Fn(u64, u64) -> I + Copy + Sync,
    I: IntoIterator<Item=u64>
{
    let total_calibration = parse_lines::<Equation>(input)?
        .into_par_iter()
        .filter(|equation| equation.is_possible(operators))
        .map(|equation| equation.result)
        .sum();

    Ok(total_calibration)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let operators = |lhs, rhs| [
        lhs + rhs,
        lhs * rhs
    ];

    let total_calibration = calibration(operators, input)?;
    Ok(Box::new(total_calibration))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let operators = |lhs: u64, rhs: u64| [
        lhs + rhs,
        lhs * rhs,
        lhs * 10u64.pow(rhs.ilog10() + 1) + rhs
    ];

    let total_calibration = calibration(operators, input)?;
    Ok(Box::new(total_calibration))
}