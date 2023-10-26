use std::{fs, time::{Duration, Instant}};

use aoc_runner_api::Puzzle;
use thiserror::Error;

pub enum RunnerAction {
    Run(Vec<Puzzle>),
    Verify(Vec<Puzzle>)
}

#[derive(Debug, Error)]
pub enum ResolutionError {
    #[error("Failed to resolve solver")]
    Solver,
    #[error("Failed to resolve input file: {0}")]
    InputFile(String),
    #[error("Failed to resolve solution file: {0}")]
    SolutionFile(String),
    #[error("Failed to locate solution entry")]
    SolutionEntry
}

#[derive(Debug, Error)]
pub enum RunnerError {
    #[error("Solver produced an incorrect answer, expected: `{expected}` got `{actual}`")]
    IncorrectAnswer {
        expected: String,
        actual: String
    },
    #[error("An error occurred whilst executing the puzzle:\n\t{0}")]
    ExecutionError(String)
}

fn get_input_for_puzzle(puzzle: Puzzle) -> Result<String, ResolutionError> {
    let path = format!("inputs/{}/day_{:02}.txt", puzzle.year, puzzle.day);
    fs::read_to_string(&path)
        .map_err(|_| ResolutionError::InputFile(path))
}

fn get_solution_for_puzzle(puzzle: Puzzle) -> Result<String, ResolutionError> {
    let path = format!("solutions/{}/day_{:02}.txt", puzzle.year, puzzle.day);
    let content = fs::read_to_string(&path)
        .map_err(|_| ResolutionError::SolutionFile(path))?;

    let solution = *content.chars()
        .as_str()
        .split(';')
        .collect::<Vec<_>>()
        .get(puzzle.part.number() as usize - 1)
        .ok_or(ResolutionError::SolutionEntry)?;

    if solution.is_empty() { Err(ResolutionError::SolutionEntry) }
    else { Ok(solution.to_owned()) }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{error}")]
    RunnerError {
        duration: Duration,
        error: RunnerError
    },
    #[error(transparent)]
    ResolutionError(#[from] ResolutionError)
}

pub struct RunStats {
    pub result: String,
    pub duration: Duration
}

pub fn run_puzzle(puzzle: Puzzle) -> Result<RunStats, Error> {
    let solver = aoc_solvers::get_solver(puzzle)
        .ok_or(ResolutionError::Solver)?;

    let input = get_input_for_puzzle(puzzle)?;

    let start_time = Instant::now();
    let result = solver(&input)
        .map_err(|err| Error::RunnerError {
            error: RunnerError::ExecutionError(err.to_string()),
            duration: start_time - Instant::now()
        })?;

    let duration = start_time - Instant::now();
    Ok(RunStats { result: result.to_string(), duration })
}

pub fn verify_puzzle(puzzle: Puzzle) -> Result<RunStats, Error> {
    let expected = get_solution_for_puzzle(puzzle)?
        .replace("\r\n", "")
        .replace('\n', "");

    let stats = run_puzzle(puzzle)?;
    let actual = stats.result.to_string()
        .replace("\r\n", "")
        .replace('\n', "");

    if expected == actual { Ok(stats) }
    else {
        Err(Error::RunnerError {
            error: RunnerError::IncorrectAnswer { expected, actual },
            duration: stats.duration
        })
    }
}