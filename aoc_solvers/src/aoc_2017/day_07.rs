use std::hash::{Hash, Hasher};

use ahash::HashMap;
use aoc_lib::parsing::{ParseError, Runnable, parse_lines, brackets};
use aoc_runner_api::SolverResult;
use itertools::Itertools;
use nom::{character::complete::{alpha1, space1, u32}, sequence::{terminated, preceded, tuple}, multi::separated_list1, bytes::complete::tag, combinator::opt, Parser};
use thiserror::Error;

#[derive(PartialEq, Eq, Hash)]
struct Program<'a> {
    name: &'a str,
    weight: u32,
    children: Vec<&'a str>
}

impl<'a> Program<'a> {
    fn parse(input: &'a str) -> Result<Program<'a>, ParseError<'a>> {
        let name = terminated(alpha1, space1);
        let weight = brackets(u32);
        let children = preceded(tag(" -> "), separated_list1(tag(", "), alpha1));

        tuple((name, weight, opt(children))).map(|(name, weight, children)| Program {
            children: children.unwrap_or_else(Vec::new),
            name, weight,
        }).run(input)
    }
}

#[derive(Debug, Error)]
enum Error<'a> {
    #[error("Node with name {0} does not exist")]
    InvalidNode(&'a str),
    #[error("Multiple unbalanced nodes exist")]
    MultipleUnbalancedNodes,
    #[error("No unbalanced node exists")]
    NoUnbalancedNode,
    #[error("The tower has no nodes")]
    NoNodes
}

fn find_root<'a, 'b>(tower: impl IntoIterator<Item=&'b Program<'a>>) -> Option<&'b Program<'a>> {
    let tower: HashMap<&str, &Program> = tower.into_iter().flat_map(|node| {
        node.children.iter().map(move |&child| (child, node))
    }).collect();

    let mut node = *tower.values().next()?;
    loop {
        match tower.get(node.name) {
            Some(parent) => node = parent,
            None => break Some(node)
        }
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let tower = parse_lines(Program::parse, input)?;
    let root = find_root(&tower).ok_or(Error::NoNodes)?;
    Ok(Box::new(root.name))
}

#[derive(Debug, Clone, Copy)]
struct BalancedNode {
    weight: u32,
    total_weight: u32
}

impl Eq for BalancedNode {}
impl PartialEq for BalancedNode {
    fn eq(&self, other: &Self) -> bool {
        self.total_weight == other.total_weight
    }
}

impl Hash for BalancedNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.total_weight.hash(state);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Balanced {
    Balanced(BalancedNode),
    Unbalanced(u32)
}

// This seems way too verbose..
// Also, stack recursion is dangerous as this could overflow the stack
fn unbalanced_difference<'a: 'b, 'b>(tower: impl IntoIterator<Item=&'b Program<'a>>) -> Result<u32, Error<'a>> {
    fn weight_difference<'a: 'b, 'b>(tower: &HashMap<&'a str, &'b Program<'a>>, root: &'b Program<'a>) -> Result<Balanced, Error<'a>> {
        let child_weights = root.children.iter().map(|&name| {
            let child = tower.get(&name).ok_or(Error::InvalidNode(name))?;
            weight_difference(tower, child)
        }).collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .counts()
            .into_iter()
            .sorted_unstable_by_key(|(_, count)| *count)
            .rev()
            .collect_vec();

        match child_weights[..] {
            [] => Ok(Balanced::Balanced(BalancedNode { weight: root.weight, total_weight: root.weight })),
            [(Balanced::Balanced(balanced), count)] => Ok(Balanced::Balanced(BalancedNode {
                weight: root.weight,
                total_weight: root.weight + balanced.total_weight * u32::try_from(count).unwrap()
            })),
            [(Balanced::Balanced(balanced), _), (Balanced::Balanced(unbalanced), 1)] => {
                Ok(Balanced::Unbalanced(unbalanced.weight - (unbalanced.total_weight - balanced.total_weight)))
            },
            [_, (difference @ Balanced::Unbalanced(_), 1)] => Ok(difference),
            _ => Err(Error::MultipleUnbalancedNodes)
        }
    }

    let tower: HashMap<&str, &Program> = tower.into_iter()
        .map(|node| (node.name, node))
        .collect();

    let root = find_root(tower.values().copied()).ok_or(Error::NoNodes)?;
    match weight_difference(&tower, root)? {
        Balanced::Balanced(_) => Err(Error::NoUnbalancedNode),
        Balanced::Unbalanced(difference) => Ok(difference)
    }
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let tower: Vec<Program> = parse_lines(Program::parse, input)?;
    let difference = unbalanced_difference(&tower)?;

    Ok(Box::new(difference))
}