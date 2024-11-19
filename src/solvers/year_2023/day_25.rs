use std::{iter::once, collections::VecDeque, cmp::minmax};

use ahash::{HashMap, HashSet, HashMapExt, HashSetExt};
use anyhow::{Result, anyhow, Context};
use aoc_lib::{parsing::{ParseError, parse_lines, TextParser}, iteration::generate};
use crate::SolverResult;
use itertools::Itertools;
use nom::{character::complete::{alpha1, char}, sequence::separated_pair, bytes::complete::tag, multi::separated_list1};
use rand::random;
use tupletools::fst;

fn parse_component(input: &str) -> Result<(&str, Vec<&str>), ParseError> {
    separated_pair(
        alpha1,
        tag(": "),
        separated_list1(char(' '), alpha1)
    ).run(input)
}

struct Graph<'a>(HashMap<&'a str, HashSet<&'a str>>);

impl<'a> Graph<'a> {
    fn parse(input: &'a str) -> Result<Self, ParseError> {
        let mut graph = Self(HashMap::new());

        for (from, edges) in parse_lines(parse_component, input)? {
            for to in edges {
                graph.insert_edge(from, to);
                graph.insert_edge(to, from);
            }
        }

        Ok(graph)
    }

    fn remove_edge(&mut self, from: &str, to: &str) {
        if let Some(edges) = self.0.get_mut(from) {
            edges.remove(to);
        }

        if let Some(edges) = self.0.get_mut(to) {
            edges.remove(from);
        }
    }

    fn insert_edge(&mut self, from: &'a str, to: &'a str) {
        match self.0.get_mut(from) {
            Some(edges) => { edges.insert(to); },
            None => { self.0.insert(from, once(to).collect()); }
        }
    }

    fn subgraph_size(&self, node: &str) -> Result<usize> {
        let mut queue: Vec<&str> = once(node).collect();
        let mut seen = HashSet::new();

        while let Some(node) = queue.pop() {
            if !seen.insert(node) { continue; }

            self.0.get(node)
                .ok_or_else(|| anyhow!("Node not found: {node}"))?
                .iter()
                .filter(|&&node| !seen.contains(node))
                .collect_into(&mut queue);
        }

        Ok(seen.len())
    }

    fn shortest_path(&self, from: &'a str, to: &'a str) -> impl Iterator<Item=&str> {
        let mut shortest_from = HashMap::<&str, &str>::new();
        let mut queue: VecDeque<&str> = once(from).collect();

        while let Some(node) = queue.pop_front() {
            if node == to { break; }

            let neighbours = self.0.get(node).unwrap()
                .iter()
                .filter(|&&neighbour| !shortest_from.contains_key(neighbour) && neighbour != from)
                .collect_vec();
            
            for &neighbour in neighbours {
                queue.push_back(neighbour);
                shortest_from.insert(neighbour, node);
            }
        }

        generate(to, move |&node| shortest_from.get(node).copied())
    }

    fn most_common_edges(&self) -> impl Iterator<Item=(&str, &str)> {
        let mut counts = HashMap::<(&str, &str), u32>::new();
        let nodes = self.0.keys().copied().collect_vec();

        // Not a fan of this because it is probabilistic
        for _ in 0..100 {
            let from = nodes[random::<usize>() % nodes.len()];
            let to = nodes[random::<usize>() % nodes.len()];
            if from == to { continue; }

            for (from, to) in self.shortest_path(from, to).tuple_windows() {
                let [small, big] = minmax(from, to);
                match counts.get_mut(&(small, big)) {
                    None => _ = counts.insert((small, big), 1),
                    Some(value) => *value += 1
                }
            }
        }

        counts.into_iter()
            .sorted_by_key(|(_, frequency)| *frequency)
            .map(fst)
            .rev()
    }
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let mut graph = Graph::parse(input)?;

    // Fuck the borrowchecker
    let edges = graph.most_common_edges()
        .take(3)
        .map(|(from, to)| (from.to_owned(), to.to_owned()))
        .collect_vec();

    for (from, to) in edges {
        graph.remove_edge(from.as_str(), to.as_str());
    }

    let node = *graph.0.iter()
        .skip_while(|(_, edges)| edges.is_empty())
        .map(fst)
        .next()
        .context("Graph has less than 4 nodes")?;

    let first_subgraph_size = graph.subgraph_size(node)?;
    let second_subgraph_size = graph.0.len() - first_subgraph_size;

    Ok(Box::new(first_subgraph_size * second_subgraph_size))
}