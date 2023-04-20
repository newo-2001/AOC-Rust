use std::{
    error::Error,
    fs,
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    cmp::{max, min}
};

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

fn parse_edge<'a>(input: &'a str) -> Result<WeightedEdge<'a>, Box<dyn Error + 'a>> {
    let edge = alpha1::<&str, VerboseError<&str>>.and(preceded(tag(" to "), alpha1))
        .map(|(from, to)| Edge(from, to));

    let distance = preceded(tag(" = "), complete::u32);
    Ok(edge.and(distance).parse(input)?.1)
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

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("inputs/2015/day_9.txt")?;

    let edges = content
        .lines().map(parse_edge)
        .collect::<Result<Vec<WeightedEdge>, Box<dyn Error>>>()
        .map_err(|err| err.to_string())?;

    let graph = Graph::from_edges(edges);
    let distances: Vec<u32> = graph.nodes.clone().into_iter()
        .permutations(graph.nodes.len())
        .filter_map(|route| graph.route_distance(route.into_iter()))
        .collect();

    let shortest_distance = distances.iter().min().ok_or("There is no valid path that visits all nodes")?;
    println!("The shortest path is {}", shortest_distance);
    
    let longest_distance = distances.iter().max().ok_or("There is no valid path that visits all nodes")?;
    println!("The longest path is {}", longest_distance);

    Ok(())
}