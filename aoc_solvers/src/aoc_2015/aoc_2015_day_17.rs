use aoc_lib::parsing::parse_lines;
use aoc_runner_api::SolverResult;
use itertools::Itertools;

fn different_sums(total: u32, sizes: &[u32]) -> impl Iterator<Item = Vec<&u32>> {
    sizes.iter()
        .powerset()
        .filter(move |containers| containers.iter().copied().sum::<u32>() == total)
}

pub fn solve_part_1(input: &str) -> SolverResult {
    let containers = parse_lines(str::parse, input)?;
    let combinations: Vec<Vec<&u32>> = different_sums(150, &containers).collect();

    Ok(Box::new(combinations.len()))
}

pub fn solve_part_2(input: &str) -> SolverResult {
    let containers = parse_lines(str::parse, input)?;
    let combinations: Vec<Vec<&u32>> = different_sums(150, &containers).collect();

    let sized_combinations = combinations.iter()
        .group_by(|containers| containers.len());

    let (_, combinations) = sized_combinations.into_iter()
        .sorted_unstable_by_key(|(amount, _)| *amount)
        .next().expect("No valid combinations found");

    Ok(Box::new(combinations.count()))
}