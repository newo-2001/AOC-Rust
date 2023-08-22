use std::collections::{HashMap, HashSet};

use aoc_lib::parsing::{run, parse_lines};
use aoc_runner_api::SolverResult;
use itertools::Itertools;
use nom::{
    combinator::value,
    bytes::complete::tag,
    Parser,
    character::complete::{alpha1, self},
    sequence::{tuple, preceded}
};

#[derive(Eq, PartialEq, Hash, Clone)]
struct Edge<'a>(&'a str, &'a str);

#[derive(Clone)]
struct WeightedEdge<'a> {
    edge: Edge<'a>,
    weight: i32
}

struct Graph<'a> {
    edges: HashMap<Edge<'a>, i32>,
    nodes: HashSet<&'a str>
}

impl Graph<'_> {
    fn from_edges<'a>(edges: Vec<WeightedEdge<'a>>) -> Graph<'a> {
        Graph {
            edges: HashMap::from_iter(edges.clone().into_iter()
                .map(|WeightedEdge { edge, weight }| (edge, weight))),
            
            nodes: HashSet::from_iter(edges.into_iter()
                .flat_map(|WeightedEdge { edge: Edge(from, to), weight: _}| vec![from, to]))
        }
    }

    fn edge_happiness(&self, edge: &Edge) -> i32 {
        *self.edges.get(edge)
            .expect(format!("Missing weight for edge: {} -> {}", edge.0, edge.1).as_str())
    }

    fn arrangement_happiness(&self, arrangement: Vec<&str>) -> i32 {
        arrangement.clone().into_iter().circular_tuple_windows()
            .chain(arrangement.into_iter().rev().circular_tuple_windows())
            .map(|(from, to)| self.edge_happiness(&Edge(from, to)))
            .sum()
    }
    
    fn all_arrangments(&self) -> impl IntoIterator<Item = Vec<&str>> {
        self.nodes.clone().into_iter().permutations(self.nodes.len())
    }

    fn most_happiness(&self) -> Result<i32, String> {
        Vec::from_iter(self.all_arrangments())
            .into_iter()
            .map(|arrangement| self.arrangement_happiness(arrangement))
            .max().ok_or(String::from("No valid arrangement found"))
    }
}

fn parse_edge<'a>(input: &'a str) -> Result<WeightedEdge<'a>, String> {
    #[derive(Clone)]
    enum Sign {
        Positive,
        Negative
    }
    
    let sign = value(Sign::Positive, tag(" gain ")).or(value(Sign::Negative, tag(" lose ")));
    let weight = preceded(tag(" would"), sign).and(complete::u16)
        .map(|(sign, value)| match sign {
            Sign::Positive => value as i32,
            Sign::Negative => -(value as i32)
        });
    
    let neighbour = preceded(tag(" happiness units by sitting next to "), alpha1);
    let mut edge = tuple((alpha1, weight, neighbour))
        .map(|(person, weight, neighbour)| WeightedEdge { edge: Edge(person, neighbour), weight });
    
    run(&mut edge, input)
}

fn build_graph(input: &str) -> Result<Graph, String> {
    let edges = parse_lines(parse_edge, input)?;
    Ok(Graph::from_edges(edges))
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let graph = build_graph(input)?;

    Ok(Box::new(graph.most_happiness()?))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let mut graph = build_graph(input)?;

    let me = graph.nodes.iter()

        .flat_map(|person| vec![
            (Edge("me", person), 0),
            (Edge(person, "me"), 0)
        ]);
    
    graph.edges.extend(me.into_iter());
    graph.nodes.insert("me");

    Ok(Box::new(graph.most_happiness()?))
}