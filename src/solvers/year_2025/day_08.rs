use itertools::Itertools;
use nom::{Parser, character::complete::{char, u32}, sequence::delimited};
use petgraph::graph::UnGraph;
use yuki::parsing::{Parsable, ParsingResult, parse_lines};

use crate::SolverResult;

#[derive(Debug, Clone, Copy)]
struct JunctionBox {
    x: u32,
    y: u32,
    z: u32
}

impl Parsable<'_> for JunctionBox {
    fn parse(input: &str) -> ParsingResult<'_, Self> {
        (
            u32,
            delimited(char(','), u32, char(',')),
            u32
        )
        .map(|(x, y, z)| JunctionBox { x, y, z })
        .parse(input)
    }
}

impl JunctionBox {
    fn distance(self, other: Self) -> f32 {
        [
            (self.x, other.x),
            (self.y, other.y),
            (self.z, other.z)
        ].into_iter()
            .map(|(a, b)| ((a as f32) - (b as f32)).powi(2))
            .sum::<f32>()
            .sqrt()
    }
}

fn create_edges<'a, I>(junction_boxes: I) -> impl Iterator<Item=(usize, usize)> where
    I: IntoIterator<Item=&'a JunctionBox>,
    I::IntoIter: Clone
{
    junction_boxes
        .into_iter()
        .copied()
        .enumerate()
        .tuple_combinations()
        .map(|((a_idx, a), (b_idx, b))| (a_idx, b_idx, a.distance(b)))
        .sorted_unstable_by(|(_, _, a_dist), (_, _, b_dist)| a_dist.total_cmp(b_dist))
        .map(|(a_idx, b_idx, _)| (a_idx, b_idx))
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let junction_boxes: Vec<JunctionBox> = parse_lines(input)?;
    let edges = create_edges(&junction_boxes);
    let graph: UnGraph<(), (), usize> = UnGraph::from_edges(edges.take(1_000));

    let result: u32 = petgraph::algo::kosaraju_scc(&graph)   
        .into_iter()
        .map(|component| component.len() as u32)
        .sorted_by(|a, b| a.cmp(b).reverse())
        .take(3)
        .product();

    Ok(Box::new(result))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let junction_boxes: Vec<JunctionBox> = parse_lines(input)?;
    let mut edges = create_edges(&junction_boxes);
    let mut graph: UnGraph<(), (), usize> = UnGraph::from_edges(edges.by_ref().take(1_000));

    let (from, to) = edges.find(|&(from, to)| {
        graph.extend_with_edges([(from, to, ())]);
        petgraph::algo::connected_components(&graph) == 1
    })
    .expect("Graph is already connected before all edges are drained");

    let distance_from_wall = u64::from(junction_boxes[from].x) * u64::from(junction_boxes[to].x);
    Ok(Box::new(distance_from_wall))
}