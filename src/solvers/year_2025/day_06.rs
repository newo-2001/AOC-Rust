use std::{fmt::{Display, Formatter}, ops::{Add, Mul}};

use aoc_lib::iteration::ExtraIter;
use itertools::{EitherOrBoth, Itertools};
use nom::{Parser, character::complete::{char, space1, u32}, combinator::value, multi::separated_list0};
use yuki::{errors::NoInput, parsing::{Parsable, ParserExt, ParsingResult}};
use anyhow::{Result, anyhow, bail};

use crate::SolverResult;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mul
}

#[derive(Debug, Clone)]
struct Problem {
    operands: Vec<u32>,
    operator: Operator
}

impl Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            &Self::Add => '+',
            &Self::Mul => '*'
        })
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let operator = format!(" {} ", self.operator);
        let problem = self.operands.iter().join(&operator);
        write!(f, "{}", problem)
    }
}

impl Parsable<'_> for Operator {
    fn parse(input: &'_ str) -> ParsingResult<'_, Self> {
        Parser::or(
            value(Self::Add, char('+')),
            value(Self::Mul, char('*'))
        ).parse(input)
    }
}

fn parse_worksheet<'a>(input: &'a str) -> Result<(impl Iterator<Item = &'a str>, Vec<Operator>)> {
    let mut lines = input.lines().rev();
    let line = lines.next().ok_or(NoInput)?;
    let operators = separated_list0(space1, Operator::parse).run(line.trim())?;

    Ok((lines.rev(), operators))
}

fn create_problems<I, O>(operands: I, operators: O) -> Result<Vec<Problem>> where
    I: IntoIterator<Item = Vec<u32>>,
    O: IntoIterator<Item = Operator>
{
    operands
        .into_iter()
        .zip_longest(operators)
        .map(|problem| match problem {
            EitherOrBoth::Both(operands, operator) => Ok(Problem { operands, operator }),
            _ => bail!("Number of operands doesn't match the number of operators")
        })
        .collect()
}

impl Problem {
    fn solve(&self) -> Option<u64> {
        let operands = self.operands
            .iter()
            .map(|&operand| u64::from(operand));

        match self.operator {
            Operator::Add => operands.reduce(Add::add),
            Operator::Mul => operands.reduce(Mul::mul)
        }
    }
}

fn solve_problems<I>(problems: I) -> Result<u64> where
    I: IntoIterator<Item=Problem>
{
    problems
        .into_iter()
        .map(|problem| problem
            .solve()
            .ok_or_else(|| anyhow!("Problem has less than 2 operands: {problem}"))
        )
        .try_fold(0, |acc, n| n.map(|n| acc + n))
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let (operands, operators) = parse_worksheet(input)?;
    let operands: Vec<Vec<u32>> = operands
        .map(|operands| separated_list0(space1, u32).run(operands.trim()))
        .try_collect()?;

    let problems = create_problems(
        operands.into_iter().transpose(),
        operators
    )?;

    let total = solve_problems(problems)?;

    Ok(Box::new(total))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let (operands, operators) = parse_worksheet(input)?;

    let operands: Vec<String> = operands
        .map(|str| str.chars())
        .transpose()
        .map(|operand| operand
            .into_iter()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
        )
        .collect();

    let operands: Vec<Vec<u32>> = operands
        .split(|operand| operand.is_empty())
        .map(|operands| operands
            .iter()
            .map(|operand| operand.parse::<u32>())
            .try_collect()
        )
        .try_collect()?;

    let problems = create_problems(operands, operators)?;
    let total = solve_problems(problems)?;

    Ok(Box::new(total))
}