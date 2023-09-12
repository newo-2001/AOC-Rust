use std::{fs, fmt::Display, time::{Duration, Instant}};

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

pub struct RunStats {
    pub result: Box<dyn Display + Send + Sync>,
    pub duration: Duration
}

fn get_input_for_puzzle(puzzle: Puzzle) -> Result<String, RunPuzzleError> {
    let path = format!("inputs/{}/day_{:02}.txt", puzzle.year, puzzle.day);
    fs::read_to_string(&path)
        .map_err(|_| RunPuzzleError::MissingInputFile(path))
}

fn get_solution_for_puzzle(puzzle: Puzzle) -> Result<String, RunPuzzleError> {
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

pub fn run_puzzle(puzzle: Puzzle) -> Result<RunStats, RunPuzzleError> {
    let solver = aoc_solvers::get_solver(puzzle)
        .ok_or(RunPuzzleError::MissingSolver)?;

    let start_time = Instant::now();

    let input = get_input_for_puzzle(puzzle)?;
    let result = solver(&input)
        .map_err(|err| RunPuzzleError::ExecutionError(err.to_string()))?;

    let end_time = Instant::now();
    Ok(RunStats {
        duration: end_time - start_time,
        result
    })
}

pub fn verify_puzzle(puzzle: Puzzle) -> Result<RunStats, RunPuzzleError> {
    let expected = get_solution_for_puzzle(puzzle)?
        .replace("\r\n", "")
        .replace('\n', "");

    let stats = run_puzzle(puzzle)?;
    let actual = stats.result.to_string()
        .replace("\r\n", "")
        .replace('\n', "");

    if expected == actual { Ok(stats) }
    else { Err(RunPuzzleError::WrongAnswer { expected, actual }) }
}