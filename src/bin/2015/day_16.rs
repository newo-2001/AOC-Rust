use std::{error::Error, fs, collections::HashMap};

use itertools::Itertools;
use nom::{
    sequence::{terminated, delimited},
    bytes::complete::tag,
    character::complete::{self, alpha1},
    combinator::opt, multi::many0, Parser, error::VerboseError
};

const DIMENSIONS: usize = 10;
const COMPOUNDS: [&'static str; DIMENSIONS] = [
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

fn parse_sue(input: &str) -> Result<Sue, String> {
    let number = delimited(tag("Sue "), complete::u16, complete::char(':'));
    let compound = delimited(complete::char(' '), alpha1, tag(": "))
        .and(terminated(complete::u8, opt(complete::char(','))));

    let compounds = many0(compound)
        .map(|x| x.into_iter().collect::<HashMap<&str, u8>>())
        .map(|compounds| COMPOUNDS.iter().map(|compound| compounds.get(compound).map(|x| *x)).collect_vec().try_into());
    
    let mut sue = number.and(compounds).map(|(number, compounds)| Sue { number, compounds: compounds.unwrap() });
    Ok(sue.parse(input).map_err(|err: nom::Err<VerboseError<&str>>| err.to_string())?.1)
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
            (i @ 1, compound) | (i @ 7, compound) => compound > fact[i],
            (i @ 3, compound) | (i @ 6, compound) => compound < fact[i],
            (i, compound) => compound == fact[i]
        })
}

fn solve<'a>(fact: Fact, system: Vec<Sue>, predicate: impl Fn(Fact, &Sue) -> bool) -> Result<Sue, String> {
    println!("Starting with {} possible solutions", system.len());

    let system = system.into_iter()
        .filter(|solution| predicate(fact, solution))
        .collect_vec();

    if let [solution] = system.as_slice() {
        Ok(*solution)
    } else { Err(match system.len() {
        0 => String::from("No solutions found"),
        n => format!("{} valid solutions remained", n)
    })}
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("inputs/2015/day_16.txt")?;

    let sues = content.lines().map(parse_sue)
        .collect::<Result<Vec<Sue>, String>>()?;

    const FACT: Fact = [3, 7, 2, 3, 0, 0, 5, 3, 2, 1];

    let solution = solve(FACT, sues.clone(), is_valid_solution)?;
    println!("The correct sue is sue #{}", solution.number);

    let solution = solve(FACT, sues, is_valid_solution_range)?;
    println!("The correct sue was sue #{} after all", solution.number);

    Ok(())
}