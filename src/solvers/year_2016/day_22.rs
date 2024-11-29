use std::{cmp::Ordering, iter::once, collections::VecDeque};

use aoc_lib::{geometry::{Point2D, grid::{Grid, GridLike}, Direction2D, CardinalDirection, Dimensions}, parsing::{ParseError, TextParser, usize}, math::Bit, iteration::{queue::{Dedupable, FindState, SearchDepth}, ExtraIter}, errors::NoSolution};
use crate::SolverResult;
use itertools::Itertools;
use nom::{sequence::{preceded, delimited, tuple}, bytes::complete::tag, character::complete::{space1, char, u8, u16}, Parser};

#[derive(Clone, Copy, Eq, PartialEq)]
struct Node {
    position: Point2D<usize>,
    size: u16,
    used: u16,
    avail: u16,
    used_percent: u8
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        let ordering = self.position.y().cmp(&other.position.y());
        match ordering {
            Ordering::Equal => self.position.x().cmp(&other.position.x()),
            _ => ordering
        }
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Node {
    fn parse(input: &str) -> Result<Self, ParseError> {
        let position = delimited(char('x'), usize, tag("-y")).and(usize);
        let position = preceded(tag("/dev/grid/node-"), position).map(Into::into);
        let size = || delimited(space1, u16, char('T'));
        let used_percent = delimited(space1, u8, char('%'));

        tuple((position, size(), size(), size(), used_percent))
            .map(|(position, size, used, avail, used_percent)| Self {
                position, size, used, avail, used_percent
            }).run(input)
    }

    const fn is_empty(&self) -> bool { self.used == 0 }
    const fn fits_on_node(&self, other: &Self) -> bool { other.avail >= self.used }

    fn as_tile<'a>(&self, neighbours: impl IntoIterator<Item=&'a Self>) -> Bit {
        neighbours.into_iter()
            .any(|neighbour| self.used >= neighbour.size)
            .into()
    }
}

fn parse_nodes(input: &str) -> Result<Vec<Node>, ParseError> {
    input.lines()
        .skip(2)
        .map(Node::parse)
        .try_collect()
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let viable_pairs = parse_nodes(input)?
        .into_iter()
        .permutations(2)
        .flatten()
        .tuples()
        .count_where(|(a, b)| !a.is_empty() && a != b && a.fits_on_node(b));

    Ok(Box::new(viable_pairs))
}

// This code looks *very* generic
// I think this could be fully abstracted to a generic grid search algorithm
fn find_path(grid: &Grid<Bit>, from: Point2D<usize>, to: Point2D<usize>) -> Result<usize, NoSolution>{
    once(SearchDepth::new(from)).collect::<VecDeque<_>>()
        .filter_duplicates()
        .recursive_find(|depth| {
            if depth.state == to { return FindState::Result(depth.depth) }

            let moves = depth.state
                .neighbours::<isize, _>(Direction2D::all())
                .filter_map(|position| {
                    (!grid.get(position)?.is_solid())
                        .then_some(depth.with(position))
                }).collect_vec();

            FindState::Branch(moves)
        }).ok_or(NoSolution)
}

// This solution makes more assumptions about the input than I'd like
pub fn solve_part_2(input: &str) -> SolverResult {
    let mut nodes = parse_nodes(input)?;
    nodes.sort();

    let Dimensions(width, _) = nodes.last()
        .ok_or(NoSolution)?
        .position.into();

    let nodes = nodes.into_iter()
        .chunks(width)
        .into_iter()
        .map(Iterator::collect)
        .collect();

    let grid = Grid::new(nodes)?;
    
    let goal_position = grid.get(grid.area().top_right())
        .ok_or(NoSolution)?.position;

    let empty_position = grid.iter()
        .find(|node| node.used == 0)
        .ok_or(NoSolution)?.position;

    // Inefficient, but whatever
    let grid = grid.clone().map(|node| {
        let neighbours = node.position
            .neighbours::<isize, _>(CardinalDirection::all())
            .filter_map(|position| grid.get(position));

        node.as_tile(neighbours)
    });

    let moves = find_path(&grid, empty_position, goal_position)? +
        (find_path(&grid, goal_position, Point2D::zero())? + 1) * 5;

    Ok(Box::new(moves))
}