use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    cmp::{max, min}
};

use aoc_lib::parsing::{run, parse_lines};
use aoc_runner_api::SolverResult;
use itertools::Itertools;
use nom::{
    character::complete::{alpha1, self},
    sequence::preceded,
    Parser,
    bytes::complete::tag,
    error::VerboseError
};

#[derive(Clone)]
struct Edge<'a>(&'a str, &'a str);

impl Hash for Edge<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let Edge(from, to) = *self;

        min(from, to).hash(state);
        max(from, to).hash(state);
    }
}

impl Eq for Edge<'_> {}
impl PartialEq for Edge<'_> {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) ||
        (self.0 == other.1 && self.1 == other.0)
    }
}

type WeightedEdge<'a> = (Edge<'a>, u32);

fn parse_edge<'a>(input: &'a str) -> Result<WeightedEdge<'a>, String> {
    let edge = alpha1::<&str, VerboseError<&str>>.and(preceded(tag(" to "), alpha1))
        .map(|(from, to)| Edge(from, to));

    let distance = preceded(tag(" = "), complete::u32);
    run(&mut edge.and(distance), input)
}

struct Graph<'a> {
    adjacency_matrix: HashMap<Edge<'a>, u32>,
    nodes: HashSet<&'a str>
}

impl<'a> Graph<'a> {
    fn distance(&'a self, edge: &'a Edge) -> Option<u32> {
        self.adjacency_matrix.get(edge).map(|x| *x)
    }

    fn route_distance(&'a self, route: impl Iterator<Item = &'a str>) -> Option<u32> {
        route.tuple_windows()
            .map(|(from, to)| self.distance(&Edge(from, to)))
            .sum()
    }
    
    fn from_edges(edges: Vec<WeightedEdge<'a>>) -> Graph<'a> {
        let nodes = edges.clone().into_iter()
            .map(|(Edge(from, _), _)| from)
            .chain(edges.clone().into_iter().map(|(Edge(_, to), _)| to));

        Self {
            adjacency_matrix: HashMap::from_iter(edges.into_iter()),
            nodes: HashSet::from_iter(nodes)
        }
    }
}

pub fn compute_distances(input: &str) -> Result<Vec<u32>, String> {
    let edges = parse_lines(parse_edge, input)?;

    let graph = Graph::from_edges(edges);
    let distances = graph.nodes.clone().into_iter()
        .permutations(graph.nodes.len())
        .filter_map(|route| graph.route_distance(route.into_iter()))
        .collect();

    Ok(distances)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let shortest_distance = *compute_distances(input)?
        .iter().min()
        .ok_or("There is no valid path that visits all nodes")?;

    Ok(Box::from(shortest_distance))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let longest_distance = *compute_distances(input)?
        .iter().max()
        .ok_or("There is no valid path that visits all nodes")?;

    Ok(Box::from(longest_distance))
}