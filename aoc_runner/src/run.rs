use std::fs;

use aoc_runner_api::{Puzzle, SolverResult};

pub enum RunnerAction {
    Run(Vec<Puzzle>),
    Verify(Vec<Puzzle>)
}

fn get_input_for_puzzle(puzzle: &Puzzle) -> Result<String, String> {
    let path = format!("inputs/{}/day_{:02}.txt", puzzle.year, puzzle.day);
    fs::read_to_string(&path)
        .map_err(|_| format!("Failed to resolve input file: {}", path))
}

fn get_solution_for_puzzle(puzzle: &Puzzle) -> Result<String, String> {
    let path = format!("solutions/{}/day_{:02}.txt", puzzle.year, puzzle.day);
    let content = fs::read_to_string(&path)
        .map_err(|_| format!("Failed to resolve solution file: {}", path))?;

    content.lines()
        .into_iter()
        .collect::<Vec<&str>>()
        .get((puzzle.part - 1) as usize)
        .map(|&s| s.to_owned())
        .ok_or(format!("Failed to resolve solution entry for {}-{}-{}", puzzle.year, puzzle.day, puzzle.part))
}

pub fn run_puzzle(puzzle: &Puzzle) -> SolverResult {
    let solver = aoc_solvers::get_solver(puzzle)
        .ok_or(format!("Failed to resolve solver for {}-{}-{}", puzzle.year, puzzle.day, puzzle.part))?;

    let input = get_input_for_puzzle(puzzle)?;

    solver(&input)
}

pub fn verify_puzzle(puzzle: &Puzzle) -> SolverResult {
    let expected = get_solution_for_puzzle(puzzle)?;
    let actual = run_puzzle(puzzle)?;

    if expected == actual.to_string() { Ok(actual) }
    else { Err(format!("expected: '{}', got: '{}'", expected, actual).into()) }
}