use aoc_lib::parsing::{curly_brackets, TextParserResult, TextParser, angle_brackets};
use crate::SolverResult;
use nom::{character::complete::{anychar, char, none_of}, combinator::{map, value}, multi::{many0, separated_list0}, Parser};

enum Group {
    SubGroups(Vec<Group>),
    Garbage(String)
}

impl Group {
    // I think this solution is beautiful
    fn parse(input: &str) -> TextParserResult<'_, Self> {
        Parser::or(
            map(
                curly_brackets(
                    separated_list0(char(','), Self::parse)
                ),
                Group::SubGroups
            ),
            map(
                angle_brackets(
                    map(
                        many0(
                            Parser::or(
                                value(None, char('!').and(anychar)),
                                none_of(">").map(Some)
                            )
                        ),
                        |garbage| garbage.into_iter().flatten().collect()
                    )
                ),
                |garbage: Vec<char>| Self::Garbage(garbage.into_iter().collect())
            )
        ).parse(input)
    }

    fn score(&self, parent_score: u32) -> u32 {
        match self {
            Self::SubGroups(subgroups) => {
                subgroups.iter()
                    .map(|subgroup| subgroup.score(parent_score + 1))
                    .sum::<u32>() + parent_score
            },
            Self::Garbage(_) => 0
        }
    }

    fn garbage_length(&self) -> usize {
        match self {
            Self::SubGroups(subgroups) => subgroups
                .iter()
                .map(Self::garbage_length)
                .sum(),
            Self::Garbage(garbage) => garbage.len()
        }
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let group = Group::parse.run(input)?;
    Ok(Box::new(group.score(1)))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let group = Group::parse.run(input)?;
    Ok(Box::new(group.garbage_length()))
}