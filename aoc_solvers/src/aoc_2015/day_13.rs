use ahash::{HashMap, HashSet};
use aoc_lib::{parsing::{parse_lines, Runnable, ParseError}, NoSolutionError};
use aoc_runner_api::SolverResult;
use itertools::Itertools;
use nom::{
    combinator::value,
    bytes::complete::tag,
    Parser,
    character::complete::{alpha1, char, u16},
    sequence::{tuple, preceded, delimited}
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
    fn from_edges<'a>(edges: impl IntoIterator<Item=WeightedEdge<'a>> + Clone) -> Graph<'a> {
        Graph {
            edges: edges.clone()
                .into_iter()
                .map(|WeightedEdge { edge, weight }| (edge, weight))
                .collect(),
            
            nodes: edges.into_iter()
                .flat_map(|WeightedEdge { edge: Edge(from, to), .. }| [from, to])
                .collect()
        }
    }

    fn edge_happiness(&self, edge: &Edge) -> i32 {
        *self.edges.get(edge)
            .unwrap_or_else(|| panic!("Missing weight for edge: {} -> {}", edge.0, edge.1))
    }

    fn arrangement_happiness(&self, arrangement: Vec<&str>) -> i32 {
        arrangement.clone().into_iter().circular_tuple_windows()
            .chain(arrangement.into_iter().rev().circular_tuple_windows())
            .map(|(from, to)| self.edge_happiness(&Edge(from, to)))
            .sum()
    }
    
    fn all_arrangments(&self) -> impl Iterator<Item = Vec<&str>> {
        self.nodes.clone().into_iter().permutations(self.nodes.len())
    }

    fn most_happiness(&self) -> Result<i32, NoSolutionError> {
        self.all_arrangments()
            .map(|arrangement| self.arrangement_happiness(arrangement))
            .max().ok_or(NoSolutionError)
    }
}

fn parse_edge(input: &str) -> Result<WeightedEdge, ParseError> {
    #[derive(Clone)]
    enum Sign {
        Positive,
        Negative
    }
    
    let sign = value(Sign::Positive, tag(" gain ")).or(value(Sign::Negative, tag(" lose ")));
    let weight = preceded(tag(" would"), sign).and(u16)
        .map(|(sign, value)| match sign {
            Sign::Positive => i32::from(value),
            Sign::Negative => -(i32::from(value))
        });
    
    let neighbour = delimited(tag(" happiness units by sitting next to "), alpha1, char('.'));
    tuple((alpha1, weight, neighbour))
        .map(|(person, weight, neighbour)| WeightedEdge { edge: Edge(person, neighbour), weight })
        .run(input)
}

fn build_graph(input: &str) -> Result<Graph, ParseError> {
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
    
    graph.edges.extend(me);
    graph.nodes.insert("me");

    Ok(Box::new(graph.most_happiness()?))
}