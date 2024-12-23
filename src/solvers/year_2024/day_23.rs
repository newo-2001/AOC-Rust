use std::{collections::hash_map::Entry, iter::once};

use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};
use itertools::Itertools;
use nom::{character::complete::{alpha1, char}, sequence::separated_pair, Parser};
use recursive::recursive;
use yuki::{errors::{MultipleSolutions, NoInput}, iterators::{ExtraIter, SingleError}, parsing::{combinators::lines, ParserExt, ParsingResult}};

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

/// Bron-Kerbosch for finding max cliques
#[recursive]
fn largest_cliques<'a>(
    graph: &Graph<'a>,
    r: HashSet<&'a str>,
    mut p: HashSet<&'a str>,
    mut x: HashSet<&'a str>
) -> HashSet<Vec<&'a str>> {
    if p.is_empty() && x.is_empty() {
        return once(r.into_iter().collect()).collect();
    }

    let mut cliques = HashSet::<Vec<&str>>::new();
    while let Some(&v) = p.iter().next() {
        let mut new_r = r.clone();
        new_r.insert(v);

        let new_p = p.intersection(&graph[v]).copied().collect();
        let new_x = x.intersection(&graph[v]).copied().collect();
        cliques.extend(largest_cliques(graph, new_r, new_p, new_x));

        p.remove(v);
        x.insert(v);
    }

    cliques
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let edges = lines(parse_edge).run(input)?;
    let graph = build_graph(edges);

    let vertices = graph.keys().copied().collect();
    let cliques = largest_cliques(&graph, HashSet::new(), vertices, HashSet::new());

    let max_size = cliques
        .iter()
        .map(Vec::len)
        .max()
        .ok_or(NoInput)?;

    let password= cliques
        .iter()
        .filter(|clique| clique.len() == max_size)
        .single()
        .map_err(|err| match err {
            SingleError::More => MultipleSolutions,
            SingleError::None => unreachable!()
        })?
        .iter()
        .sorted()
        .join(",");

    Ok(Box::new(password))
}