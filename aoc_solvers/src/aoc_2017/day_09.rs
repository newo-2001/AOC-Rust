use aoc_lib::parsing::{curly_brackets, TextParserResult, Runnable, angle_brackets};
use aoc_runner_api::SolverResult;
use nom::{multi::{separated_list0, many0}, character::complete::{char, anychar, none_of}, Parser, combinator::value};

enum Group {
    SubGroups(Vec<Group>),
    Garbage(String)
}

impl Group {
    // I think this solution is beautiful
    fn parse(input: &str) -> TextParserResult<Group> {
        Parser::or(
            curly_brackets(
                separated_list0(char(','), Group::parse)
            ).map(Group::SubGroups),
            angle_brackets(
                many0(
                    Parser::or(
                        value(None, char('!').and(anychar)),
                        none_of(">").map(Some)
                    )
                ).map(|garbage| garbage.into_iter().flatten().collect())
            ).map(|garbage: Vec<char>| Group::Garbage(garbage.into_iter().collect()))
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
            Self::SubGroups(subgroups) => subgroups.iter().map(Group::garbage_length).sum(),
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