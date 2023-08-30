use std::{fs, fmt::Display};

use aoc_runner_api::Puzzle;
use thiserror::Error;

pub enum RunnerAction {
    Run(Vec<Puzzle>),
    Verify(Vec<Puzzle>)
}

#[derive(Debug, Error)]
pub enum RunPuzzleError {
    #[error("Failed to resolve solver")]
    MissingSolver,
    #[error("Failed to resolve input file: {0}")]
    MissingInputFile(String),
    #[error("Failed to resolve solution file: {0}")]
    MissingSolutionFile(String),
    #[error("Failed to locate solution entry for part {0}")]
    MissingSolutionEntry(u8),
    #[error("Solver produced wrong answer, expected: `{expected}` got `{actual}`")]
    WrongAnswer {
        expected: String,
        actual: String
    },
    #[error("An error occurred whilst executing the puzzle:\n\t{0}")]
    ExecutionError(String)
}

fn get_input_for_puzzle(puzzle: &Puzzle) -> Result<String, RunPuzzleError> {
    let path = format!("inputs/{}/day_{:02}.txt", puzzle.year, puzzle.day);
    fs::read_to_string(&path)
        .map_err(|_| RunPuzzleError::MissingInputFile(path))
}

fn get_solution_for_puzzle(puzzle: &Puzzle) -> Result<String, RunPuzzleError> {
    let path = format!("solutions/{}/day_{:02}.txt", puzzle.year, puzzle.day);
    let content = fs::read_to_string(&path)
        .map_err(|_| RunPuzzleError::MissingSolutionFile(path))?;

    let solution = *content.chars()
        .as_str()
        .split(';')
        .collect::<Vec<_>>()
        .get(puzzle.part as usize - 1)
        .ok_or(RunPuzzleError::MissingSolutionEntry(puzzle.part))?;

    Ok(solution.to_owned())
}

pub fn run_puzzle(puzzle: &Puzzle) -> Result<Box<dyn Display>, RunPuzzleError> {
    let solver = aoc_solvers::get_solver(puzzle)
        .ok_or(RunPuzzleError::MissingSolver)?;

    let input = get_input_for_puzzle(puzzle)?;
    solver(&input).map_err(|err| RunPuzzleError::ExecutionError(err.to_string()))
}

pub fn verify_puzzle(puzzle: &Puzzle) -> Result<Box<dyn Display>, RunPuzzleError> {
    let expected = get_solution_for_puzzle(puzzle)?
        .replace("\r\n", "")
        .replace('\n', "");

    let result = run_puzzle(puzzle)?;
    let actual = result.to_string()
        .replace("\r\n", "")
        .replace('\n', "");

    if expected == actual { Ok(result) }
    else { Err(RunPuzzleError::WrongAnswer { expected, actual }) }
}