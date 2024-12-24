use std::collections::HashMap;

use nom::{branch::alt, bytes::complete::tag, character::complete::{alpha1, anychar, char, line_ending, u8}, combinator::{map, value}, multi::count, sequence::{pair, separated_pair}, Parser};
use recursive::recursive;
use yuki::{parsing::{combinators::{lines, Map2}, Parsable, ParserExt, ParsingResult}, tuples::swap};

use crate::SolverResult;

#[derive(Debug, Clone, Copy)]
enum Operation<'a> {
    And(Name<'a>, Name<'a>),
    Or(Name<'a>, Name<'a>),
    Xor(Name<'a>, Name<'a>),
    Constant(bool)
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Name<'a> {
    Branch(&'a str),
    Leaf(char, u8)
}

impl<'a> Parsable<'a> for Name<'a> {
    fn parse(input: &'a str) -> ParsingResult<'a, Self> {
        Parser::or(
            pair(anychar, u8).map2(Name::Leaf),
            map(alpha1, Name::Branch)
        )
        .parse(input)
    }
}

type Graph<'a> = HashMap<Name<'a>, Operation<'a>>;

fn parse_node(input: &str) -> ParsingResult<(Name<'_>, Operation<'_>)> {
    separated_pair(
        alt((
            separated_pair(Name::parse, tag(" AND "), Name::parse).map2(Operation::And),
            separated_pair(Name::parse, tag(" OR "), Name::parse).map2(Operation::Or),
            separated_pair(Name::parse, tag(" XOR "), Name::parse).map2(Operation::Xor)
        )),
        tag(" -> "),
        Name::parse
    )
    .map(swap)
    .parse(input)
}

fn parse_graph(input: &str) -> ParsingResult<Graph<'_>> {
    separated_pair(
        lines(
            map(
                separated_pair(
                    Name::parse,
                    tag(": "),
                    Parser::or(
                        value(false, char('0')),
                        value(true, char('1'))
                    )
                ),
                |(name, value)| (name, Operation::Constant(value))
            )
        ),
        count(line_ending, 2),
        lines(parse_node),
    )
    .map(|(constants, nodes)| constants
        .into_iter()
        .chain(nodes)
        .collect()
    )
    .parse(input)
}

#[recursive]
fn solve_node(graph: &Graph, operation: Operation<'_>) -> bool {
    match &operation {
        Operation::And(lhs, rhs) => solve_node(graph, graph[lhs]) && solve_node(graph, graph[rhs]),
        Operation::Or(lhs, rhs) => solve_node(graph, graph[lhs]) || solve_node(graph, graph[rhs]),
        Operation::Xor(lhs, rhs) => solve_node(graph, graph[lhs]) ^ solve_node(graph, graph[rhs]),
        Operation::Constant(state) => *state
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let graph = parse_graph.run(input)?;

    let z: u64 = graph
        .iter()
        .filter_map(|(&name, &operation)| match name {
            Name::Leaf('z', bit) => Some(u64::from(solve_node(&graph, operation)) << bit),
            Name::Branch(_) | Name::Leaf(_, _) => None
        })
        .sum();

    Ok(Box::new(z))
}