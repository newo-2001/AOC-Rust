use aoc_lib::parsing::parse_lines;
use crate::SolverResult;
use itertools::Itertools;

fn different_sums(total: u32, sizes: &[u32]) -> impl Iterator<Item = Vec<&u32>> {
    sizes.iter()
        .powerset()
        .filter(move |containers| containers.iter().copied().sum::<u32>() == total)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let containers = parse_lines(str::parse, input)?;
    let combinations = different_sums(150, &containers).count();

    Ok(Box::new(combinations))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let containers = parse_lines(str::parse, input)?;
    let combinations: Vec<Vec<&u32>> = different_sums(150, &containers).collect();

    let sized_combinations = combinations.iter()
        .chunk_by(|containers| containers.len());

    let (_, combinations) = sized_combinations.into_iter()
        .sorted_unstable_by_key(|(amount, _)| *amount)
        .next().expect("No valid combinations found");

    Ok(Box::new(combinations.count()))
}