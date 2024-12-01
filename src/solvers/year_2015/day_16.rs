use ahash::HashMap;
use aoc_lib::parsing::{parse_lines, TextParser, ParseError};
use yuki::errors::NoSolution;
use crate::SolverResult;
use itertools::Itertools;
use nom::{
    sequence::{terminated, delimited},
    bytes::complete::tag,
    character::complete::{alpha1, char, u8, u16},
    combinator::opt, multi::many0, Parser
};

const DIMENSIONS: usize = 10;
const COMPOUNDS: [&str; DIMENSIONS] = [
    "children", "cats", "samoyeds", "pomeranians", "akitas",
    "vizslas", "goldfish", "trees", "cars", "perfumes"
];

type Compounds = [Option<u8>; DIMENSIONS];
type Fact = [u8; DIMENSIONS];

#[derive(Copy, Clone)]
struct Sue {
    number: u16,
    compounds: Compounds
}

impl Sue {
    fn parse(input: &str) -> Result<Self, ParseError> {
        let number = delimited(tag("Sue "), u16, char(':'));
        let compound = delimited(char(' '), alpha1, tag(": "))
            .and(terminated(u8, opt(char(','))));

        let compounds = many0(compound)
            .map(|x| x.into_iter().collect::<HashMap<&str, u8>>())
            .map(|compounds| COMPOUNDS.iter().map(|compound| compounds.get(compound).copied()).collect_vec().try_into());
        
        number.and(compounds)
            .map(|(number, compounds)| Self { number, compounds: compounds.unwrap() })
            .run(input)
    }
}

fn is_valid_solution(fact: Fact, solution: &Sue) -> bool {
    solution.compounds.iter().enumerate()
        .filter_map(|(i, compound)| compound.map(|x| (i, x)))
        .all(|(index, compound)| compound == fact[index])
}

fn is_valid_solution_range(fact: Fact, solution: &Sue) -> bool {
    solution.compounds.iter().enumerate()
        .filter_map(|(i, compound)| compound.map(|x| (i, x)))
        .all(|(index, compound)| match (index, compound) {
            (i @ (1 | 7), compound) => compound > fact[i],
            (i @ (3 | 6), compound) => compound < fact[i],
            (i, compound) => compound == fact[i]
        })
}

fn solve(fact: Fact, system: Vec<Sue>, predicate: impl Fn(Fact, &Sue) -> bool) -> Result<Sue, NoSolution> {
    let system = system.into_iter()
        .filter(|solution| predicate(fact, solution))
        .collect_vec();

    if let [solution] = system.as_slice() {
        Ok(*solution)
    } else { Err(NoSolution) }
}

const FACT: Fact = [3, 7, 2, 3, 0, 0, 5, 3, 2, 1];

pub fn solve_part_1(input: &str) -> SolverResult {
    let sues = parse_lines(Sue::parse, input)?;
    let solution = solve(FACT, sues, is_valid_solution)?;

    Ok(Box::new(solution.number))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let sues = parse_lines(Sue::parse, input)?;
    let solution = solve(FACT, sues, is_valid_solution_range)?;

    Ok(Box::new(solution.number))
}