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

    let solution = *content.chars()
        .as_str()
        .split(';')
        .collect::<Vec<_>>()
        .get(puzzle.part as usize - 1)
        .ok_or(format!("Failed to resolve solution entry for {}-{}-{}", puzzle.year, puzzle.day, puzzle.part))?;

    Ok(solution.to_owned())
}

pub fn run_puzzle(puzzle: &Puzzle) -> SolverResult<'_> {
    let solver = aoc_solvers::get_solver(puzzle)
        .ok_or(format!("Failed to resolve solver for {}-{}-{}", puzzle.year, puzzle.day, puzzle.part))?;

    let input = get_input_for_puzzle(puzzle)?;

    let result = solver(&input).map_err(|err| err.to_string())?;
    Ok(result)
}

pub fn verify_puzzle(puzzle: &Puzzle) -> SolverResult {
    let expected = get_solution_for_puzzle(puzzle)?
        .replace("\r\n", "")
        .replace('\n', "");

    let result = run_puzzle(puzzle)?;
    let actual = result.to_string()
        .replace("\r\n", "")
        .replace('\n', "");

    if expected == actual { Ok(result) }
    else { Err(format!("expected: '{}', got: '{}'", expected, actual).into()) }
}