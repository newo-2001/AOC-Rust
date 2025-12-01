use nom::{AsChar, Parser, branch::alt, bytes::complete::{tag, take_while_m_n}, character::complete::{anychar, char, u32}, combinator::{map_parser, rest, value}, multi::{many_till, many0}, sequence::{preceded, separated_pair, terminated}};
use yuki::{parsing::{combinators::{map2, parens}, Parsable, ParserExt, ParsingResult}, tuples::{snd, Fst}};

use crate::SolverResult;

#[derive(Clone, Copy)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont
}

fn parse_num(input: &str) -> ParsingResult<'_, u32> {
    map_parser(
        take_while_m_n(1, 3, AsChar::is_dec_digit),
        u32
    )
    .parse(input)
}

impl<'a> Parsable<'a> for Instruction {
    fn parse(input: &'a str) -> ParsingResult<'a, Self> {
        alt((
            map2(
                preceded(
                    tag("mul"),
                    parens(
                        separated_pair(
                            parse_num,
                            char(','),
                            parse_num
                        )
                    )
                ),
                Self::Mul
            ),
            value(Self::Do, tag("do()")),
            value(Self::Dont, tag("don't()"))
        ))
        .parse(input)
    }
}

fn parse_instructions(input: &str) -> ParsingResult<'_, Vec<Instruction>> {
    terminated(
        many0(
            many_till(
                anychar,
                Instruction::parse
            )
            .map(snd)
        ),
        rest
    )
    .parse(input)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let sum: u32 = parse_instructions.run(input)?
        .into_iter()
        .filter_map(|instruction| match instruction {
            Instruction::Mul(left, right) => Some(left * right),
            Instruction::Do | Instruction::Dont => None
        })
        .sum();

    Ok(Box::new(sum))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let sum = parse_instructions.run(input)?
        .into_iter()
        .fold((0, true), |(sum, enabled), instruction| match instruction {
            Instruction::Do => (sum, true),
            Instruction::Dont => (sum, false),
            Instruction::Mul(left, right) if enabled => (sum + left * right, enabled),
            Instruction::Mul(_, _) => (sum, enabled)
        })
        .fst();

    Ok(Box::new(sum))
}