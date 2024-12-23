use std::collections::hash_map::Entry;

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use itertools::Itertools;
use nom::{character::complete::{alpha1, char}, sequence::separated_pair, Parser};
use yuki::parsing::{combinators::lines, ParserExt, ParsingResult};

use crate::SolverResult;

fn parse_edge(input: &str) -> ParsingResult<(&str, &str)> {
    separated_pair(
        alpha1,
        char('-'),
        alpha1
    )
    .parse(input)
}

type Graph<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn insert_edge<'a>(graph: &mut Graph<'a>, (from, to): (&'a str, &'a str)) {
    let adjacency_list = match graph.entry(from) {
        Entry::Occupied(entry) => entry.into_mut(),
        Entry::Vacant(entry) => entry.insert(HashSet::new())
    };

    adjacency_list.insert(to);
}

fn build_graph<'a>(edges: impl IntoIterator<Item=(&'a str, &'a str)>) -> Graph<'a> {
    let mut graph = Graph::new();

    for (from, to) in edges {
        insert_edge(&mut graph, (from, to));
        insert_edge(&mut graph, (to, from));
    }

    graph
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let edges = lines(parse_edge).run(input)?;
    let graph = build_graph(edges);

    let groups = graph
        .iter()
        .filter(|(node, _)| node.starts_with('t'))
        .flat_map(|(&node, edges)| edges
            .iter()
            .flat_map(|&edge| graph[edge]
                .intersection(edges)
                .map(move |common_edge| [node, edge, common_edge])
                .update(|group| group.sort_unstable())
            )
            .collect_vec()
        )
        .unique()
        .count();

    Ok(Box::new(groups))
}