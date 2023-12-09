use ahash::HashMap;
use aoc_lib::{geometry::RotationDirection, parsing::{ParseError, Parsable, TextParser, TextParserResult, parens, Map2}, iteration::{TryFoldWhile, ExtraIter}};
use aoc_runner_api::SolverResult;
use nom::{multi::{many1, count, separated_list1}, character::complete::{line_ending, alphanumeric1}, sequence::{terminated, separated_pair}, Parser, bytes::complete::tag};
use anyhow::{Result, anyhow};
use derive_more::Display;
use num::Integer;
use tupletools::fst;

#[derive(Clone, PartialEq, Eq, Hash, Display)]
struct NodeId<'a>(&'a str);

impl<'a> NodeId<'a> {
    fn parse(input: &'a str) -> TextParserResult<'a, Self> {
        alphanumeric1.map(Self).parse(input)
    }

    fn is_start(&self) -> bool { self.0.ends_with('A') }
    fn is_end(&self) -> bool { self.0.ends_with('Z') }
}

struct Edges<'a>(NodeId<'a>, NodeId<'a>);

impl<'a> Edges<'a> {
    fn parse(input: &'a str) -> TextParserResult<'a, Self> {
        parens(separated_pair(NodeId::parse, tag(", "), NodeId::parse))
            .map2(Edges)
            .parse(input)
    }

    fn get(&self, direction: RotationDirection) -> &NodeId<'_> {
        match direction {
            RotationDirection::Left => &self.0,
            RotationDirection::Right => &self.1
        }
    }
}

fn parse_map(input: &str) -> Result<(Vec<RotationDirection>, HashMap<NodeId, Edges>), ParseError> {
    let instructions = terminated(many1(RotationDirection::parse), count(line_ending, 2));
    let node = separated_pair(NodeId::parse, tag(" = "), Edges::parse);
    let edges = separated_list1(line_ending, node)
        .map(|nodes| nodes.into_iter().collect::<HashMap<_, _>>());

    instructions.and(edges).run(input)
}

fn distance_until<'a, I>(
    map: &'a HashMap<NodeId<'a>, Edges<'a>>,
    instructions: I,
    start: &'a NodeId<'a>,
    predicate: impl Fn(&'a NodeId<'a>) -> bool
) -> Result<usize>
    where I: IntoIterator<Item=&'a RotationDirection>,
          I::IntoIter: Clone
{
    instructions
        .into_iter()
        .cycle()
        .try_fold_while((0usize, start), |(distance, node), direction| {
            match map.get(node) {
                None => TryFoldWhile::Err(anyhow!("Attempted to visit undefined node: {}", node)),
                Some(edges) => {
                    let node = edges.get(*direction);
                    let next = (distance + 1, node);

                    if (predicate)(node) { TryFoldWhile::Done(next) }
                    else { TryFoldWhile::Continue(next) }
                }
            }
        }).map(fst)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let (instructions, map) = parse_map(input)?;
    let distance = distance_until(&map, &instructions, &NodeId("AAA"), |node| node == &NodeId("ZZZ"))?;

    Ok(Box::new(distance))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let (instructions, map) = parse_map(input)?;

    let distance: usize = map.keys()
        .filter(|node| node.is_start())
        .map(|start| {
            distance_until(&map, &instructions, start, NodeId::is_end)
        }).collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .reduce(|x, y| x.lcm(&y))
        .ok_or(anyhow!("Input contained no starting nodes"))?;

    Ok(Box::new(distance))
}