use ahash::{HashSet, HashSetExt};
use aoc_lib::parsing::{Runnable, ParseError};
use aoc_runner_api::SolverResult;
use itertools::Itertools;
use nom::{character::complete::{u32, line_ending}, bytes::complete::tag, sequence::separated_pair, multi::separated_list0, Parser};
use petgraph::{Graph, Undirected, visit::{Bfs, Walker, IntoNodeIdentifiers}, adj::NodeIndex};

fn parse_graph(input: &str) -> Result<Graph<(), (), Undirected>, ParseError> {
    let edges = separated_list0(tag(", "), u32);
    let node = separated_pair(u32, tag(" <-> "), edges)
        .map(|(id, edges)| edges.into_iter().map(move |to| (id, to)));
    
    separated_list0(line_ending, node)
        .map(|nodes| {
            let edges = nodes.into_iter().flatten().collect_vec();
            Graph::from_edges(edges)
        }).run(input)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let graph = parse_graph(input)?;
    let subgraph_size = Bfs::new(&graph, NodeIndex::from(0))
        .iter(&graph)
        .count();

    Ok(Box::new(subgraph_size))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let graph = parse_graph(input)?;
    
    let mut seen = HashSet::<_>::new();
    let disconnected_subgraphs: u32 = graph.node_identifiers()
        .fold(0,|subgraphs, node| {
            let is_duplicate_subgraph = Bfs::new(&graph, node)
                .iter(&graph)
                .take_while(|&node| seen.insert(node))
                .count() == 0; // .empty() would not force enumeration

            if is_duplicate_subgraph { subgraphs }
            else { subgraphs + 1 }
        });

    Ok(Box::new(disconnected_subgraphs))
}