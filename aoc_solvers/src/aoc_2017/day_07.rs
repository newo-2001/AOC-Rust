use std::{collections::{HashMap, VecDeque}, iter::once};

use aoc_lib::{parsing::{ParseError, Runnable, parse_lines}, iteration::queue::{Queue, FindState}, NoSolutionError};
use aoc_runner_api::SolverResult;
use nom::{character::complete::{alpha1, self, space1}, sequence::{delimited, terminated, preceded, tuple}, multi::separated_list1, bytes::complete::tag, combinator::opt, Parser};

#[derive(PartialEq, Eq, Hash)]
struct Program<'a> {
    name: &'a str,
    weight: u32,
    children: Vec<&'a str>
}

fn parse_program(input: &str) -> Result<Program, ParseError> {
    let name = terminated(alpha1, space1);
    let weight = delimited(complete::char('('), complete::u32, complete::char(')'));
    let children = preceded(tag(" -> "), separated_list1(tag(", "), alpha1));

    tuple((name, weight, opt(children))).map(|(name, weight, children)| Program {
        children: children.unwrap_or_else(Vec::new),
        name, weight,
    }).run(input)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let programs = parse_lines(parse_program, input)?;
    let map: HashMap<&str, &Program> = programs.iter().flat_map(|parent| {
        parent.children.iter().map(move |&child| (child, parent))
    }).collect();

    let queue: VecDeque<_> = map.values().take(1).collect();
    let root = queue.recursive_find(|node| {
        if let Some(parent) = map.get(node.name) {
            FindState::Branch(once(parent))
        } else { FindState::Result(node.name) }
    }).ok_or(NoSolutionError)?;

    Ok(Box::new(root))
}