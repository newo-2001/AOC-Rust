use std::fs;

use aoc_runner_api::{Puzzle, SolverResult};

pub enum RunnerAction {
    Run(Vec<Puzzle>)
}

pub fn get_input_for_puzzle(puzzle: &Puzzle) -> Result<String, String> {
    let path = format!("inputs/{}/day_{:02}.txt", puzzle.year, puzzle.day);
    fs::read_to_string(&path)
        .map_err(|_| format!("Failed to resolve input file: {}", path))
}

pub fn run_puzzle(puzzle: &Puzzle) -> SolverResult {
    let input = get_input_for_puzzle(puzzle)?;

    let solver = aoc_solvers::get_solver(puzzle)
        .ok_or(format!("Failed to resolve solver for {}-{}-{}", puzzle.year, puzzle.day, puzzle.part))?;

    solver(&input)
}